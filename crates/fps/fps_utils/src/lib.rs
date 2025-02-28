use enumerate_inv_mods::enumerate_invs;
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

impl<T: ConvHelper> From<Vec<T>> for Fps<T> {
    fn from(data: Vec<T>) -> Self {
        assert!(data.len() < T::modulus() as usize);
        Self { data }
    }
}

impl<T: ConvHelper> Fps<T> {
    pub fn new(deg: usize) -> Self {
        assert!(deg < T::modulus() as usize);
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
        let invs = enumerate_invs::<T>(n + 1);
        for i in 1..n + 1 {
            data[i] = self.data[i - 1] * invs[i];
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
        let one = T::new(1_u8);
        let mut g = Fps::from(vec![one]);
        let log = ceil_log2(deg as u32) as usize;
        // mod x^(2^i)を求める
        for i in 1..=log {
            let mut f = self.truncate(1 << i);
            f.data[0] += one;
            g = (&g * &(f - &g.log(1 << i))).truncate(1 << i);
        }
        g.truncate(deg)
    }
}

/// `[x^n] p(x)/q(x)` を `O(k log k log n)` 時間で求める  
/// `deg(p) < k, deg(q) = k, q[0] = 1` とする  
/// [moriさんの記事](https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a#bostanmori-%E3%81%AE%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0-1)
pub fn bostan_mori<T: ConvHelper>(p: &Fps<T>, q: &Fps<T>, mut n: u64) -> T {
    assert!(!p.is_empty() && !q.is_empty());
    assert!(p.len() < q.len());
    assert_eq!(q.data[0].value(), 1);
    let mut p = p.clone();
    let mut q = q.clone();
    while n > 0 {
        let mut q_minus_x = q.clone();
        for i in (1..q.len()).step_by(2) {
            q_minus_x.data[i] = -q_minus_x.data[i];
        }
        let vx2 = q * &q_minus_x;
        let vx = Fps::from(vx2.data.into_iter().step_by(2).collect::<Vec<T>>());
        let pqminus = p * q_minus_x;
        let u = if n % 2 == 0 {
            Fps::from(pqminus.data.into_iter().step_by(2).collect::<Vec<T>>())
        } else {
            Fps::from(
                pqminus
                    .data
                    .into_iter()
                    .skip(1)
                    .step_by(2)
                    .collect::<Vec<T>>(),
            )
        };
        p = u;
        q = vx;
        n >>= 1;
    }
    p.data[0] / q.data[0]
}

macro_rules! impl_ops {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident) => {
        impl<T: ConvHelper, S> $trait<S> for Fps<T>
        where
            Self: $assign_trait<S>,
        {
            type Output = Fps<T>;
            fn $method(mut self, rhs: S) -> Self::Output {
                Fps::<T>::$assign_method(&mut self, rhs);
                self
            }
        }
        impl<T: ConvHelper, S> $trait<S> for &Fps<T>
        where
            Fps<T>: $trait<S, Output = Fps<T>>,
        {
            type Output = Fps<T>;
            fn $method(self, rhs: S) -> Self::Output {
                Fps::<T>::$method(self.clone(), rhs)
            }
        }
        impl<T: ConvHelper> $assign_trait for Fps<T> {
            fn $assign_method(&mut self, rhs: Self) {
                Fps::<T>::$assign_method(self, &rhs)
            }
        }
    };
}

impl_ops!(Add, add, AddAssign, add_assign);
impl_ops!(Sub, sub, SubAssign, sub_assign);
impl_ops!(Mul, mul, MulAssign, mul_assign);

impl<T: ConvHelper> AddAssign<&Self> for Fps<T> {
    fn add_assign(&mut self, rhs: &Self) {
        let n = self.len().max(rhs.len());
        self.data.resize(n, T::raw(0));
        for (idx, &r) in rhs.data.iter().enumerate() {
            self.data[idx] += r;
        }
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

impl<T: ConvHelper> MulAssign<&Self> for Fps<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.data = convolution(&self.data, &rhs.data);
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use static_modint::ModInt998244353 as MInt;

    #[test]
    fn test_bm_fib_all() {
        const N: usize = 100000;
        let naive = {
            let mut fib = vec![MInt::new(0); N];
            fib[1] += 1;
            for i in 2..N {
                fib[i] = fib[i - 1] + fib[i - 2];
            }
            fib
        };
        let p = Fps::from(vec![MInt::new(0), MInt::new(1)]);
        let q = Fps::from(vec![MInt::new(1), -MInt::new(1), -MInt::new(1)]);
        for i in 0..N {
            assert_eq!(bostan_mori(&p, &q, i as u64), naive[i]);
        }
    }

    #[test]
    fn test_bm_fib_big() {
        use matrix::{Matrix, UsualSemiring};
        type Rig = UsualSemiring<MInt>;
        let mut rng = thread_rng();
        fn use_matrix(n: u64) -> MInt {
            if n == 0 {
                return MInt::new(0);
            }
            let mat =
                Matrix::<Rig>::from([[MInt::new(1), MInt::new(1)], [MInt::new(1), MInt::new(0)]]);
            let vec = Matrix::from([[MInt::new(1)], [MInt::new(0)]]);
            let res = mat.pow(n - 1) * &vec;
            res.get(0, 0)
        }
        let p = Fps::from(vec![MInt::new(0), MInt::new(1)]);
        let q = Fps::from(vec![MInt::new(1), -MInt::new(1), -MInt::new(1)]);
        for _ in 0..1000 {
            let n = rng.gen_range(10_u64.pow(5)..=10_u64.pow(18));
            assert_eq!(bostan_mori(&p, &q, n), use_matrix(n));
        }
    }
}
