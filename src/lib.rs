use fixed::types::U6F10;
pub struct Sexitismal {
    seconds: U6F10,
    deg_min: u16,
}
mod constants {
    use fixed::types::U6F10;
    impl super::Sexitismal {
        pub const SECONDS_MODULUS: U6F10 = U6F10::const_from_int(60);
        pub const MINUTES_MODULUS: u16 = 60;
        pub const DEGREES_MODULUS: u16 = 360;

        pub const MINUTES_WIDTH: usize = 7;
        pub const MINUTES_OFFSET: usize = 0;
        pub const MINUTES_MASK: u16 =
            ((1 << Self::MINUTES_WIDTH) - 1) << Self::MINUTES_OFFSET;

        pub const DEGREES_WIDTH: usize = 16 - Self::MINUTES_WIDTH;
        pub const DEGREES_OFFSET: usize = Self::MINUTES_WIDTH;
        pub const DEGREES_MASK: u16 =
            ((1 << Self::DEGREES_WIDTH) - 1) << Self::DEGREES_OFFSET;
    }
}
mod getters {
    use fixed::types::U6F10;

    impl super::Sexitismal {
        pub fn get_seconds(&self) -> U6F10 {
            self.seconds
        }
        pub fn get_minutes(&self) -> u8 {
            ((self.deg_min & Self::MINUTES_MASK) >> Self::MINUTES_OFFSET) as u8
        }
        pub fn get_degrees(&self) -> u16 {
            (self.deg_min & Self::DEGREES_MASK) >> Self::DEGREES_OFFSET
        }
    }
}
mod setters {
    use fixed::types::U6F10;
    impl super::Sexitismal {
        pub fn set_degrees(&mut self, mut value: u16) {
            value %= Self::DEGREES_MODULUS;
            self.deg_min &= Self::MINUTES_MASK;
            self.deg_min |=
                (value << Self::DEGREES_OFFSET) & Self::DEGREES_MASK;
        }
        pub fn set_minutes(&mut self, mut value: u16) {
            value %= Self::MINUTES_MODULUS;
            self.deg_min &= Self::DEGREES_MASK;
            self.deg_min |=
                (value << Self::MINUTES_OFFSET) & Self::MINUTES_MASK;
        }
        pub fn set_seconds(&mut self, mut value: U6F10) {
            value %= Self::SECONDS_MODULUS;
            self.seconds = value;
        }
    }
}
