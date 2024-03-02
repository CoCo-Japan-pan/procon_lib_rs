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
    #[inline]
    fn calc_mod(x: u64) -> u64 {
        let tmp = (x >> 61) + (x & MOD);
        if tmp >= MOD {
            tmp - MOD
        } else {
            tmp
        }
    }
    /// https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
    #[inline]
    fn mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & ((1 << 31) - 1);
        let bu = b >> 31;
        let bd = b & ((1 << 31) - 1);
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & ((1 << 30) - 1);
        Self::calc_mod(
            au * bu * 2 + midu + (midd << 31) + ad * bd,
        )
    }
}

const MOD: u64 = (1 << 61) - 1;

impl ModInt for ModIntMersenne {
    fn pow(&self, mut n: u64) -> Self {
        let mut ret = ModIntMersenne::new(1);
        let mut base = *self;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * base;
            }
            base = base * base;
            n >>= 1;
        }
        ret
    }
    fn inv(&self) -> Self {
        self.pow(MOD - 2)
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
        ModIntMersenne {
            value: ModIntMersenne::calc_mod(x),
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
        self.value = Self::mul(self.value, rhs.value);
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
        self.mul_assign(rhs.inv());
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
