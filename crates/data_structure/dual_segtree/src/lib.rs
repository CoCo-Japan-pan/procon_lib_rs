//! 作用素を通常のセグメント木のように持つ  
//! 作用が可換なら作用の伝播をしなくてOK  
//! 作用が可換でないなら作用の伝播をしてから適用する

use algebra::{Action, Commutative};
use std::ops::{Bound::*, RangeBounds};

/// 作用を区間適用, 1点取得ができるデータ構造
#[derive(Debug)]
pub struct DualSegTree<T: Action> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    lazy_nodes: Vec<T>,
}

impl<T: Action> DualSegTree<T> {
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
            lazy_nodes: vec![T::id_action(); 2 * leaf_size],
        }
    }

    /// 一点取得(その点への作用を適用した結果を返す)
    pub fn get_mapped(&self, i: usize, mut target: T::Target) -> T::Target {
        assert!(i < self.range_size);
        let mut i = i + self.leaf_size;
        while i > 0 {
            self.lazy_nodes[i].apply(&mut target);
            i >>= 1;
        }
        target
    }

    /// 一点取得(その点への作用の合成を返す)
    pub fn get_composition(&self, i: usize) -> T {
        assert!(i < self.range_size);
        let mut i = i + self.leaf_size;
        let mut res = T::id_action();
        while i > 0 {
            res.composition(&self.lazy_nodes[i]);
            i >>= 1;
        }
        res
    }

    /// 可換性を仮定しない作用の区間適用  
    /// 可換な作用は`apply_range_commutative`の方が定数倍高速
    pub fn apply_range<R: RangeBounds<usize>>(&mut self, range: R, map: &T) {
        let mut l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
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
        let mut parent = T::id_action();
        std::mem::swap(&mut parent, &mut self.lazy_nodes[i]);
        self.lazy_nodes[i * 2].composition(&parent);
        self.lazy_nodes[i * 2 + 1].composition(&parent);
    }
}

impl<T: Action + Commutative> DualSegTree<T> {
    /// 区間に可換な作用を適用する 可換なので作用の伝播をしなくてOK
    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut self, range: R, map: &T) {
        let mut l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
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
