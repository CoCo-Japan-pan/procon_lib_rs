//! 作用素を通常のセグメント木のように持つ  
//! 作用が可換なら作用の伝播をしなくてOK  
//! 作用が可換でないなら作用の伝播をしてから適用する

use algebra::{CommutativeMap, Map, NonCommutativeMap};
use std::ops::RangeBounds;

/// 作用を区間適用, 1点取得(その点への作用の合成の取得)ができるデータ構造
pub struct DualSegTree<T: Map> {
    lazy_nodes: Vec<T>,
    leaf_size: usize,
    range_size: usize,
    log: usize,
}

impl<T: Map> DualSegTree<T> {
    pub fn new(size: usize) -> Self {
        let mut leaf_size = 1;
        let mut log = 0;
        while leaf_size < size {
            leaf_size *= 2;
            log += 1;
        }
        Self {
            lazy_nodes: vec![T::id(); 2 * leaf_size],
            leaf_size,
            range_size: size,
            log,
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

impl<T: CommutativeMap> DualSegTree<T> {
    /// 区間に可換な作用を適用する 可換なので作用の伝播をしなくてOK
    pub fn apply_commutative<R: RangeBounds<usize>>(&mut self, range: R, map: &T) {
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
}

impl<T: NonCommutativeMap> DualSegTree<T> {
    /// 区間に非可換な作用を適用する 非可換なので作用の伝播を先に行う必要がある
    pub fn apply_non_commutative<R: RangeBounds<usize>>(&mut self, range: R, map: &T) {
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
        if l == r {
            return;
        }
        l += self.leaf_size;
        r += self.leaf_size;
        // 両端の上にあるノードを先に伝播させておく 高々2log回
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.propagate(l >> i);
            }
            if ((r >> i) << i) != r {
                self.propagate((r - 1) >> i);
            }
        }
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

    fn propagate(&mut self, i: usize) {
        // 親ノードから子ノードへの作用の伝播
        self.lazy_nodes[i * 2] = T::compostion(&self.lazy_nodes[i * 2], &self.lazy_nodes[i]);
        self.lazy_nodes[i * 2 + 1] =
            T::compostion(&self.lazy_nodes[i * 2 + 1], &self.lazy_nodes[i]);
        self.lazy_nodes[i] = T::id();
    }
}
