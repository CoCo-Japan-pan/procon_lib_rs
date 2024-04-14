//! 蟻本p165をもとにしている  
//! fenwick tree を二つ用いて、区間加算、区間和クエリを処理します  

use fenwick_tree::FenwickTree;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg, RangeBounds, Sub};

pub struct RAQRSQ<
    T: Clone
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + Neg<Output = T>
        + TryFrom<usize>
        + Mul<Output = T>,
> where
    <T as TryFrom<usize>>::Error: Debug,
{
    range_size: usize,
    ft1: FenwickTree<T>,
    ft2: FenwickTree<T>,
}

impl<
        T: Clone
            + Add<Output = T>
            + AddAssign
            + Sub<Output = T>
            + Neg<Output = T>
            + TryFrom<usize>
            + Mul<Output = T>,
    > RAQRSQ<T>
where
    <T as TryFrom<usize>>::Error: Debug,
{
    pub fn new(size: usize, zero: T) -> Self {
        Self {
            range_size: size,
            ft1: FenwickTree::new(size + 1, zero.clone()),
            ft2: FenwickTree::new(size + 1, zero),
        }
    }

    /// 1点加算
    pub fn add_point(&mut self, idx: usize, val: T) {
        assert!(idx < self.range_size);
        self.ft1.add(idx, val);
    }

    /// 区間加算
    pub fn add<R: RangeBounds<usize>>(&mut self, range: R, val: T) {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let right = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.range_size,
        };
        self.ft1.add(left, -val.clone() * left.try_into().unwrap());
        self.ft2.add(left, val.clone());
        self.ft1.add(right, val.clone() * right.try_into().unwrap());
        self.ft2.add(right, -val);
    }

    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.range_size,
        };
        assert!(start <= end && end <= self.range_size);
        self.sum_from_first(end) - self.sum_from_first(start)
    }

    fn sum_from_first(&self, idx: usize) -> T {
        assert!(idx <= self.range_size);
        self.ft1.sum(0..idx) + self.ft2.sum(0..idx) * idx.try_into().unwrap()
    }
}
