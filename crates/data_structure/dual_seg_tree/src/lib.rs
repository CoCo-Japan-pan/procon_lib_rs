//! 作用素を通常のセグメント木のように持つ
//! 作用が可換なら作用の伝播をしなくてOK

pub use algebra::CommutativeMap;
use std::ops::RangeBounds;

/// 可換な作用を区間適用, 1点取得(その点への作用の合成の取得)ができるデータ構造
pub struct DualSegTree<T: CommutativeMap> {
    lazy_nodes: Vec<T>,
    leaf_size: usize,
    range_size: usize,
}

impl<T: CommutativeMap> DualSegTree<T> {
    pub fn new(size: usize) -> Self {
        let mut leaf_size = 1;
        while leaf_size < size {
            leaf_size *= 2;
        }
        Self {
            lazy_nodes: vec![T::id(); 2 * leaf_size],
            leaf_size,
            range_size: size,
        }
    }

    /// 区間に作用を適用する
    pub fn apply<R: RangeBounds<usize>>(&mut self, range: R, map: &T) {
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
        l += self.leaf_size;
        r += self.leaf_size;
        while l < r {
            if l & 1 == 1 {
                self.lazy_nodes[l] = T::compostion(&self.lazy_nodes[l], map);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.lazy_nodes[r] = T::compostion(&self.lazy_nodes[r], map);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    /// 一点取得 その点への作用の合成を返す
    pub fn get(&self, i: usize) -> T {
        assert!(i < self.range_size);
        let mut i = i + self.leaf_size;
        let mut res = T::id();
        while i > 0 {
            res = T::compostion(&res, &self.lazy_nodes[i]);
            i >>= 1;
        }
        res
    }
}
