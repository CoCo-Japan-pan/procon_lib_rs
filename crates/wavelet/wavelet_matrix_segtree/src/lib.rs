//! Wavelet Matrix に、ビットごとのSegment Treeを追加することで、
//! 二次元セグ木と同様に(オフラインな)1点更新、矩形和を求められる

use algebra::{Commutative, Group, Monoid};
use bitdict::BitDict;
use internal_bits::ceil_log2;
use internal_type_traits::Integral;
use segtree::SegTree;
use std::ops::RangeBounds;

/// 座標圧縮をするWrapper Tが座標圧縮する型
pub struct WMSegWrapper<M: Monoid + Commutative, T: Integral> {
    wm: WaveletMatrixSegTree<M>,
    sorted_y: Vec<T>,
    x_y: Vec<(T, T)>,
}

impl<M: Monoid + Commutative, T: Integral> WMSegWrapper<M, T> {
    pub fn new(update_points: Vec<(T, T)>) -> Self {
        Self::from_weight(update_points, vec![])
    }

    /// update_pointsは更新クエリのある点の座標のリスト ただしinit_weightsの点も含める  
    /// init_weightsは初期状態の点の座標と重みのリスト (x, y, w)  
    /// init_weightsは重複しないことを前提とする
    pub fn from_weight(
        mut update_points: Vec<(T, T)>,
        mut init_weights: Vec<(T, T, M::Target)>,
    ) -> Self {
        update_points.sort_unstable();
        update_points.dedup();
        let mut sorted_y = update_points
            .iter()
            .map(|(_, y)| y)
            .copied()
            .collect::<Vec<_>>();
        sorted_y.sort_unstable();
        let compressed_list = update_points
            .iter()
            .map(|(_, y)| sorted_y.binary_search(y).unwrap())
            .collect::<Vec<_>>();
        let mut weight_list = vec![M::id_element(); update_points.len()];
        init_weights.sort_unstable_by_key(|&(x, y, _)| (x, y));
        for ls in init_weights.windows(2) {
            let (x1, y1) = (ls[0].0, ls[0].1);
            let (x2, y2) = (ls[1].0, ls[1].1);
            assert_ne!((x1, y1), (x2, y2), "init_weights has duplicated points!!!");
        }
        for (x, y, w) in init_weights {
            let idx = update_points.binary_search(&(x, y)).unwrap();
            weight_list[idx] = w.clone();
        }
        let wm = WaveletMatrixSegTree::<M>::from_weight(&compressed_list, &weight_list);
        Self {
            wm,
            sorted_y,
            x_y: update_points,
        }
    }

    fn get_pos_range<R: RangeBounds<T>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let r = match range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        assert!(l <= r);
        let l = self.x_y.partition_point(|&(x, _)| x < l);
        let r = self.x_y.partition_point(|&(x, _)| x < r);
        (l, r)
    }

    fn get_num_range<R: RangeBounds<T>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let r = match range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        assert!(l <= r);
        let l = self.sorted_y.partition_point(|&y| y < l);
        let r = self.sorted_y.partition_point(|&y| y < r);
        (l, r)
    }

    pub fn set(&mut self, x: T, y: T, new_val: M::Target) {
        let x = self
            .x_y
            .binary_search(&(x, y))
            .expect("(x, y) is not in update_queries!!!");
        self.wm.set(x, new_val);
    }

    pub fn get(&self, x: T, y: T) -> M::Target {
        let Ok(x) = self.x_y.binary_search(&(x, y)) else {
            return M::id_element();
        };
        self.wm.get_weight(x)
    }

    pub fn rect_sum_monoid<R1: RangeBounds<T>, R2: RangeBounds<T>>(
        &self,
        x_range: R1,
        y_range: R2,
    ) -> M::Target {
        let (xl, xr) = self.get_pos_range(x_range);
        let (y_low, y_hi) = self.get_num_range(y_range);
        self.wm.rect_sum_monoid(xl..xr, y_low..y_hi)
    }

    pub fn rect_sum_group<R1: RangeBounds<T>, R2: RangeBounds<T>>(
        &self,
        x_range: R1,
        y_range: R2,
    ) -> M::Target
    where
        M: Group,
    {
        let (xl, xr) = self.get_pos_range(x_range);
        let (y_low, y_hi) = self.get_num_range(y_range);
        self.wm.rect_sum_group(xl..xr, y_low..y_hi)
    }
}

pub struct WaveletMatrixSegTree<M: Monoid + Commutative> {
    upper_bound: usize,
    len: usize,
    /// indices[i] = 下からiビット目に関する索引
    indices: Vec<BitDict>,
    /// ビットごとのSegTree
    segtree_per_bit: Vec<SegTree<M>>,
}

