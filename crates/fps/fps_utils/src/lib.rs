use internal_bits::ceil_log2;
use ntt::{convolution, ConvHelper};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fps<T: ConvHelper> {
    pub data: Vec<T>,
}

impl<T: ConvHelper> Display for Fps<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        write!(f, "{}", self.data[0])?;
        for x in self.data.iter().skip(1) {
            write!(f, " {}", x)?;
        }
        Ok(())
    }
}

impl<T: ConvHelper, S> From<Vec<S>> for Fps<T>
where
    T: From<S>,
{
    fn from(data: Vec<S>) -> Self {
        Self {
            data: data.into_iter().map(T::from).collect(),
        }
    }
}

impl<T: ConvHelper> Fps<T> {
    pub fn new(deg: usize) -> Self {
        Self {
            data: vec![T::raw(0); deg],
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// mod x^nを取る
    pub fn truncate(&self, n: usize) -> Self {
        let size = self.len().min(n);
        Self {
            data: self.data[..size].to_vec(),
        }
    }
}

#[allow(clippy::needless_range_loop)]
impl<T: ConvHelper> Fps<T> {
    /// 微分
    pub fn differential(&self) -> Self {
        let n = self.len();
        let mut data = vec![T::raw(0); n - 1];
        for i in 0..n - 1 {
            data[i] = self.data[i + 1] * T::new(i + 1);
        }
        Fps::from(data)
    }

    /// 積分
    pub fn integral(&self) -> Self {
        let n = self.len();
        let mut data = vec![T::raw(0); n + 1];
        for i in 1..n + 1 {
            data[i] = self.data[i - 1] / T::new(i);
        }
        Fps::from(data)
    }

    /// mod x^deg
    pub fn inverse(&self, deg: usize) -> Self {
        assert_ne!(self.data[0].value(), 0);
        let mut g = Fps::from(vec![self.data[0].inv()]);
        let log = ceil_log2(deg as u32) as usize;
        // mod x^(2^i)を求める
        for i in 1..=log {
            g = (&g * T::new(2) - &(&g * &g * self.truncate(1 << i))).truncate(1 << i);
        }
        g.truncate(deg)
    }

    /// mod x^deg
    pub fn log(&self, deg: usize) -> Self {
        assert_eq!(self.data[0].value(), 1);
        let f = self.differential() * self.inverse(deg);
        f.truncate(deg - 1).integral()
    }

    /// mod x^deg
    pub fn exp(&self, deg: usize) -> Self {
        assert_eq!(self.data[0].value(), 0);
        let mut g = Fps::from(vec![T::new(1)]);
        let log = ceil_log2(deg as u32) as usize;
        // mod x^(2^i)を求める
        for i in 1..=log {
            let mut f = self.truncate(1 << i);
            f.data[0] += T::new(1);
            g = (&g * &(f - &g.log(1 << i))).truncate(1 << i);
        }
        g.truncate(deg)
    }
}

impl<T: ConvHelper> Add<&Self> for Fps<T> {
    type Output = Fps<T>;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: ConvHelper> Add for &Fps<T> {
    type Output = Fps<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let (big, small) = if self.len() < rhs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        let mut data = big.data.clone();
        for (idx, &s) in small.data.iter().enumerate() {
            data[idx] += s;
        }
        Fps::from(data)
    }
}

impl<T: ConvHelper> AddAssign<&Self> for Fps<T> {
    fn add_assign(&mut self, rhs: &Self) {
        let n = self.len().max(rhs.len());
        self.data.resize(n, T::raw(0));
        for (idx, &r) in rhs.data.iter().enumerate() {
            self.data[idx] += r;
        }
    }
}

impl<T: ConvHelper> Sub<&Self> for Fps<T> {
    type Output = Fps<T>;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: ConvHelper> Sub for &Fps<T> {
    type Output = Fps<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let (big, small) = if self.len() < rhs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        let mut data = big.data.clone();
        for (idx, &s) in small.data.iter().enumerate() {
            data[idx] -= s;
        }
        Fps::from(data)
    }
}

impl<T: ConvHelper> SubAssign<&Self> for Fps<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        let n = self.len().max(rhs.len());
        self.data.resize(n, T::raw(0));
        for (idx, &r) in rhs.data.iter().enumerate() {
            self.data[idx] -= r;
        }
    }
}

impl<T: ConvHelper> Mul for &Fps<T> {
    type Output = Fps<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fps::from(convolution(&self.data, &rhs.data))
    }
}

impl<T: ConvHelper> Mul for Fps<T> {
    type Output = Fps<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<T: ConvHelper> Mul<&Self> for Fps<T> {
    type Output = Fps<T>;
    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: ConvHelper> MulAssign<&Self> for Fps<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.data = convolution(&self.data, &rhs.data);
    }
}

impl<T: ConvHelper> Mul<T> for Fps<T> {
    type Output = Fps<T>;
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: ConvHelper> Mul<T> for &Fps<T> {
    type Output = Fps<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let ret = self.clone();
        ret * rhs
    }
}

impl<T: ConvHelper> MulAssign<T> for Fps<T> {
    fn mul_assign(&mut self, rhs: T) {
        for x in self.data.iter_mut() {
            *x *= rhs;
        }
    }
}

impl<T: ConvHelper> Neg for Fps<T> {
    type Output = Fps<T>;
    fn neg(self) -> Self::Output {
        Fps::from(self.data.into_iter().map(|x| -x).collect::<Vec<T>>())
    }
}
