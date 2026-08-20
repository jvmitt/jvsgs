pub mod models {

    // System Base
    pub struct Date {
        pub year: u16,
        pub month: u8,
        pub day: u8,
        pub hour: u8,
        pub minute: u8,
        pub second: u8,
    }

    // Logging
    pub enum CraftType {
        Plane,
        Quad,
        Vtol,
    }

    pub struct Craft {
        pub name: [u8; 64],
        pub model: [u8; 64],
        pub craft_type: CraftType,
        pub pid_file: [u8; 64],
        pub use_count: u64,
        pub last_use: Date,
    }
}
