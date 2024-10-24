use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

/// 二次元座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    /// 内積
    pub fn dot(self, rhs: Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
    /// 外積  
    /// これが0以上なら、self -> rhs へは反時計回り180度以内で行ける  
    /// これが0以下なら、self -> rhs へは時計回り180度以内で行ける
    pub fn cross(self, rhs: Self) -> i64 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}
