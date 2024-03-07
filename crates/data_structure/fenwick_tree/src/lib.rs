use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FenwickTree<T: Clone + AddAssign + Sub<Output = T>> {
    size: usize,
    e: T,
    pow_2_floor: usize,
    data: Vec<T>,
}

impl<T: Clone + AddAssign + Sub<Output = T>> FenwickTree<T> {
    pub fn new(size: usize, e: T) -> Self {
        let pow_2_floor = 1_usize << size.ilog2();
        Self {
            size,
            e: e.clone(),
            pow_2_floor,
            data: vec![e; size + 1],
        }
    }

    pub fn add(&mut self, mut idx: usize, val: T) {
        idx += 1;
        while idx <= self.size {
            self.data[idx] += val.clone();
            idx += idx.trailing_zeros() as usize;
        }
    }

    pub fn set(&mut self, idx: usize, val: T) {
        self.add(idx, val - self.sum(idx..idx + 1));
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

    /// a[0] + ... a[x] >= w となる最小の x を返す
    pub fn lower_bound(&self, mut w: T) -> usize
    where
        T: PartialOrd + SubAssign,
    {
        if w <= self.e {
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
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.data[idx].clone();
            idx -= idx.trailing_zeros() as usize;
        }
        sum
    }
}
