pub struct SexiDir {
    value: crate::sexi::SexiData<360>,
}
impl SexiDir {
    pub fn get_ms(&self) -> i32 {
        self.value.get_ms()
    }
    pub fn get_sec(&self) -> i32 {
        self.value.get_sec()
    }
    pub fn get_min(&self) -> i32 {
        self.value.get_min()
    }
    pub fn get_deg(&self) -> i32 {
        self.value.get_top()
    }
}
impl SexiDir {
    pub fn set_ms(&mut self, ms: i32) {
        self.value.set_ms(ms)
    }
    pub fn set_sec(&mut self, sec: i32) {
        self.value.set_sec(sec)
    }
    pub fn set_min(&mut self, min: i32) {
        self.value.set_min(min)
    }
    pub fn set_deg(&mut self, deg: i32) {
        self.value.set_top(deg)
    }
}
mod fmt {
    use std::fmt::{Display, Formatter, Result};
    impl Display for super::SexiDir {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let d = self.get_deg();
            let m = self.get_min();
            let s = self.get_sec();
            let ms = self.get_ms();

            if ms != 0 {
                write!(f, "{:>3}°{:02}'{:02}.{:03}", d, m, s, ms)
            } else if s != 0 || m != 0 {
                write!(f, "{:>3}°{:02}'{:02}", d, m, s)
            } else {
                write!(f, "{:>3}°", d)
            }
        }
    }
}

#[cfg(test)]
mod test {}
