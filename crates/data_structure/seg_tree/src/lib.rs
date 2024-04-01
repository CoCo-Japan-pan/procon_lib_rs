//! ac_library_rsと同じです。

use algebra::Monoid;
use std::ops::RangeBounds;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegTree<M: Monoid> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    data: Vec<M::Target>,
}

impl<M: Monoid> From<Vec<M::Target>> for SegTree<M> {
    fn from(v: Vec<M::Target>) -> Self {
        let range_size = v.len();
        let log = (32 - (range_size as u32).saturating_sub(1).leading_zeros()) as usize;
        let leaf_size = 1 << log;
        let mut data = vec![M::id_element(); leaf_size * 2];
        data[leaf_size..leaf_size + range_size].clone_from_slice(&v);
        let mut seg_tree = SegTree {
            range_size,
            leaf_size,
            log,
            data,
        };
        for i in (1..leaf_size).rev() {
            seg_tree.update(i);
        }
        seg_tree
    }
}

impl<M: Monoid> SegTree<M> {
    pub fn new(n: usize) -> Self {
        vec![M::id_element(); n].into()
    }

    pub fn set(&mut self, mut p: usize, x: M::Target) {
        assert!(p < self.range_size);
        p += self.leaf_size;
        self.data[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> M::Target {
        assert!(p < self.range_size);
        self.data[p + self.leaf_size].clone()
    }

    pub fn prod<R: RangeBounds<usize>>(&self, range: R) -> M::Target {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        if l == 0 && r == self.range_size {
            return self.all_prod();
        }

        l += self.leaf_size;
        r += self.leaf_size;
        let mut sml = M::id_element();
        let mut smr = M::id_element();
        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> M::Target {
        self.data[1].clone()
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::Target) -> bool,
    {
        assert!(l <= self.range_size);
        assert!(f(&M::id_element()));
        if l == self.range_size {
            return self.range_size;
        }
        l += self.leaf_size;
        let mut sm = M::id_element();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, &self.data[l])) {
                while l < self.leaf_size {
                    l >>= 1;
                    let res = M::binary_operation(&sm, &self.data[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.leaf_size;
            }
            sm = M::binary_operation(&sm, &self.data[l]);
            l += 1;
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.range_size
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::Target) -> bool,
    {
        assert!(r <= self.range_size);
        assert!(f(&M::id_element()));
        if r == 0 {
            return 0;
        }
        r += self.leaf_size;
        let mut sm = M::id_element();
        while {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !f(&M::binary_operation(&self.data[r], &sm)) {
                while r < self.leaf_size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(&self.data[r], &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.leaf_size;
            }
            sm = M::binary_operation(&self.data[r], &sm);
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }
}

impl<M: Monoid> SegTree<M> {
    fn update(&mut self, k: usize) {
        self.data[k] = M::binary_operation(&self.data[k * 2], &self.data[k * 2 + 1]);
    }
}
