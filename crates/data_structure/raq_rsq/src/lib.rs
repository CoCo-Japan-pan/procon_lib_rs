//! 蟻本p165をもとにしている  
//! fenwick tree を二つ用いて、区間加算、区間和クエリを処理します  

use fenwick_tree::FenwickTree;
use std::ops::{Add, AddAssign, Bound::*, Mul, Neg, RangeBounds, Sub};

pub struct RAQRSQ<
    T: Clone
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + Neg<Output = T>
        + From<u32>
        + Mul<Output = T>,
> {
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
            + From<u32>
            + Mul<Output = T>,
    > RAQRSQ<T>
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

    fn get_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let begin = match range.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => self.range_size,
        };
        assert!(begin <= end && end <= self.range_size);
        (begin, end)
    }

    /// 区間加算
    pub fn add<R: RangeBounds<usize>>(&mut self, range: R, val: T) {
        let (begin, end) = self.get_range(range);
        self.ft1.add(begin, -val.clone() * (begin as u32).into());
        self.ft2.add(begin, val.clone());
        self.ft1.add(end, val.clone() * (end as u32).into());
        self.ft2.add(end, -val);
    }

    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        let (begin, end) = self.get_range(range);
        self.sum_from_first(end) - self.sum_from_first(begin)
    }

    fn sum_from_first(&self, idx: usize) -> T {
        assert!(idx <= self.range_size);
        self.ft1.sum(0..idx) + self.ft2.sum(0..idx) * (idx as u32).into()
    }
}
