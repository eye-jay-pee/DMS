use super::SexiData;
mod add {
    use std::ops::Add;
    impl<const TOP: i32> Add for super::SexiData<TOP> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                raw: (self.raw + rhs.raw).rem_euclid(Self::TOTAL_MOD),
            }
        }
    }
}
mod sub {
    use std::ops::Sub;
    impl<const TOP: i32> Sub for super::SexiData<TOP> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                raw: (self.raw - rhs.raw).rem_euclid(Self::TOTAL_MOD),
            }
        }
    }
}
mod mul {
    use std::ops::Mul;
    macro_rules! impl_mul{
        ($($t:ty),*) => {
            $(
                impl<const MOD: i32> Mul<$t> for super::SexiData<MOD> {
                    type Output = Self;
                    fn mul(self, rhs: $t) -> Self::Output {
                        Self {
                            raw: ((self.raw as $t) * rhs) as i32
                        }
                    }
                }
            )*
        };
    }
    impl_mul!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
}
mod div {
    use std::ops::Div;
    macro_rules! impl_div{
        ($($t:ty),*) => {
            $(
                impl<const MOD: i32> Div<$t> for super::SexiData<MOD> {
                    type Output = Self;
                    fn div(self, rhs: $t) -> Self::Output {
                        Self {
                            raw: ((self.raw as $t) * rhs) as i32
                        }
                    }
                }
            )*
        };
    }
    impl_div!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
}
