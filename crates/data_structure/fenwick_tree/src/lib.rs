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
        self.add(idx, val - self.get(idx));
    }

    pub fn get(&self, idx: usize) -> T {
        assert!(idx < self.size);
        if (idx & 1) == 0 {
            self.data[idx + 1].clone()
        } else {
            self.sum(idx..=idx)
        }
    }

    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => self.size,
        };
        assert!(start <= end && end <= self.size);
        self.sum_from_first(end) - self.sum_from_first(start)
    }

    /// `a[0] + ... + a[x - 1] < w` を満たす最大の `x` を返す  
    /// 単調性(w未満とw以上が分かれている)を仮定  
    /// `w <= zero` のときは `0` を返す
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_sum_get() {
        const SIZE: usize = 1000;
        let mut rng = thread_rng();
        let mut ft = FenwickTree::new(SIZE, 0_i64);
        let mut list = vec![0; SIZE];
        const MIN: i64 = -1000_000_000;
        const MAX: i64 = 1000_000_000;

        for id in 0..SIZE {
            let add = rng.gen_range(MIN..=MAX);
            ft.add(id, add);
            list[id] += add;
        }

        for _ in 0..SIZE {
            let idx = rng.gen_range(0..SIZE);
            let add = rng.gen_range(MIN..=MAX);
            ft.add(idx, add);
            list[idx] += add;

            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left..=SIZE);
            let sum = list[left..right].iter().sum::<i64>();
            assert_eq!(ft.sum(left..right), sum);
        }

        for id in 0..SIZE {
            assert_eq!(ft.get(id), list[id]);
        }
    }

    #[test]
    fn test_lower_bound() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MAX: i64 = 10;
        let mut ft = FenwickTree::new(SIZE, 0_i64);
        let mut list = vec![0; SIZE];
        for id in 0..SIZE {
            let add = rng.gen_range(0..=MAX);
            ft.add(id, add);
            list[id] += add;
        }
        for _ in 0..SIZE {
            let id = rng.gen_range(0..SIZE);
            let add = rng.gen_range(0..=MAX);
            ft.add(id, add);
            list[id] += add;

            let lower_bound = rng.gen_range(1..list.iter().sum::<i64>());
            let id = ft.lower_bound(lower_bound);
            let sum = list[..id].iter().sum::<i64>();
            assert!(sum < lower_bound);
            assert!(sum + list[id] >= lower_bound);

            let lower_bound = list.iter().sum::<i64>() + 1;
            let id = ft.lower_bound(lower_bound);
            assert_eq!(id, SIZE);
        }
    }
}
