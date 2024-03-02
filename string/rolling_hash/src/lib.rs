use modint_traits::{FromPrimitiveInt, ModInt};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModIntMersenne {
    value: u64,
}

impl Display for ModIntMersenne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ModIntMersenne {
    pub fn value(&self) -> u64 {
        self.value
    }
    pub fn modulus() -> u64 {
        MOD
    }
}

const MOD: u64 = (1 << 61) - 1;

impl ModInt for ModIntMersenne {
    fn pow(&self, n: u64) -> Self {
        todo!()
    }
    fn inv(&self) -> Self {
        todo!()
    }
}

impl FromPrimitiveInt<u32> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: u32) -> Self::Output {
        ModIntMersenne { value: x.into() }
    }
}

impl FromPrimitiveInt<u64> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: u64) -> Self::Output {
        let tmp = (x >> 61) + (x & MOD);
        ModIntMersenne {
            value: if tmp >= MOD { tmp - MOD } else { tmp },
        }
    }
}

impl FromPrimitiveInt<usize> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: usize) -> Self {
        let casted: u64 = x.try_into().unwrap();
        ModIntMersenne::new(casted)
    }
}

impl FromPrimitiveInt<i32> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: i32) -> Self {
        if x == i32::MIN {
            -ModIntMersenne::new((x + (MOD as i32)).abs() as u64)
        } else if x < 0 {
            -ModIntMersenne::new(x.abs() as u64)
        } else {
            ModIntMersenne::new(x as u64)
        }
    }
}

impl FromPrimitiveInt<i64> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: i64) -> Self {
        if x == i64::MIN {
            -ModIntMersenne::new((x + (MOD as i64)).abs() as u64)
        } else if x < 0 {
            -ModIntMersenne::new(x.abs() as u64)
        } else {
            ModIntMersenne::new(x as u64)
        }
    }
}

impl FromPrimitiveInt<isize> for ModIntMersenne {
    type Output = ModIntMersenne;
    fn new(x: isize) -> Self {
        let casted: i64 = x.try_into().unwrap();
        ModIntMersenne::new(casted)
    }
}

impl Neg for ModIntMersenne {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value == 0 {
            self
        } else {
            ModIntMersenne {
                value: MOD - self.value,
            }
        }
    }
}

impl AddAssign for ModIntMersenne {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}

impl Add for ModIntMersenne {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp += rhs;
        tmp
    }
}

impl SubAssign for ModIntMersenne {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}

impl Sub for ModIntMersenne {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp -= rhs;
        tmp
    }
}

impl MulAssign for ModIntMersenne {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Mul for ModIntMersenne {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp *= rhs;
        tmp
    }
}

impl DivAssign for ModIntMersenne {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Div for ModIntMersenne {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp /= rhs;
        tmp
    }
}
