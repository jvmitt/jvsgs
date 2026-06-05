#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f4xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 2u8; use
    stm32f4xx_hal :: pac :: ADC1; use stm32f4xx_hal :: rcc :: Config; use
    stm32f4xx_hal :: gpio; use stm32f4xx_hal :: gpio :: * ; use super :: * ;
    use env :: env :: * ; use models :: models :: * ; use io :: io :: * ; use
    ppm :: ppm :: * ; use usb_host :: usb_host :: * ; mod env; mod models; mod
    io; mod ppm; mod usb_host; systick_monotonic! (Mono, 1000);
    #[doc = r" User code end"] #[doc = r"Shared resources"] struct Shared
    { ppm : PpmState, delay_val : u32, adc_module : Adc < ADC1 > , }
    #[doc = r"Local resources"] struct Local
    {
        delay : timer :: DelayMs < TIM1 > , builtin_led : PC13 < Output > ,
        buzzer : PB9 < Output > , control_output : ControlOutput,
        control_panel : ControlPanel < stm32f4xx_hal :: gpio :: PA1 <
        stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA7 <
        stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA5 <
        stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA4 <
        stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PB1 <
        stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PB0 <
        stm32f4xx_hal :: gpio :: Analog > , > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > ,
        #[doc = r" The space used to allocate async executors in bytes."] pub
        executors_size : usize, #[doc = r" Core peripherals"] pub core : rtic
        :: export :: Peripherals, #[doc = r" Device peripherals (PAC)"] pub
        device : stm32f4xx_hal :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a > ,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals, executors_size : usize) ->
        Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, core :
                core, device : stm32f4xx_hal :: pac :: Peripherals :: steal(),
                cs : rtic :: export :: CriticalSection :: new(),
                executors_size,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_init_Context as
        Context;
    } #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local)
    {
        defmt :: info! ("Initializing JVSGS..."); let dp = cx.device; let cp =
        cx.core; Mono :: start(cp.SYST, 16_000_000);
        dp.RCC.apb1enr().modify(| _, w | w.tim2en().set_bit()); let ppre1 =
        ((dp.RCC.cfgr().read().bits() >> 10) & 0b111) as u8; let mut rcc =
        dp.RCC.freeze(Config :: hsi().sysclk(64.MHz()).use_hse(25.MHz())); let
        clocks = rcc.clocks; let gpioa = dp.GPIOA.split(& mut rcc); let gpiob
        = dp.GPIOB.split(& mut rcc); let gpioc = dp.GPIOC.split(& mut rcc);
        let delay = dp.TIM1.delay_ms(& mut rcc); defmt :: info!
        ("Retrieving SD card files..."); defmt :: warn!
        ("Could not find a SD card."); defmt :: info!
        ("Retrieving EEPROM backup persistant memory..."); defmt :: warn!
        ("Could not find a valid memory template on EEPROM."); defmt :: info!
        ("Initializing EEPROM memory template..."); defmt :: info!
        ("EEPROM memory template done."); defmt :: warn!
        ("Please consider using a SD card as persistant memory instead of the EEPROM backup.");
        let mut adc = Adc ::
        new(dp.ADC1, true, AdcConfig :: default(), & mut rcc);
        adc.calibrate(); let mut builtin_led =
        gpioc.pc13.into_push_pull_output(); let mut ppm : PpmState =
        init_ppm(8, 7, 2, 1500, 22500, 300, true,
        gpiob.pb12.into_push_pull_output().erase(),
        gpiob.pb13.into_pull_up_input().erase(), 100, unsafe
        { & * pac :: TIM2 :: ptr() }, true, clocks.pclk1().raw() as u32,
        2_000_000u32,); defmt :: info!
        ("PPM initialized with {} channels.", ppm.config.channel_number); let
        mut navigation_btns = gpioa.pa15.into_pull_up_input(); let
        control_panel = ControlPanel
        {
            arm_switch : gpiob.pb3.into_pull_up_input().erase(), abort_btn :
            gpiob.pb4.into_pull_up_input().erase(), left_potentiometer :
            Potentiometer
            { pin : gpioa.pa1.into_analog(), min : 0, max : 4096, },
            right_potentiometer : Potentiometer
            { pin : gpioa.pa7.into_analog(), min : 0, max : 4096, },
            left_joystick : Joystick
            {
                x : Potentiometer
                { pin : gpioa.pa5.into_analog(), min : 0, max : 4096, }, y :
                Potentiometer
                { pin : gpioa.pa4.into_analog(), min : 0, max : 4096, },
            }, right_joystick : Joystick
            {
                x : Potentiometer
                { pin : gpiob.pb1.into_analog(), min : 0, max : 4096, }, y :
                Potentiometer
                { pin : gpiob.pb0.into_analog(), min : 0, max : 4096, },
            },
        }; let mut control_output = ControlOutput
        {
            roll : 1500, pitch : 1500, throttle : 1000, yaw : 1500, mode :
            1000,
        }; let mut buzzer = gpiob.pb9.into_push_pull_output(); defmt :: info!
        ("Initialization complete."); builtin_led.set_high(); operational ::
        spawn().ok();
        (Shared { ppm, delay_val : 1000_u32, adc_module : adc, }, Local
        { delay, builtin_led, buzzer, control_panel, control_output, },)
    } impl < 'a > __rtic_internal_idleSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idleSharedResources
            {
                delay_val : shared_resources ::
                delay_val_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_idleLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idleLocalResources
            {
                delay : & mut *
                (& mut *
                __rtic_internal_local_resource_delay.get_mut()).as_mut_ptr(),
                builtin_led : & mut *
                (& mut *
                __rtic_internal_local_resource_builtin_led.get_mut()).as_mut_ptr(),
                buzzer : & mut *
                (& mut *
                __rtic_internal_local_resource_buzzer.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `idle` has access to"] pub struct
    __rtic_internal_idleSharedResources < 'a >
    {
        #[allow(missing_docs)] pub delay_val : shared_resources ::
        delay_val_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `idle` has access to"] pub struct
    __rtic_internal_idleLocalResources < 'a >
    {
        #[allow(missing_docs)] pub delay : & 'static mut timer :: DelayMs <
        TIM1 > , #[allow(missing_docs)] pub builtin_led : & 'static mut PC13 <
        Output > , #[allow(missing_docs)] pub buzzer : & 'static mut PB9 <
        Output > , #[doc(hidden)] pub __rtic_internal_marker : :: core ::
        marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : idle :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        idle :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idle_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                idle :: LocalResources :: new(), shared : idle ::
                SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleLocalResources as
        LocalResources; #[doc(inline)] pub use super ::
        __rtic_internal_idleSharedResources as SharedResources; #[doc(inline)]
        pub use super :: __rtic_internal_idle_Context as Context;
    } #[allow(non_snake_case)] fn idle(mut cx : idle :: Context) -> !
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; defmt ::
        info! ("Engaging idle mode."); let led = cx.local.builtin_led; let
        delay = cx.local.delay; loop
        {
            led.set_high();
            delay.delay_ms(cx.shared.delay_val.lock(| del | * del));
            led.set_low();
            delay.delay_ms(cx.shared.delay_val.lock(| del | * del));
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn TIM2()
    {
        const PRIORITY : u8 = 3u8; rtic :: export ::
        run(PRIORITY, || { tim2(tim2 :: Context :: new()) });
    } impl < 'a > __rtic_internal_tim2SharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_tim2SharedResources
            {
                ppm : shared_resources :: ppm_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `tim2` has access to"] pub struct
    __rtic_internal_tim2SharedResources < 'a >
    {
        #[allow(missing_docs)] pub ppm : shared_resources ::
        ppm_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_tim2_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Shared Resources this task has access to"] pub
        shared : tim2 :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_tim2_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_tim2_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                tim2 :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod tim2
    {
        #[doc(inline)] pub use super :: __rtic_internal_tim2SharedResources as
        SharedResources; #[doc(inline)] pub use super ::
        __rtic_internal_tim2_Context as Context;
    } #[allow(non_snake_case)] fn tim2(mut cx : tim2 :: Context)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ;
        cx.shared.ppm.lock(| ppm | { tim2_ppm(ppm); });
    } impl < 'a > __rtic_internal_operationalLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_operationalLocalResources
            {
                control_panel : & mut *
                (& mut *
                __rtic_internal_local_resource_control_panel.get_mut()).as_mut_ptr(),
                control_output : & mut *
                (& mut *
                __rtic_internal_local_resource_control_output.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_operationalSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_operationalSharedResources
            {
                ppm : shared_resources :: ppm_that_needs_to_be_locked ::
                new(), adc_module : shared_resources ::
                adc_module_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `operational` has access to"] pub struct
    __rtic_internal_operationalLocalResources < 'a >
    {
        #[allow(missing_docs)] pub control_panel : & 'a mut ControlPanel <
        stm32f4xx_hal :: gpio :: PA1 < stm32f4xx_hal :: gpio :: Analog > ,
        stm32f4xx_hal :: gpio :: PA7 < stm32f4xx_hal :: gpio :: Analog > ,
        stm32f4xx_hal :: gpio :: PA5 < stm32f4xx_hal :: gpio :: Analog > ,
        stm32f4xx_hal :: gpio :: PA4 < stm32f4xx_hal :: gpio :: Analog > ,
        stm32f4xx_hal :: gpio :: PB1 < stm32f4xx_hal :: gpio :: Analog > ,
        stm32f4xx_hal :: gpio :: PB0 < stm32f4xx_hal :: gpio :: Analog > , > ,
        #[allow(missing_docs)] pub control_output : & 'a mut ControlOutput,
        #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () > ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `operational` has access to"] pub struct
    __rtic_internal_operationalSharedResources < 'a >
    {
        #[allow(missing_docs)] pub ppm : shared_resources ::
        ppm_that_needs_to_be_locked < 'a > , #[allow(missing_docs)] pub
        adc_module : shared_resources :: adc_module_that_needs_to_be_locked <
        'a > , #[doc(hidden)] pub __rtic_internal_marker : core :: marker ::
        PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_operational_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : operational :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        operational :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_operational_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_operational_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                operational :: LocalResources :: new(), shared : operational
                :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_operational_spawn() -> :: core ::
    result :: Result < (), () >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(operational, & __rtic_internal_operational_EXEC);
            if exec.try_allocate()
            {
                exec.spawn(operational(unsafe
                { operational :: Context :: new() })); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2); Ok(())
            } else { Err(()) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_operational_waker() -> :: core ::
    task :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(operational, & __rtic_internal_operational_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(operational, &
                __rtic_internal_operational_EXEC); exec.set_pending(); rtic ::
                export :: pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod operational
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_operationalLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_operationalSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_operational_Context as
        Context; #[doc(inline)] pub use super ::
        __rtic_internal_operational_spawn as spawn; #[doc(inline)] pub use
        super :: __rtic_internal_operational_waker as waker;
    } #[allow(non_snake_case)] async fn operational < 'a >
    (mut cx : operational :: Context < 'a >)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; defmt ::
        info! ("Engaging operational mode."); loop
        {
            cx.shared.ppm.lock(| ppm | { filter_ppm(ppm); }); *
            cx.local.control_output =
            cx.shared.adc_module.lock(| adc_module | ControlOutput
            {
                roll :
                format_ppm(adc_module.convert(&
                cx.local.control_panel.right_joystick.x.pin, SampleTime ::
                Cycles_480), cx.local.control_panel.right_joystick.x.min,
                cx.local.control_panel.right_joystick.x.max), pitch :
                format_ppm(adc_module.convert(&
                cx.local.control_panel.right_joystick.y.pin, SampleTime ::
                Cycles_480), cx.local.control_panel.right_joystick.x.min,
                cx.local.control_panel.right_joystick.x.max), throttle :
                format_ppm(adc_module.convert(&
                cx.local.control_panel.left_potentiometer.pin, SampleTime ::
                Cycles_480), cx.local.control_panel.right_joystick.x.min,
                cx.local.control_panel.right_joystick.x.max), yaw :
                format_ppm(adc_module.convert(&
                cx.local.control_panel.left_joystick.x.pin, SampleTime ::
                Cycles_480), cx.local.control_panel.right_joystick.x.min,
                cx.local.control_panel.right_joystick.x.max), mode :
                cx.local.control_output.mode,
            });
            cx.shared.ppm.lock(| ppm |
            {
                ppm.ppm [0] = cx.local.control_output.roll; ppm.ppm [1] =
                cx.local.control_output.pitch; ppm.ppm [2] =
                cx.local.control_output.throttle; ppm.ppm [3] =
                cx.local.control_output.yaw; ppm.ppm [4] = 1500; ppm.ppm [5] =
                1500; ppm.ppm [6] = 1500; ppm.ppm [7] =
                cx.local.control_output.mode;
            }); Mono :: delay(10.millis()).await;
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_ppm : rtic :: RacyCell < core :: mem ::
    MaybeUninit < PpmState >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: ppm_that_needs_to_be_locked < 'a >
    {
        type T = PpmState; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut PpmState) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 3u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_ppm.get_mut() as * mut _,
                CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_delay_val : rtic :: RacyCell < core :: mem
    :: MaybeUninit < u32 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: delay_val_that_needs_to_be_locked < 'a >
    {
        type T = u32; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u32) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 0u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_delay_val.get_mut() as *
                mut _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_shared_resource_adc_module : rtic :: RacyCell < core ::
    mem :: MaybeUninit < Adc < ADC1 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: adc_module_that_needs_to_be_locked < 'a >
    {
        type T = Adc < ADC1 > ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Adc < ADC1 >) -> RTIC_INTERNAL_R)
        -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_adc_module.get_mut() as *
                mut _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        ppm_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > ppm_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                ppm_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        delay_val_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > delay_val_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                delay_val_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        adc_module_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > adc_module_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                adc_module_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_delay : rtic :: RacyCell < core :: mem ::
    MaybeUninit < timer :: DelayMs < TIM1 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic4"] static
    __rtic_internal_local_resource_builtin_led : rtic :: RacyCell < core ::
    mem :: MaybeUninit < PC13 < Output > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic5"] static
    __rtic_internal_local_resource_buzzer : rtic :: RacyCell < core :: mem ::
    MaybeUninit < PB9 < Output > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic6"] static
    __rtic_internal_local_resource_control_output : rtic :: RacyCell < core ::
    mem :: MaybeUninit < ControlOutput >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic7"] static
    __rtic_internal_local_resource_control_panel : rtic :: RacyCell < core ::
    mem :: MaybeUninit < ControlPanel < stm32f4xx_hal :: gpio :: PA1 <
    stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA7 <
    stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA5 <
    stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PA4 <
    stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PB1 <
    stm32f4xx_hal :: gpio :: Analog > , stm32f4xx_hal :: gpio :: PB0 <
    stm32f4xx_hal :: gpio :: Analog > , > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_upper_case_globals)] static __rtic_internal_operational_EXEC :
    rtic :: export :: executor :: AsyncTaskExecutorPtr = rtic :: export ::
    executor :: AsyncTaskExecutorPtr :: new(); #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 1"]
    #[no_mangle] unsafe fn EXTI2()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(operational, & __rtic_internal_operational_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(operational, &
                __rtic_internal_operational_EXEC); exec.set_pending(); rtic ::
                export :: pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2);
            });
        });
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() -> !
    {
        rtic :: export :: assert_send :: < PpmState > (); rtic :: export ::
        assert_send :: < Adc < ADC1 > > (); rtic :: export :: interrupt ::
        disable(); let mut core : rtic :: export :: Peripherals = rtic ::
        export :: Peripherals :: steal().into(); let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI0; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI1; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI2; const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'EXTI2' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI2, rtic :: export ::
        cortex_logical2hw(1u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI2); const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 3u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'TIM2' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM2, rtic :: export ::
        cortex_logical2hw(3u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM2); #[inline(never)] fn __rtic_init_resources < F >
        (f : F) where F : FnOnce() { f(); } let mut executors_size = 0; let
        executor = :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_1_args(operational)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_operational_EXEC.set_in_main(& executor); extern "C"
        { pub static _stack_start : u32; pub static __ebss : u32; } let
        stack_start = & _stack_start as * const _ as u32; let ebss = & __ebss
        as * const _ as u32; if stack_start > ebss
        {
            if rtic :: export :: msp :: read() <= ebss
            { panic! ("Stack overflow after allocating executors"); }
        }
        __rtic_init_resources(||
        {
            let (shared_resources, local_resources) =
            init(init :: Context :: new(core.into(), executors_size));
            __rtic_internal_shared_resource_ppm.get_mut().write(core :: mem ::
            MaybeUninit :: new(shared_resources.ppm));
            __rtic_internal_shared_resource_delay_val.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.delay_val));
            __rtic_internal_shared_resource_adc_module.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.adc_module));
            __rtic_internal_local_resource_delay.get_mut().write(core :: mem
            :: MaybeUninit :: new(local_resources.delay));
            __rtic_internal_local_resource_builtin_led.get_mut().write(core ::
            mem :: MaybeUninit :: new(local_resources.builtin_led));
            __rtic_internal_local_resource_buzzer.get_mut().write(core :: mem
            :: MaybeUninit :: new(local_resources.buzzer));
            __rtic_internal_local_resource_control_output.get_mut().write(core
            :: mem :: MaybeUninit :: new(local_resources.control_output));
            __rtic_internal_local_resource_control_panel.get_mut().write(core
            :: mem :: MaybeUninit :: new(local_resources.control_panel)); rtic
            :: export :: interrupt :: enable();
        }); idle(idle :: Context :: new())
    }
}