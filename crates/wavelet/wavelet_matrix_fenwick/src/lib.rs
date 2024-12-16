//! Wavelet Matrix に Fenwick Tree (Binary Indexed Tree) を載せて、
//! 1点更新、矩形和クエリを処理する  
//! BITはSegment Treeより定数倍が軽いので、通常の意味での和を求める場合はこちらを推奨

use bitdict::BitDict;
use fenwick_tree::FenwickTree;
use internal_bits::ceil_log2;
use internal_type_traits::Integral;
use std::ops::RangeBounds;

/// 座標圧縮とx座標の重複除去を行うWrapper `T`が座標圧縮する型 `U`が重みの型
pub struct WMFenwickWrapper<T: Integral, U: Integral> {
    wm: WaveletMatrixFenwick<U>,
    sorted_y: Vec<T>,
    x_y: Vec<(T, T)>,
}

impl<T: Integral, U: Integral> WMFenwickWrapper<T, U> {
    /// すべて単位元で初期化する場合
    pub fn new(update_points: Vec<(T, T)>) -> Self {
        Self::from_weight(update_points, &[])
    }

    /// update_pointsは更新クエリのある点の座標のリスト ただしinit_weightsの点も含める  
    /// init_weightsは初期状態の点の座標と重みのリスト (x, y, w)  
    /// もしinit_weightsの点が重複する場合は、それらmonoidの積として初期化するので注意(上書きしたい場合は事前に重複を消す前処理をしてください)
    pub fn from_weight(mut update_points: Vec<(T, T)>, init_weights: &[(T, T, U)]) -> Self {
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
        let mut weight_list = vec![U::zero(); update_points.len()];
        for (x, y, w) in init_weights {
            let idx = update_points
                .binary_search(&(*x, *y))
                .expect("init_weight points are not in update_points!!!");
            weight_list[idx] += *w;
        }
        let wm = WaveletMatrixFenwick::<U>::from_weight(&compressed_list, &weight_list);
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

    /// 点(x, y)の重みをnew_valに更新する
    pub fn set(&mut self, x: T, y: T, new_val: U) {
        let x = self
            .x_y
            .binary_search(&(x, y))
            .expect("(x, y) is not in update_queries!!!");
        self.wm.set(x, new_val);
    }

    /// 点(x, y)の重みにadd_valを加算する
    pub fn add(&mut self, x: T, y: T, add_val: U) {
        let x = self
            .x_y
            .binary_search(&(x, y))
            .expect("(x, y) is not in update_queries!!!");
        self.wm.add(x, add_val);
    }

    /// 点(x, y)の重みを取得する
    pub fn get(&self, x: T, y: T) -> U {
        let Ok(x) = self.x_y.binary_search(&(x, y)) else {
            return U::zero();
        };
        self.wm.get_weight(x)
    }

    /// 矩形区間内の点の重みの和を求める
    pub fn rect_sum<R1: RangeBounds<T>, R2: RangeBounds<T>>(&self, x_range: R1, y_range: R2) -> U {
        let (xl, xr) = self.get_pos_range(x_range);
        let (y_low, y_hi) = self.get_num_range(y_range);
        self.wm.rect_sum(xl, xr, y_low, y_hi)
    }
}

struct WaveletMatrixFenwick<T: Integral> {
    len: usize,
    /// indices[i] = 下からiビット目に関する索引
    indices: Vec<BitDict>,
    /// ビットごとのFenwickTree
    fenwick_per_bit: Vec<FenwickTree<T>>,
}

impl<T: Integral> WaveletMatrixFenwick<T> {
    /// `compressed_list[x] = y` が点(x, y)に、`weight_list[x] = w` が点(x, y)の重みwに対応する  
    /// compressed_listには今後更新クエリのある(x, y)も含める  
    /// compressed_listは座標圧縮されていることを期待する  
    /// xは重複不可なので、順番を振りなおしてもらうことになる  
    /// 全て0以上
    fn from_weight(compressed_list: &[usize], weight_list: &[T]) -> Self {
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
        let mut fenwick_per_bit = Vec::with_capacity(log);
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
            let mut cur_fenwick = FenwickTree::new(len, T::zero());
            for (i, &w) in weight_list.iter().enumerate() {
                cur_fenwick.add(i, w);
            }
            fenwick_per_bit.push(cur_fenwick);
        }
        fenwick_per_bit.reverse();
        Self {
            len,
            indices,
            fenwick_per_bit,
        }
    }

    /// x座標が[begin, end)内、y座標はupper未満の点の重みの和を求める
    fn prefix_rect_sum(&self, mut begin: usize, mut end: usize, upper: usize) -> T {
        if upper == 0 {
            return T::zero();
        }
        let mut ret = T::zero();
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (upper >> ln) & 1;
            let rank1_begin = index.rank_1(begin);
            let rank1_end = index.rank_1(end);
            let rank0_begin = begin - rank1_begin;
            let rank0_end = end - rank1_end;
            if bit == 1 {
                ret += self.fenwick_per_bit[ln].sum(rank0_begin..rank0_end);
                begin = index.rank0_all() + rank1_begin;
                end = index.rank0_all() + rank1_end;
            } else {
                begin = rank0_begin;
                end = rank0_end;
            }
        }
        ret
    }

    fn rect_sum(&self, x_begin: usize, x_end: usize, y_begin: usize, y_end: usize) -> T {
        self.prefix_rect_sum(x_begin, x_end, y_end) - self.prefix_rect_sum(x_begin, x_end, y_begin)
    }

    fn add(&mut self, mut x: usize, add_val: T) {
        assert!(x < self.len);
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = index.access(x);
            if bit {
                x = index.rank0_all() + index.rank_1(x);
            } else {
                x = index.rank_0(x);
            }
            self.fenwick_per_bit[ln].add(x, add_val);
        }
    }

    fn set(&mut self, mut x: usize, new_val: T) {
        assert!(x < self.len);
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = index.access(x);
            if bit {
                x = index.rank0_all() + index.rank_1(x);
            } else {
                x = index.rank_0(x);
            }
            self.fenwick_per_bit[ln].set(x, new_val);
        }
    }

    fn get_weight(&self, x: usize) -> T {
        assert!(x < self.len);
        let index = self.indices.last().unwrap();
        let x = if index.access(x) {
            index.rank0_all() + index.rank_1(x)
        } else {
            index.rank_0(x)
        };
        self.fenwick_per_bit.last().unwrap().get(x)
    }
}
