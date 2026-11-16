#[allow(dead_code)]
pub struct SexiData<const MOD: i32> {
    raw: i32,
}

/// constants
#[allow(dead_code)]
impl<const MOD: i32> SexiData<MOD> {
    const MS_MOD: i32 = 1000;
    const SEC_MOD: i32 = 60;
    const MIN_MOD: i32 = 60;
    const TOP_MOD: i32 = MOD;

    const MS_SCALER: i32 = 1;
    const SEC_SCALER: i32 = Self::MS_MOD;
    const MIN_SCALER: i32 = Self::SEC_SCALER * Self::SEC_MOD;
    const TOP_SCALER: i32 = Self::MIN_SCALER * Self::MIN_MOD;

    const TOTAL_MOD: i32 = Self::TOP_SCALER * Self::TOP_MOD;
}

/// getters
#[allow(dead_code)]
impl<const MOD: i32> SexiData<MOD> {
    pub fn get_ms(&self) -> i32 {
        self.raw / Self::MS_SCALER % Self::MS_MOD
    }
    pub fn get_sec(&self) -> i32 {
        self.raw / Self::SEC_SCALER % Self::SEC_MOD
    }
    pub fn get_min(&self) -> i32 {
        self.raw / Self::MIN_SCALER % Self::MIN_MOD
    }
    pub fn get_top(&self) -> i32 {
        self.raw / Self::TOP_SCALER % Self::TOP_MOD
    }
}

/// setters
#[allow(dead_code)]
impl<const MOD: i32> SexiData<MOD> {
    pub fn set_ms(&mut self, ms: i32) {
        self.raw -= self.get_ms() * Self::MS_SCALER;
        self.raw += ms;
    }
    pub fn set_sec(&mut self, sec: i32) {
        self.raw -= self.get_sec() * Self::SEC_SCALER;
        self.raw += sec * Self::SEC_SCALER;
    }
    pub fn set_min(&mut self, min: i32) {
        self.raw -= self.get_min() * Self::MIN_SCALER;
        self.raw += min * Self::MIN_SCALER;
    }
    pub fn set_top(&mut self, top: i32) {
        self.raw -= self.get_top() * Self::TOP_SCALER;
        self.raw += top * Self::TOP_SCALER;
    }
}

/// mathmatical trait implementations
mod math;
