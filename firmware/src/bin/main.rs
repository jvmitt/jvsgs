#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

#![feature(proc_macro_hygiene)]

// Temporary
#[allow(unused_mut)]

use jvsgs as _; // global logger + panicking-behavior + memory layout
use stm32f4xx_hal::{
        adc::{
            config::{AdcConfig, SampleTime},
            Adc,
        },
       gpio::*,
       i2c::I2c,
       rcc::*,
       pac::{self, syscfg, I2C1, TIM1, TIM2, TIM3, TIM4, TIM5, ADC1},
       serial::Tx,
       prelude::*,
       timer::{self, CounterMs, Event, Timer2},
};

use rtic_monotonics::systick::prelude::*;

use defmt_rtt as _;
use panic_probe as _;


#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [EXTI0, EXTI1, EXTI2],
    peripherals = true
)]
mod app {

    use stm32f4xx_hal::pac::ADC1;
    use stm32f4xx_hal::rcc::Config;
    use stm32f4xx_hal::gpio;
    use stm32f4xx_hal::gpio::*;
    use super::*;

    use env::env::*;
    mod env;

    use models::models::*;
    mod models;

    use io::io::*;
    mod io;

    use ppm::ppm::*;
    mod ppm;

    use usb_host::usb_host::*;
    mod usb_host;

    systick_monotonic!(Mono, 1000);

    #[shared]
    struct Shared {
        ppm: PpmState,
        delay_val: u32,
        adc_module: Adc<ADC1>,
    }

    #[local]
    struct Local {

        delay: timer::DelayMs<TIM1>,
        builtin_led: PC13<Output>,
        buzzer: PB9<Output>,

        control_output: ControlOutput,

        control_panel: ControlPanel<
            stm32f4xx_hal::gpio::PA1<stm32f4xx_hal::gpio::Analog>,
            stm32f4xx_hal::gpio::PA7<stm32f4xx_hal::gpio::Analog>,
            stm32f4xx_hal::gpio::PA5<stm32f4xx_hal::gpio::Analog>,
            stm32f4xx_hal::gpio::PA4<stm32f4xx_hal::gpio::Analog>,
            stm32f4xx_hal::gpio::PB1<stm32f4xx_hal::gpio::Analog>,
            stm32f4xx_hal::gpio::PB0<stm32f4xx_hal::gpio::Analog>,
        >,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("Initializing JVSGS...");

        let dp = cx.device; // PAC Device Peripherals
        let cp = cx.core; // Cortex-M Core Peripherals

        Mono::start(cp.SYST, 16_000_000);

        // STM32 Base
        dp.RCC.apb1enr().modify(|_, w| w.tim2en().set_bit()); // For PPM timing before RCC gets taken
        let ppre1 = ((dp.RCC.cfgr().read().bits() >> 10) & 0b111) as u8; // pass it to apb_2x bool
        let mut rcc = dp.RCC.freeze(
            Config::hsi()
                .sysclk(64.MHz())
                .use_hse(25.MHz())
        );
        let clocks = rcc.clocks;

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);
        let gpioc = dp.GPIOC.split(&mut rcc);

        let delay = dp.TIM1.delay_ms(&mut rcc);


        // Environment
        // Future logging messages on data retrieving
        //defmt::info!("Retrieving SD card files...");
        //defmt::warn!("Could not find a SD card.");
        //defmt::info!("Retrieving EEPROM backup memory...");
        //defmt::warn!("Done. Please consider using a SD card as persistant memory instead of the EEPROM backup.");

        // Analog Input
        let mut adc = Adc::new(dp.ADC1, true, AdcConfig::default(), &mut rcc);
        adc.calibrate();


        /*
        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into(),
            pin_dp: gpioa.pa12.into(),
            hclk: rcc.clocks.hclk(),
        };
         */

        let mut builtin_led = gpioc.pc13.into_push_pull_output();

        let mut ppm: PpmState = init_ppm(
            8, // Number of PPM channels
            7, // Mode channel, init as 1000
            2, // Throttle channel, init as 1000
            1500, // Every channel but throttle and mode default value
            22500, // Frame length
            300, // Pulse length
            true, // State tha is considered ON on your system
            gpiob.pb12.into_push_pull_output().erase(), // PPM signal output pin
            gpiob.pb13.into_pull_up_input().erase(), // Switch pin
            100, // Switch step
            unsafe { &*pac::TIM2::ptr() }, // TIM2 register block
            true, // apb_2x
            clocks.pclk1().raw() as u32, // Pclk hz
            2_000_000u32, // Tick hz
        ); defmt::info!("PPM initialized with {} channels.", ppm.config.channel_number);

        // Buttons
        let mut navigation_btns = gpioa.pa15.into_pull_up_input();

