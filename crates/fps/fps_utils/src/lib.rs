use ntt::{convolution, ConvHelper};
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fps<T: ConvHelper> {
    pub data: Vec<T>,
}

impl<T: ConvHelper> Fps<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: ConvHelper> Mul for Fps<T> {
    type Output = Fps<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fps::new(convolution(&self.data, &rhs.data))
    }
}

impl<T: ConvHelper> Mul for &Fps<T> {
    type Output = Fps<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fps::new(convolution(&self.data, &rhs.data))
    }
}
