pub struct SexiTime {
    value: crate::sexi::SexiData<12>,
}
impl SexiTime {
    pub fn get_ms(&self) -> i32 {
        self.value.get_ms()
    }
    pub fn get_sec(&self) -> i32 {
        self.value.get_sec()
    }
    pub fn get_min(&self) -> i32 {
        self.value.get_min()
    }
    pub fn get_hour(&self) -> i32 {
        self.value.get_top()
    }
}
impl SexiTime {
    pub fn set_ms(&mut self, ms: i32) {
        self.value.set_ms(ms)
    }
    pub fn set_sec(&mut self, sec: i32) {
        self.value.set_sec(sec)
    }
    pub fn set_min(&mut self, min: i32) {
        self.value.set_min(min)
    }
    pub fn set_hour(&mut self, deg: i32) {
        self.value.set_top(deg)
    }
}
mod fmt {
    use std::fmt::{Display, Formatter, Result};
    impl Display for super::SexiTime {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let d = self.get_hour();
            let m = self.get_min();
            let s = self.get_sec();
            let ms = self.get_ms();

            if ms != 0 {
                write!(f, "{:>3}:{:02}:{:02}.{:03}", d, m, s, ms)
            } else if s != 0 || m != 0 {
                write!(f, "{:>2}:{:02}:{:02}", d, m, s)
            } else {
                write!(f, "{:>2} O-Clock", d)
            }
        }
    }
}
