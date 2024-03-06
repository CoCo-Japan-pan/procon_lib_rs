use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const MOD: u64 = (1 << 61) - 1;

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
    pub fn new<T: RemEuclidU64>(x: T) -> Self {
        x.rem_euclid_u64()
    }
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
        Self::calc_mod(au * bu * 2 + midu + (midd << 31) + ad * bd)
    }
}

pub trait RemEuclidU64 {
    fn rem_euclid_u64(self) -> ModIntMersenne;
}

impl RemEuclidU64 for u32 {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        ModIntMersenne { value: self as u64 }
    }
}

impl RemEuclidU64 for u64 {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        ModIntMersenne {
            value: ModIntMersenne::calc_mod(self),
        }
    }
}

impl RemEuclidU64 for usize {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        let casted: u64 = self.try_into().unwrap();
        casted.rem_euclid_u64()
    }
}

impl RemEuclidU64 for i32 {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        if self < 0 {
            -(self.unsigned_abs().rem_euclid_u64())
        } else {
            (self as u64).rem_euclid_u64()
        }
    }
}

impl RemEuclidU64 for i64 {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        if self < 0 {
            -(self.unsigned_abs().rem_euclid_u64())
        } else {
            (self as u64).rem_euclid_u64()
        }
    }
}

impl RemEuclidU64 for isize {
    fn rem_euclid_u64(self) -> ModIntMersenne {
        let casted: i64 = self.try_into().unwrap();
        casted.rem_euclid_u64()
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