impl<M: Monoid + Commutative> WaveletMatrixSegTree<M> {
    /// `compressed_list[x] = y` が点(x, y)に、`weight_list[x] = w` が点(x, y)の重みwに対応する  
    /// compressed_listには今後更新クエリのある(x, y)も含める  
    /// compressed_listは座標圧縮されていることを期待する  
    /// xは重複不可なので、順番を振りなおしてもらうことになる  
    /// 全て0以上
    pub fn from_weight(compressed_list: &[usize], weight_list: &[M::Target]) -> Self {
        assert_eq!(compressed_list.len(), weight_list.len());
        let len = compressed_list.len();
        let upper_bound = *compressed_list.iter().max().unwrap_or(&0) + 1;
        let log = ceil_log2(upper_bound as u32 + 1) as usize;
        let mut indices = vec![BitDict::new(len); log];
        // 注目する桁のbitが0となる数、1となる数
        let mut tmp = vec![Vec::with_capacity(len); 2];
        let mut list = compressed_list.to_vec();
        let mut weight_list = weight_list.to_vec();
        let mut tmp_weight = vec![Vec::with_capacity(len); 2];
        let mut segtree_per_bit = Vec::with_capacity(log);
        for (ln, index) in indices.iter_mut().enumerate().rev() {
            for (x, (y, w)) in list.drain(..).zip(weight_list.drain(..)).enumerate() {
                if (y >> ln) & 1 == 1 {
                    index.set(x);
                    tmp[1].push(y);
                    tmp_weight[1].push(w);
                } else {
                    tmp[0].push(y);
                    tmp_weight[0].push(w);
                }
            }
            index.build();
            list.append(&mut tmp[0]);
            list.append(&mut tmp[1]);
            weight_list.append(&mut tmp_weight[0]);
            weight_list.append(&mut tmp_weight[1]);
            segtree_per_bit.push(SegTree::from(&weight_list));
        }
        segtree_per_bit.reverse();
        Self {
            upper_bound,
            len,
            indices,
            segtree_per_bit,
        }
    }

    /// `compressed_list[x] = y` が点(x, y)に対応し、重みは単位元で初期化する  
    /// `compressed_list`には今後更新クエリのある(x, y)も含める
    pub fn from_identity(compressed_list: &[usize]) -> Self {
        let weight_list = vec![M::id_element(); compressed_list.len()];
        Self::from_weight(compressed_list, &weight_list)
    }

