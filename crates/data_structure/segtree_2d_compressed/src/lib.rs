//! Sparseな場合に対応するため、更新queryを先読みして座標圧縮する  
//! もともと単位元で初期化されていると仮定する  
//! 2次元なので可換性を要求  
//! <https://drive.google.com/file/d/1bSjYiA-nSsHzBbCnLq1GeTpRzs2Ucm0q/view>で学びました  

use algebra::{Commutative, Monoid};
use internal_type_traits::Integral;
use segtree::SegTree;
use std::ops::{Bound::*, Range, RangeBounds};

/// Tは座標圧縮する型  
#[derive(Debug)]
pub struct SegTree2DCompressed<M: Monoid + Commutative, T: Integral> {
    height_compressed: Vec<T>,
    width_compressed: Vec<Vec<T>>,
    data: Vec<SegTree<M>>,
}

impl<M: Monoid + Commutative, T: Integral> SegTree2DCompressed<M, T> {
    /// オフラインで読み込んだ更新クエリのある座標を与え、単位元で初期化
    pub fn new(update_queries: &[(T, T)]) -> Self {
        let height_compressed = {
            let mut tmp = update_queries.iter().map(|&(h, _)| h).collect::<Vec<_>>();
            tmp.sort_unstable();
            tmp.dedup();
            tmp
        };
        let width_compressed = {
            let mut tmp = vec![vec![]; height_compressed.len() * 2];
            for &(h, w) in update_queries.iter() {
                let h = height_compressed.binary_search(&h).unwrap() + height_compressed.len();
                tmp[h].push(w);
            }
            for v in tmp.iter_mut() {
                v.sort_unstable();
                v.dedup();
            }
            for h in (1..height_compressed.len()).rev() {
                let child_left = tmp[h * 2].clone();
                let child_right = tmp[h * 2 + 1].clone();
                tmp[h] = child_left.into_iter().chain(child_right).collect();
                tmp[h].sort_unstable();
                tmp[h].dedup();
            }
            tmp
        };
        let data = (0..height_compressed.len() * 2)
            .map(|i| SegTree::<M>::new(width_compressed[i].len()))
            .collect();
        Self {
            height_compressed,
            width_compressed,
            data,
        }
    }

    pub fn get(&self, h: T, w: T) -> M::Target {
        if let Ok(h) = self.height_compressed.binary_search(&h) {
            let h = h + self.height_compressed.len();
            if let Ok(w) = self.width_compressed[h].binary_search(&w) {
                return self.data[h].get(w);
            }
        }
        M::id_element()
    }

    /// 更新の都合上、setよりも二項演算の適用の方が定数倍速い  
    pub fn add(&mut self, h: T, w: T, val: M::Target) {
        let mut h = self
            .height_compressed
            .binary_search(&h)
            .expect("h is not in update_queries");
        h += self.height_compressed.len();
        while h > 0 {
            let cur_w_id = self.width_compressed[h]
                .binary_search(&w)
                .expect("w is not in update_queries");
            let old_val = self.data[h].get(cur_w_id);
            self.data[h].set(cur_w_id, M::binary_operation(&old_val, &val));
            h >>= 1;
        }
    }

    #[allow(clippy::collapsible_else_if, clippy::redundant_clone)]
    pub fn set(&mut self, h: T, w: T, val: M::Target) {
        // setよりもaddのような差分での更新の方が楽にかけるかも
        let mut h = self
            .height_compressed
            .binary_search(&h)
            .expect("h is not in update_queries");
        h += self.height_compressed.len();
        let mut pre_h = 2 * h;
        let mut pre_val = val.clone();
        while h > 0 {
            let cur_w_id = self.width_compressed[h]
                .binary_search(&w)
                .expect("w is not in update_queries");
            if h >= self.height_compressed.len() {
                self.data[h].set(cur_w_id, val.clone());
            } else {
                let other_child = if pre_h == 2 * h {
                    if let Ok(w) = self.width_compressed[2 * h + 1].binary_search(&w) {
                        self.data[2 * h + 1].get(w)
                    } else {
                        M::id_element()
                    }
                } else {
                    if let Ok(w) = self.width_compressed[2 * h].binary_search(&w) {
                        self.data[2 * h].get(w)
                    } else {
                        M::id_element()
                    }
                };
                let new_val = M::binary_operation(&pre_val, &other_child);
                pre_val = new_val.clone();
                self.data[h].set(cur_w_id, new_val);
            }
            pre_h = h;
            h >>= 1;
        }
    }

    pub fn prod<R1: RangeBounds<T>, R2: RangeBounds<T>>(
        &self,
        height_range: R1,
        width_range: R2,
    ) -> M::Target {
        let height_left = match height_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let height_right = match height_range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        assert!(height_left <= height_right);
        let mut height_left = self.height_compressed.partition_point(|&h| h < height_left);
        let mut height_right = self
            .height_compressed
            .partition_point(|&h| h < height_right);
        height_left += self.height_compressed.len();
        height_right += self.height_compressed.len();
        let mut ret = M::id_element();
        while height_left < height_right {
            if height_left & 1 != 0 {
                let w_range = self.calc_row_range(height_left, &width_range);
                ret = M::binary_operation(&ret, &self.data[height_left].prod(w_range));
                height_left += 1;
            }
            if height_right & 1 != 0 {
                height_right -= 1;
                let w_range = self.calc_row_range(height_right, &width_range);
                ret = M::binary_operation(&ret, &self.data[height_right].prod(w_range));
            }
            height_left >>= 1;
            height_right >>= 1;
        }
        ret
    }

    fn calc_row_range<R1: RangeBounds<T>>(&self, h: usize, width_range: &R1) -> Range<usize> {
        let w_left = match width_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let w_right = match width_range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        let w_left = self.width_compressed[h].partition_point(|&w| w < w_left);
        let w_right = self.width_compressed[h].partition_point(|&w| w < w_right);
        w_left..w_right
    }
}