        // Min and Max value as of now are placeholders, read it from a calibration function or file
        let control_panel = ControlPanel {
            arm_switch: gpiob.pb3.into_pull_up_input().erase(),
            abort_btn: gpiob.pb4.into_pull_up_input().erase(),
            left_potentiometer: Potentiometer {
                pin: gpioa.pa1.into_analog(),
                min: 0,
                max: 4096,
            },
            right_potentiometer: Potentiometer {
                pin: gpioa.pa7.into_analog(),
                min: 0,
                max: 4096,
            },
            left_joystick: Joystick {
                x: Potentiometer {
                    pin: gpioa.pa5.into_analog(),
                    min: 0,
                    max: 4096,
                },
                y: Potentiometer {
                    pin: gpioa.pa4.into_analog(),
                    min: 0,
                    max: 4096,
                },
            },
            right_joystick: Joystick {
                x: Potentiometer {
                    pin: gpiob.pb1.into_analog(),
                    min: 0,
                    max: 4096,
                },
                y: Potentiometer {
                    pin: gpiob.pb0.into_analog(),
                    min: 0,
                    max: 4096,
                },
            },
        };

        let mut control_output = ControlOutput {
            roll: 1500,
            pitch: 1500,
            throttle: 1000,
            yaw: 1500,
            mode: 1000,
        };

        // User Cues
        let mut buzzer = gpiob.pb9.into_push_pull_output();

        // Successfull setup confirmation 
        defmt::info!("Initialization complete.");

        builtin_led.set_high();

        operational::spawn().ok();

        (
            Shared {
                ppm,
                delay_val: 1000_u32,
                adc_module: adc,
            },
            Local {
                delay,
                builtin_led,
                buzzer,
                control_panel,
                control_output,
            },
        )
    }

    #[idle(shared = [delay_val], local = [delay, builtin_led, buzzer])]
    fn idle(mut cx: idle::Context) -> ! {
        defmt::info!("Engaging idle mode.");

        let led = cx.local.builtin_led;
        let delay = cx.local.delay;

        loop {

            led.set_high();

            delay.delay_ms(cx.shared.delay_val.lock(|del| *del));

            led.set_low();

            delay.delay_ms(cx.shared.delay_val.lock(|del| *del));

        }
    }

    #[task(priority = 1, shared = [ppm, adc_module], local = [control_panel, control_output])]
    async fn operational(mut cx: operational::Context) {
        defmt::info!("Engaging operational mode.");

        loop {

            cx.shared.ppm.lock(|ppm| {
                filter_ppm(ppm);
            });

            *cx.local.control_output = cx.shared.adc_module.lock(|adc_module| ControlOutput {
                roll: format_ppm(adc_module.convert(&cx.local.control_panel.right_joystick.x.pin, SampleTime::Cycles_480), cx.local.control_panel.right_joystick.x.min, cx.local.control_panel.right_joystick.x.max),
                pitch: format_ppm(adc_module.convert(&cx.local.control_panel.right_joystick.y.pin, SampleTime::Cycles_480), cx.local.control_panel.right_joystick.x.min, cx.local.control_panel.right_joystick.x.max),
                throttle: format_ppm(adc_module.convert(&cx.local.control_panel.left_potentiometer.pin, SampleTime::Cycles_480), cx.local.control_panel.right_joystick.x.min, cx.local.control_panel.right_joystick.x.max),
                yaw: format_ppm(adc_module.convert(&cx.local.control_panel.left_joystick.x.pin, SampleTime::Cycles_480), cx.local.control_panel.right_joystick.x.min, cx.local.control_panel.right_joystick.x.max),
                mode: cx.local.control_output.mode,
            });

            /*
            defmt::info!(
                "{}, {}, {}, {}",
                cx.local.control_output.roll,
                cx.local.control_output.pitch,
                cx.local.control_output.throttle,
                cx.local.control_output.yaw
            );
             */


            cx.shared.ppm.lock(|ppm| {
                ppm.ppm[0] = cx.local.control_output.roll;
                ppm.ppm[1] = cx.local.control_output.pitch;
                ppm.ppm[2] = cx.local.control_output.throttle;
                ppm.ppm[3] = cx.local.control_output.yaw;
                ppm.ppm[4] = 1500;
                ppm.ppm[5] = 1500;
                ppm.ppm[6] = 1500;
                ppm.ppm[7] = cx.local.control_output.mode;
            });

            Mono::delay(10.millis()).await;

        }

    }

    #[task(binds = TIM2, priority = 3, shared = [ppm])]
    fn tim2(mut cx: tim2::Context) {
        cx.shared.ppm.lock(|ppm| {
            tim2_ppm(ppm);
        });
    }

}
