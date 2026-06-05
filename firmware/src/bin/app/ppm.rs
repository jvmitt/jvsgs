pub mod ppm {

    use rtic::Mutex;
    use stm32f4xx_hal::{
        gpio::{ErasedPin, Input, Output, PushPull},
        pac::TIM2,
    };

    pub struct PpmConfig {
        pub channel_number: usize,
        pub channel_to_modify: usize,
        pub channel_throttle: usize,
        pub channel_default_value: u16,
        pub frame_length: u16,
        pub pulse_length: u16,
        pub on_state: bool,
        pub signal_pin: ErasedPin<Output<PushPull>>,
        pub switch_pin: ErasedPin<Input>,
        pub switch_step: u16,
    }

    pub struct PpmState {
        pub config: PpmConfig,
        pub ppm: [u16; 16],
        pub state: bool,
        pub current_channel_step: i16,
        pub previous_switch_value: u8,
        pub tick_hz: u32,
    }

    pub fn format_ppm(input: u16, input_min: u16, input_max: u16) -> u16 {
        let mut input = input as u32;
        let input_min = input_min as u32;
        let input_max = input_max as u32;
        let output_min: u32 = 1000;
        let output_max: u32 = 2000;
        if input < input_min {
            input = input_min;
        } else if input > input_max {
            input = input_max;
        };
        (((input - input_min) * (output_max - output_min) + (input_max - input_min) / 2)
            / (input_max - input_min)
            + output_min) as u16
    }

    pub fn init_ppm(
        channel_number: usize,
        channel_to_modify: usize,
        channel_throttle: usize,
        channel_default_value: u16,
        frame_length: u16,
        pulse_length: u16,
        on_state: bool,
        signal_pin: ErasedPin<Output<PushPull>>,
        switch_pin: ErasedPin<Input>,
        switch_step: u16,
        tim: &stm32f4xx_hal::pac::tim2::RegisterBlock,
        apb_2x: bool,
        pclk_hz: u32,
        tick_hz: u32,
    ) -> PpmState {
        let mut config = PpmConfig {
            channel_number,
            channel_to_modify,
            channel_throttle,
            channel_default_value,
            frame_length,
            pulse_length,
            on_state,
            signal_pin, // Output
            switch_pin, // Input Pullup
            switch_step,
        };

        let previous_switch_value: u8 = 1;

        // PPM
        let mut ppm: [u16; 16] = [0; 16];
        for i in 0..ppm.len() {
            if i < config.channel_number {
                if i == config.channel_throttle || i == config.channel_to_modify {
                    ppm[i] = 1000;
                } else {
                    ppm[i] = 1500;
                };
            };
        }

        if on_state {
            config.signal_pin.set_low();
        } else {
            config.signal_pin.set_high();
        };

        // TIM 2 config for PPM init
        let tim_clk_hz = if apb_2x { pclk_hz * 2 } else { pclk_hz };
        assert!(tim_clk_hz % tick_hz == 0);
        let psc: u16 = ((tim_clk_hz / tick_hz) - 1) as u16;
        tim.cr1().modify(|_, w| w.cen().clear_bit());
        tim.psc().write(|w| unsafe { w.psc().bits(psc) });
        tim.arr().write(|w| unsafe { w.arr().bits(u32::MAX) });
        tim.ccmr1_output()
            .modify(|_, w| unsafe { w.oc1m().bits(0) });
        tim.ccer().modify(|_, w| w.cc1e().set_bit());
        tim.cnt().write(|w| unsafe { w.cnt().bits(0) });
        tim.ccr1().write(|w| unsafe { w.ccr().bits(100) });
        tim.egr().write(|w| w.ug().set_bit());
        tim.sr().modify(|_, w| {
            w.uif().clear_bit();
            w.cc1if().clear_bit()
        });
        tim.dier().modify(|_, w| w.cc1ie().set_bit());
        tim.cr1().modify(|_, w| w.cen().set_bit());

        let current_channel_step: i16 = config.switch_step as i16;
        let state: bool = true;

        PpmState {
            config,
            ppm,
            state,
            current_channel_step,
            previous_switch_value,
            tick_hz,
        }
    }

    pub fn filter_ppm(ppm: &mut PpmState) {
        let switch_state: u8;
        if ppm.config.switch_pin.is_high() {
            switch_state = 1;
        } else {
            switch_state = 0;
        };

        if ppm.ppm[ppm.config.channel_to_modify] >= 0 {
            ppm.ppm[ppm.config.channel_to_modify] += ppm.current_channel_step as u16;
        } else {
            ppm.ppm[ppm.config.channel_to_modify] -= ppm.current_channel_step as u16;
        };

        if ppm.ppm[ppm.config.channel_to_modify] > 2000
            || ppm.ppm[ppm.config.channel_to_modify] < 1000
        {
            ppm.current_channel_step *= -1;
            if ppm.ppm[ppm.config.channel_to_modify] >= 0 {
                ppm.ppm[ppm.config.channel_to_modify] += ppm.current_channel_step as u16;
            } else {
                ppm.ppm[ppm.config.channel_to_modify] -= ppm.current_channel_step as u16;
            };
        };

        ppm.previous_switch_value = switch_state;
    }

    pub fn tim2_ppm(ppm: &mut PpmState) {
        let tim2 = unsafe { &*TIM2::ptr() };
        if tim2.sr().read().cc1if().bit_is_clear() {
            return;
        }
        tim2.sr().write(|w| w.cc1if().clear_bit());
        let current_ccr = tim2.ccr1().read().ccr().bits();
        let pulse_len_us = ppm.config.pulse_length as u32;
        let frame_len_us = ppm.config.frame_length as u32;

        unsafe {
            if ppm.state {
                if ppm.config.on_state {
                    ppm.config.signal_pin.set_high()
                } else {
                    ppm.config.signal_pin.set_low()
                };

                let next_ccr = current_ccr.wrapping_add(pulse_len_us * 2);
                tim2.ccr1().write(|w| w.ccr().bits(next_ccr));

                ppm.state = false;
            } else {
                static mut CUR_CHAN_NUMB: usize = 0;
                static mut CALC_REST: u32 = 0;

                if ppm.config.on_state {
                    ppm.config.signal_pin.set_low()
                } else {
                    ppm.config.signal_pin.set_high()
                };
                ppm.state = true;

                if CUR_CHAN_NUMB >= ppm.config.channel_number {
                    CUR_CHAN_NUMB = 0;
                    CALC_REST += ppm.config.pulse_length as u32;

                    let rest_us = frame_len_us.saturating_sub(CALC_REST);
                    let next_ccr = current_ccr.wrapping_add(rest_us * 2);
                    tim2.ccr1().write(|w| w.ccr().bits(next_ccr));

                    CALC_REST = 0;
                } else {
                    let chan_us = ppm.ppm[CUR_CHAN_NUMB] as u32;
                    let gap_us = chan_us.saturating_sub(pulse_len_us);
                    let next_ccr = current_ccr.wrapping_add(gap_us * 2);
                    tim2.ccr1().write(|w| w.ccr().bits(next_ccr));

                    CALC_REST += ppm.ppm[CUR_CHAN_NUMB] as u32;
                    CUR_CHAN_NUMB += 1;
                };
            };
        };
    }
}