    fn get_pos_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.len,
        };
        assert!(l <= r && r <= self.len);
        (l, r)
    }

    fn get_num_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        }
        .min(self.upper_bound);
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.upper_bound,
        }
        .min(self.upper_bound);
        assert!(l <= r);
        (l, r)
    }

    /// x座標がx_range内、y座標はupper未満の点の重みの和を求める
    pub fn prefix_rect_sum<R: RangeBounds<usize>>(&self, x_range: R, upper: usize) -> M::Target {
        if upper == 0 {
            return M::id_element();
        }
        let (mut begin, mut end) = self.get_pos_range(x_range);
        let mut ret = M::id_element();
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (upper >> ln) & 1;
            let rank1_begin = index.rank_1(begin);
            let rank1_end = index.rank_1(end);
            let rank0_begin = begin - rank1_begin;
            let rank0_end = end - rank1_end;
            if bit == 1 {
                ret = M::binary_operation(
                    &ret,
                    &self.segtree_per_bit[ln].prod(rank0_begin..rank0_end),
                );
                begin = index.rank0_all() + rank1_begin;
                end = index.rank0_all() + rank1_end;
            } else {
                begin = rank0_begin;
                end = rank0_end;
            }
        }
        ret
    }

    /// 群を重みとして載せている場合における、矩形区間和内の点の重みの和を求める  
    /// prefix_sumを二度求めて引く 非再帰なので定数倍が良いはず
    pub fn rect_sum_group<R1: RangeBounds<usize> + Clone, R2: RangeBounds<usize>>(
        &self,
        x_range: R1,
        y_range: R2,
    ) -> M::Target
    where
        M: Group,
    {
        let (begin, end) = self.get_num_range(y_range);
        let s2 = self.prefix_rect_sum(x_range.clone(), end);
        let s1 = self.prefix_rect_sum(x_range, begin);
        M::binary_operation(&M::inverse(&s1), &s2)
    }

    /// モノイドを重みとして載せている場合における、矩形区間和内の点の重みの和を求める  
    /// 完全に覆うか外れるかするまで再帰的に二冪の長さの区間に分けていく
    pub fn rect_sum_monoid<R1: RangeBounds<usize>, R2: RangeBounds<usize>>(
        &self,
        x_range: R1,
        y_range: R2,
    ) -> M::Target {
        let (xl, xr) = self.get_pos_range(x_range);
        let (y_low, y_hi) = self.get_num_range(y_range);
        let mut ret = M::id_element();
        let ln = self.indices.len();
        self.dfs(&mut ret, ln, xl, xr, 0, 1 << ln, y_low, y_hi);
        ret
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        &self,
        ret: &mut M::Target,
        ln: usize,
        xl: usize,
        xr: usize,
        yl: usize,
        yr: usize,
        y_low: usize,
        y_hi: usize,
    ) {
        assert_eq!(yr - yl, 1 << ln);
        if y_hi <= yl || yr <= y_low {
            return;
        }
        if y_low <= yl && yr <= y_hi {
            *ret = M::binary_operation(ret, &self.segtree_per_bit[ln].prod(xl..xr));
            return;
        }
        let ln = ln - 1;
        let rank1_xl = self.indices[ln].rank_1(xl);
        let rank1_xr = self.indices[ln].rank_1(xr);
        let rank0_all = self.indices[ln].rank0_all();
        let rank0_xl = xl - rank1_xl;
        let rank0_xr = xr - rank1_xr;
        let ymid = (yl + yr) / 2;
        self.dfs(ret, ln, rank0_xl, rank0_xr, yl, ymid, y_low, y_hi);
        self.dfs(
            ret,
            ln,
            rank0_all + rank1_xl,
            rank0_all + rank1_xr,
            ymid,
            yr,
            y_low,
            y_hi,
        );
    }

    pub fn set(&mut self, mut x: usize, new_val: M::Target) {
        assert!(x < self.len);
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = index.access(x);
            if bit {
                x = index.rank0_all() + index.rank_1(x);
            } else {
                x = index.rank_0(x);
            }
            self.segtree_per_bit[ln].set(x, new_val.clone());
        }
    }

    pub fn get_weight(&self, x: usize) -> M::Target {
        assert!(x < self.len);
        let index = self.indices.last().unwrap();
        let x = if index.access(x) {
            index.rank0_all() + index.rank_1(x)
        } else {
            index.rank_0(x)
        };
        self.segtree_per_bit.last().unwrap().get(x)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    // 加算群
    struct AddGroup;
    impl Monoid for AddGroup {
        type Target = i64;
        fn id_element() -> Self::Target {
            0
        }
        fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
            a + b
        }
    }
    impl Commutative for AddGroup {}
    impl Group for AddGroup {
        fn inverse(a: &Self::Target) -> Self::Target {
            -a
        }
    }

    #[test]
    fn test_static_rect_sum() {
        use wavelet_matrix_cum_sum::WaveletMatrixCumSum;
        let mut rng = thread_rng();
        const SIZE: usize = 100000;
        let num_list: Vec<usize> = (0..SIZE).map(|_| rng.gen_range(0..SIZE)).collect();
        let wm_cum_sum = WaveletMatrixCumSum::new(&num_list, &num_list);
        let num_list_i64: Vec<i64> = num_list.iter().map(|i| *i as i64).collect();
        let wm_seg = WaveletMatrixSegTree::<AddGroup>::from_weight(&num_list, &num_list_i64);

        for _ in 0..SIZE {
            let xl = rng.gen_range(0..SIZE);
            let xr = rng.gen_range(xl..SIZE);
            let yl = rng.gen_range(0..SIZE);
            let yr = rng.gen_range(yl..SIZE);
            let cum_sum_ans = wm_cum_sum.rect_sum(xl..xr, yl..yr) as i64;
            assert_eq!(cum_sum_ans, wm_seg.rect_sum_group(xl..xr, yl..yr));
            assert_eq!(cum_sum_ans, wm_seg.rect_sum_monoid(xl..xr, yl..yr))
        }
    }

    #[test]
    fn test_point_add_rect_sum() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        let mut weight_list = (0..SIZE)
            .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))
            .collect::<Vec<i64>>();
        let y_list = (0..SIZE)
            .map(|_| rng.gen_range(0..=SIZE))
            .collect::<Vec<usize>>();
        let mut wm = WaveletMatrixSegTree::<AddGroup>::from_weight(&y_list, &weight_list);
        for _ in 0..1000 {
            let x_left = rng.gen_range(0..=SIZE);
            let x_right = rng.gen_range(x_left..=SIZE);
            let y_left = rng.gen_range(0..=SIZE);
            let y_right = rng.gen_range(y_left..=SIZE);
            let real_sum = weight_list
                .iter()
                .enumerate()
                .skip(x_left)
                .take(x_right - x_left)
                .filter(|&(i, _)| y_left <= y_list[i] && y_list[i] < y_right)
                .map(|(_, &w)| w)
                .sum::<i64>();
            assert_eq!(
                wm.rect_sum_group(x_left..x_right, y_left..y_right),
                real_sum
            );
            assert_eq!(
                wm.rect_sum_monoid(x_left..x_right, y_left..y_right),
                real_sum
            );
            let pos = rng.gen_range(0..SIZE);
            let new_weight = rng.gen_range(-1000_000_000..=1000_000_000);
            weight_list[pos] = new_weight;
            wm.set(pos, new_weight);
        }
    }

    #[test]
    fn test_get_weight() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        let mut weight_list = (0..SIZE)
            .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))
            .collect::<Vec<i64>>();
        let y_list = (0..SIZE)
            .map(|_| rng.gen_range(0..=SIZE))
            .collect::<Vec<usize>>();
        let mut wm = WaveletMatrixSegTree::<AddGroup>::from_weight(&y_list, &weight_list);
        for _ in 0..1000 {
            for i in 0..1000 {
                assert_eq!(weight_list[i], wm.get_weight(i));
            }
            let pos = rng.gen_range(0..SIZE);
            let new_weight = rng.gen_range(-1000_000_000..=1000_000_000);
            weight_list[pos] = new_weight;
            wm.set(pos, new_weight);
        }
    }
}
