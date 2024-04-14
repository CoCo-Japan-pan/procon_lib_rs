use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FenwickTree<T: Clone + AddAssign + Sub<Output = T>> {
    size: usize,
    pow_2_floor: usize,
    zero: T,
    data: Vec<T>,
}

impl<T: Clone + AddAssign + Sub<Output = T>> FenwickTree<T> {
    pub fn new(size: usize, zero: T) -> Self {
        let pow_2_floor = if size == 0 { 0 } else { 1 << size.ilog2() };
        Self {
            size,
            pow_2_floor,
            zero: zero.clone(),
            data: vec![zero; size + 1],
        }
    }

    pub fn add(&mut self, mut idx: usize, val: T) {
        assert!(idx < self.size);
        idx += 1;
        while idx <= self.size {
            self.data[idx] += val.clone();
            idx += idx & idx.wrapping_neg()
        }
    }

    pub fn set(&mut self, idx: usize, val: T) {
        assert!(idx < self.size);
        self.add(idx, val - self.sum(idx..=idx));
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
            std::ops::Bound::Unbounded => self.size,
        };
        assert!(start <= end && end <= self.size);
        self.sum_from_first(end) - self.sum_from_first(start)
    }

    /// `a[0] + ... a[x] >= w` となる最小の x を返す
    pub fn lower_bound(&self, mut w: T) -> usize
    where
        T: PartialOrd + SubAssign,
    {
        if w <= self.zero {
            return 0;
        }
        let mut x = 0;
        let mut k = self.pow_2_floor;
        while k > 0 {
            if x + k <= self.size && self.data[x + k] < w {
                w -= self.data[x + k].clone();
                x += k;
            }
            k >>= 1;
        }
        x
    }

    fn sum_from_first(&self, mut idx: usize) -> T {
        assert!(idx <= self.size);
        let mut sum = self.zero.clone();
        while idx > 0 {
            sum += self.data[idx].clone();
            idx -= idx & idx.wrapping_neg();
        }
        sum
    }
}

/// From https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/fenwicktree.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound::*;

    #[test]
    fn fenwick_tree_works() {
        let mut bit = FenwickTree::<i64>::new(5, 0i64);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0..5), 15);
        assert_eq!(bit.sum(0..4), 10);
        assert_eq!(bit.sum(1..3), 5);

        assert_eq!(bit.sum(..), 15);
        assert_eq!(bit.sum(..2), 3);
        assert_eq!(bit.sum(..=2), 6);
        assert_eq!(bit.sum(1..), 14);
        assert_eq!(bit.sum(1..=3), 9);
        assert_eq!(bit.sum((Excluded(0), Included(2))), 5);
    }
}
