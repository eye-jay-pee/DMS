use super::*;
impl Dms {
    pub fn is_negative(&self) -> bool {}
    pub fn is_positive(&self) -> bool {}
}
impl Dms {
    pub fn get_degrees(&self) -> i16 {
        (self.raw >> DEGREES_BIT_OFFSET & DEGREES_BIT_MASK) as u16
    }
}
impl Dms {
    pub fn get_minutes(&self) -> u8 {
        (self.raw >> MINUTES_BIT_OFFSET & MINUTES_BIT_MASK) as u8
    }
}
impl Dms {
    pub fn get_seconds(&self) -> f32 {
        let s = (self.raw >> SECONDS_BIT_OFFSET & SECONDS_BIT_MASK) as f32;
        s / FIXED_POINT_DIVISOR
    }
}
