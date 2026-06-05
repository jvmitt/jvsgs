pub mod io {

    use stm32f4xx_hal::gpio::{Analog, ErasedPin, Input, Output, PushPull};

    pub struct Potentiometer<PIN> {
        pub pin: PIN,
        pub min: u16,
        pub max: u16,
    }

    pub struct Joystick<X, Y> {
        pub x: Potentiometer<X>,
        pub y: Potentiometer<Y>,
    }

    pub struct ControlPanel<LP, RP, LJX, LJY, RJX, RJY> {
        pub arm_switch: ErasedPin<Input>,
        pub abort_btn: ErasedPin<Input>,
        pub left_potentiometer: Potentiometer<LP>,
        pub right_potentiometer: Potentiometer<RP>,
        pub left_joystick: Joystick<LJX, LJY>,
        pub right_joystick: Joystick<RJX, RJY>,
    }

    pub struct ControlOutput {
        pub roll: u16,
        pub pitch: u16,
        pub throttle: u16,
        pub yaw: u16,
        pub mode: u16,
    }
}
