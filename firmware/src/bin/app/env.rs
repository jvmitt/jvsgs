pub mod env {

    #![feature(min_adt_const_params)]

    pub struct Env {
        pub var: Var,
        pub board: Board,
    }

    pub struct Var {
        pub is_armed: bool,
        pub in_panic: bool,
        pub status_code: u8,
    }

    pub struct Board {
        pub name: &'static str,
        pub family: &'static str,
        pub mcu: &'static str,
        pub adc_resolution: u16,
    }

    pub fn init_env(
        is_armed: bool,
        in_panic: bool,
        status_code: u8,
        name: &'static str,
        family: &'static str,
        mcu: &'static str,
        adc_resolution: u16,
    ) -> Env {
        let var = Var {
            is_armed,
            in_panic,
            status_code,
        };

        let board = Board {
            name,
            family,
            mcu,
            adc_resolution,
        };

        Env { var, board }
    }
}
