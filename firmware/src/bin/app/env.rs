pub mod env {

    pub struct Env {
        var: Var,
    }

    pub struct Var {
        pub is_armed: bool,
        pub status_code: u8,
    }

    pub fn init_env(is_armed: bool, status_code: u8) -> Env {
        let var = Var {
            is_armed,
            status_code,
        };

        Env { var }
    }
}
