//! 行列ライブラリ 行列積は普通に`O(d^3)`で計算される

use internal_type_traits::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T: Copy + AddAssign + Mul<Output = T> + Zero + One> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Copy + AddAssign + Mul<Output = T> + Zero + One> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
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

impl<T: Copy + AddAssign + Mul<Output = T> + Zero + One> Matrix<T> {
    pub fn new(height: usize, width: usize, default_val: T) -> Self {
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

    pub fn get(&self, h: usize, w: usize) -> T {
        self.data[h * self.width + w]
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

impl<T: Copy + AddAssign + Mul<Output = T> + Zero + One> MulAssign<&Self> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.height);
        let mut res = Matrix::new(self.height, rhs.width, T::zero());
        for i in 0..self.height {
            for k in 0..self.width {
                for j in 0..rhs.width {
                    res.data[i * res.width + j] +=
                        self.data[i * self.width + k] * rhs.data[k * rhs.width + j];
                }
            }
        }
        *self = res;
    }
}

impl<T: Copy + AddAssign + Mul<Output = T> + Default + Zero + One> Mul<&Self> for Matrix<T> {
    type Output = Self;
    fn mul(mut self, rhs: &Self) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Copy + AddAssign + Mul<Output = T> + Default + Zero + One> AddAssign<&Self> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.height, rhs.height);
        assert_eq!(self.width, rhs.width);
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i * self.width + j] += rhs.data[i * self.width + j];
            }
        }
    }
}

impl<T: Copy + AddAssign + Mul<Output = T> + Default + Zero + One> Add<&Self> for Matrix<T> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}

impl<T: Copy + AddAssign + Mul<Output = T> + Default + SubAssign + Zero + One> SubAssign<&Self>
    for Matrix<T>
{
    fn sub_assign(&mut self, rhs: &Self) {
        assert_eq!(self.height, rhs.height);
        assert_eq!(self.width, rhs.width);
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i * self.width + j] -= rhs.data[i * self.width + j];
            }
        }
    }
}

impl<T: Copy + AddAssign + Mul<Output = T> + Default + SubAssign + Zero + One> Sub<&Self>
    for Matrix<T>
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

    #[test]
    fn test_matrix() {
        let a = Matrix::<i32>::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::<i32>::from(vec![vec![5, 6], vec![7, 8]]);
        let c = Matrix::<i32>::from(vec![vec![19, 22], vec![43, 50]]);
        assert_eq!(a * &b, c);
    }

    #[test]
    fn test_matrix_pow() {
        let a = Matrix::<i32>::from(vec![vec![2, 0], vec![0, 3]]);
        let b = Matrix::<i32>::from(vec![vec![32, 0], vec![0, 243]]);
        assert_eq!(a.pow(5), b);
    }
}
