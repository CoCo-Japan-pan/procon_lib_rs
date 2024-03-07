//! 作用素を通常のセグメント木のように持つ  
//! 作用が可換なら作用の伝播をしなくてOK  
//! 作用が可換でないなら作用の伝播をしてから適用する

use algebra::{CommutativeMap, Map, NonCommutativeMap};
use std::ops::RangeBounds;

/// 作用を区間適用, 1点取得ができるデータ構造
pub struct DualSegTree<T: Map> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    lazy_nodes: Vec<T>,
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
            range_size: size,
            leaf_size,
            log,
            lazy_nodes: vec![T::id_map(); 2 * leaf_size],
        }
    }

    /// 一点取得(その点への作用を適用した結果を返す)
    pub fn get_mapped(&self, i: usize, mut target: T::Target) -> T::Target {
        assert!(i < self.range_size);
        let mut i = i + self.leaf_size;
        while i > 0 {
            self.lazy_nodes[i].mapping(&mut target);
            i >>= 1;
        }
        target
    }

    /// 一点取得(その点への作用の合成を返す)
    pub fn get_composition(&self, i: usize) -> T {
        assert!(i < self.range_size);
        let mut i = i + self.leaf_size;
        let mut res = T::id_map();
        while i > 0 {
            res.composition(&self.lazy_nodes[i]);
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
        if l == r {
            return;
        }
        l += self.leaf_size;
        r += self.leaf_size;
        while l < r {
            if l & 1 == 1 {
                self.lazy_nodes[l].composition(map);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.lazy_nodes[r].composition(map);
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
                self.lazy_nodes[l].composition(map);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.lazy_nodes[r].composition(map);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    fn propagate(&mut self, i: usize) {
        // 親ノードから子ノードへの作用の伝播
        let mut parent = T::id_map();
        std::mem::swap(&mut parent, &mut self.lazy_nodes[i]);
        self.lazy_nodes[i * 2].composition(&parent);
        self.lazy_nodes[i * 2 + 1].composition(&parent);
    }
}
