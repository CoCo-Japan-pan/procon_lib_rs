//! 行列ライブラリ 行列積は普通に`O(d^3)`で計算される

use algebra::Semiring;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T: Semiring> {
    height: usize,
    width: usize,
    data: Vec<T::Target>,
}

impl<T: Semiring> From<Vec<Vec<T::Target>>> for Matrix<T> {
    fn from(v: Vec<Vec<T::Target>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let data = v.into_iter().flatten().collect();
        Self {
            height,
            width,
            data,
        }
    }
}

impl<T: Semiring, const H: usize, const W: usize> From<[[T::Target; W]; H]> for Matrix<T> {
    fn from(v: [[T::Target; W]; H]) -> Self {
        let height = H;
        let width = W;
        let data = v.into_iter().flatten().collect();
        Self {
            height,
            width,
            data,
        }
    }
}

impl<T: Semiring> Matrix<T> {
    pub fn new(height: usize, width: usize, default_val: T::Target) -> Self {
        Self {
            height,
            width,
            data: vec![default_val; height * width],
        }
    }

    pub fn unit(n: usize) -> Self {
        let mut res = Self::new(n, n, T::zero());
        for i in 0..n {
            res.data[i * n + i] = T::one();
        }
        res
    }

    pub fn transpose(&self) -> Self {
        let mut res = Self::new(self.width, self.height, T::zero());
        for i in 0..self.height {
            for j in 0..self.width {
                res.data[j * self.height + i] = self.data[i * self.width + j].clone();
            }
        }
        res
    }

    pub fn get(&self, h: usize, w: usize) -> T::Target {
        assert!(h < self.height && w < self.width);
        self.data[h * self.width + w].clone()
    }

    pub fn get_mut(&mut self, h: usize, w: usize) -> &mut T::Target {
        assert!(h < self.height && w < self.width);
        &mut self.data[h * self.width + w]
    }

    pub fn pow(&self, mut n: u64) -> Self {
        assert_eq!(self.height, self.width);
        let mut res = Self::unit(self.height);
        let mut a = self.clone();
        while n > 0 {
            if (n & 1) == 1 {
                res *= &a;
            }
            a *= &a.clone();
            n >>= 1;
        }
        res
    }
}

impl<T: Semiring> MulAssign<&Self> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.height);
        let mut res = Matrix::new(self.height, rhs.width, T::zero());
        for i in 0..self.height {
            for k in 0..self.width {
                for j in 0..rhs.width {
                    T::add_assign(
                        &mut res.data[i * res.width + j],
                        &T::mul(&self.data[i * self.width + k], &rhs.data[k * rhs.width + j]),
                    );
                }
            }
        }
        *self = res;
    }
}

impl<T: Semiring> Mul<&Self> for Matrix<T> {
    type Output = Self;
    fn mul(mut self, rhs: &Self) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Semiring> AddAssign<&Self> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.height, rhs.height);
        assert_eq!(self.width, rhs.width);
        for i in 0..self.height {
            for j in 0..self.width {
                T::add_assign(
                    &mut self.data[i * self.width + j],
                    &rhs.data[i * self.width + j],
                );
            }
        }
    }
}

impl<T: Semiring> Add<&Self> for Matrix<T> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}

impl<T: Semiring> SubAssign<&Self> for Matrix<T>
where
    T::Target: SubAssign,
{
    fn sub_assign(&mut self, rhs: &Self) {
        assert_eq!(self.height, rhs.height);
        assert_eq!(self.width, rhs.width);
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i * self.width + j] -= rhs.data[i * self.width + j].clone();
            }
        }
    }
}

impl<T: Semiring> Sub<&Self> for Matrix<T>
where
    T::Target: SubAssign,
{
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UsualSemiring;
    impl Semiring for UsualSemiring {
        type Target = i32;
        fn zero() -> Self::Target {
            0
        }
        fn one() -> Self::Target {
            1
        }
        fn add_assign(a: &mut Self::Target, b: &Self::Target) {
            *a += b;
        }
        fn mul(a: &Self::Target, b: &Self::Target) -> Self::Target {
            a * b
        }
    }

    #[test]
    fn test_matrix() {
        let a = Matrix::<UsualSemiring>::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::<UsualSemiring>::from(vec![vec![5, 6], vec![7, 8]]);
        let c = Matrix::<UsualSemiring>::from(vec![vec![19, 22], vec![43, 50]]);
        assert_eq!(a * &b, c);
    }

    #[test]
    fn test_matrix_pow() {
        let a = Matrix::<UsualSemiring>::from(vec![vec![2, 0], vec![0, 3]]);
        let b = Matrix::<UsualSemiring>::from(vec![vec![32, 0], vec![0, 243]]);
        assert_eq!(a.pow(5), b);
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::<UsualSemiring>::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let b = Matrix::<UsualSemiring>::from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        assert_eq!(a.transpose(), b);
    }
}
