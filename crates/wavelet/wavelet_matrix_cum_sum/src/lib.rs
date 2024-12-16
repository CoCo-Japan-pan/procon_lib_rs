//! Wavelet Matrix の索引として、ビットごとの累積和を追加することで、重み付きの点の静的な矩形区間和を高速に求めることができる。  
//! 数列の区間`[l, r)`のうちの`x <= c < y`を満たす数値の和を求める`range_sum`クエリは、各点の重み`weight_list`を
//! (圧縮前の)y座標と同じものとすることで、矩形和のクエリに帰着できる。

use bitdict::BitDict;
use internal_bits::ceil_log2;
use internal_type_traits::Integral;
use std::ops::RangeBounds;

/// 座標圧縮とx座標の重複除去を行うWrapper  
/// `T`は座標圧縮の対象となる型、`U`は重みの型  
/// 単に矩形和クエリを扱うだけならこちらを使うことを推奨
pub struct WMCumSumWrapper<T: Integral, U: Integral> {
    wm: WaveletMatrixCumSum<U>,
    x_y: Vec<(T, T)>,
    sorted_y: Vec<T>,
}

impl<T: Integral, U: Integral> WMCumSumWrapper<T, U> {
    /// init_weightsは点の座標と重みのリスト `(x, y, w)`
    pub fn new(mut init_weights: Vec<(T, T, U)>) -> Self {
        init_weights.sort_unstable();
        let init_weights = init_weights
            .into_iter()
            .map(|(x, y, w)| ((x, y), w))
            .collect::<Vec<_>>();
        let (x_y, w): (Vec<(T, T)>, Vec<U>) = init_weights.into_iter().unzip();
        let y = x_y.iter().map(|&(_, y)| y).collect::<Vec<_>>();
        let sorted_y = {
            let mut sorted_y = y.clone();
            sorted_y.sort_unstable();
            sorted_y.dedup();
            sorted_y
        };
        let y = y
            .into_iter()
            .map(|y| sorted_y.binary_search(&y).unwrap())
            .collect::<Vec<_>>();
        let wm = WaveletMatrixCumSum::new(&y, &w);
        Self { wm, x_y, sorted_y }
    }

    /// 点(x, y)の重みを取得する
    pub fn get(&self, x: T, y: T) -> U {
        let Ok(id) = self.x_y.binary_search(&(x, y)) else {
            return U::zero();
        };
        self.wm.get_weight(id)
    }

    fn get_x_range<R: RangeBounds<T>>(&self, x_range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match x_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let r = match x_range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        let l = self.x_y.partition_point(|&(x, _)| x < l);
        let r = self.x_y.partition_point(|&(x, _)| x < r);
        (l, r)
    }

    fn get_y_range<R: RangeBounds<T>>(&self, y_range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let l = match y_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => T::min_value(),
        };
        let r = match y_range.end_bound() {
            Included(&r) => r + T::one(),
            Excluded(&r) => r,
            Unbounded => T::max_value(),
        };
        let l = self.sorted_y.partition_point(|&y| y < l);
        let r = self.sorted_y.partition_point(|&y| y < r);
        (l, r)
    }

    /// 矩形区間和内の点の重みの和を求める
    pub fn rect_sum<R1: RangeBounds<T>, R2: RangeBounds<T>>(&self, x_range: R1, y_range: R2) -> U {
        let (xl, xr) = self.get_x_range(x_range);
        let (yl, yr) = self.get_y_range(y_range);
        self.wm.rect_sum(xl..xr, yl..yr)
    }
}

/// Tは重さの型  
/// Wavelet Matrix にビットごとの累積和を追加したもの
#[derive(Debug, Clone)]
pub struct WaveletMatrixCumSum<T: Integral> {
    upper_bound: usize,
    len: usize,
    /// indices[i] = 下からiビット目に関する索引
    indices: Vec<BitDict>,
    /// ビットごとの累積和
    cumsum_per_bit: Vec<Vec<T>>,
}

impl<T: Integral> WaveletMatrixCumSum<T> {
    /// `compressed_list[x] = y` が点(x, y)に、`weight_list[x] = w` が点(x, y)の重みwに対応する  
    /// compressed_listは座標圧縮されていることを期待する  
    /// xは重複不可なので、順番を振りなおしてもらうことになる  
    /// 全て0以上
    pub fn new(compressed_list: &[usize], weight_list: &[T]) -> Self {
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
        let mut cum_sum = vec![vec![T::zero(); len + 1]; log];
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
            for (i, &w) in weight_list.iter().enumerate() {
                cum_sum[ln][i + 1] = cum_sum[ln][i] + w;
            }
        }
        Self {
            upper_bound,
            len,
            indices,
            cumsum_per_bit: cum_sum,
        }
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
    pub fn prefix_rect_sum<R: RangeBounds<usize>>(&self, x_range: R, upper: usize) -> T {
        if upper == 0 {
            return T::zero();
        }
        let (mut begin, mut end) = self.get_pos_range(x_range);
        let mut ret = T::zero();
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (upper >> ln) & 1;
            let rank1_begin = index.rank_1(begin);
            let rank1_end = index.rank_1(end);
            let rank0_begin = begin - rank1_begin;
            let rank0_end = end - rank1_end;
            if bit == 1 {
                ret += self.cumsum_per_bit[ln][rank0_end] - self.cumsum_per_bit[ln][rank0_begin];
                begin = index.rank0_all() + rank1_begin;
                end = index.rank0_all() + rank1_end;
            } else {
                begin = rank0_begin;
                end = rank0_end;
            }
        }
        ret
    }

    /// 矩形区間和内の点の重みの和を求める
    pub fn rect_sum<R1: RangeBounds<usize> + Clone, R2: RangeBounds<usize>>(
        &self,
        x_range: R1,
        y_range: R2,
    ) -> T {
        let (begin, end) = self.get_num_range(y_range);
        self.prefix_rect_sum(x_range.clone(), end) - self.prefix_rect_sum(x_range, begin)
    }

    pub fn get_weight(&self, x: usize) -> T {
        assert!(x < self.len);
        let index = self.indices.last().unwrap();
        let x = if index.access(x) {
            index.rank0_all() + index.rank_1(x)
        } else {
            index.rank_0(x)
        };
        let cumsum = self.cumsum_per_bit.last().unwrap();
        cumsum[x + 1] - cumsum[x]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_rect_sum() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        let weight_list = (0..SIZE)
            .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))
            .collect::<Vec<i64>>();
        let y_list = (0..SIZE)
            .map(|_| rng.gen_range(0..=SIZE))
            .collect::<Vec<usize>>();
        let wm = WaveletMatrixCumSum::new(&y_list, &weight_list);
        for id in 0..SIZE {
            assert_eq!(wm.get_weight(id), weight_list[id]);
        }
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
            assert_eq!(wm.rect_sum(x_left..x_right, y_left..y_right), real_sum);
        }
    }

    #[test]
    fn test_two_beki() {
        let mut rng = thread_rng();
        const SIZE: usize = 128;
        let y_list = [127; SIZE];
        let weight_list = (0..SIZE)
            .map(|_| rng.gen_range(0..=1000_000_000))
            .collect::<Vec<u64>>();
        let wm = WaveletMatrixCumSum::new(&y_list, &weight_list);
        for id in 0..SIZE {
            assert_eq!(wm.get_weight(id), weight_list[id]);
        }
        for _ in 0..1000 {
            let x_left = rng.gen_range(0..=SIZE);
            let x_right = rng.gen_range(x_left..=SIZE);
            let y_left = rng.gen_range(0..=SIZE);
            let y_right = rng.gen_range(SIZE..=SIZE * 10);
            let real_sum = weight_list
                .iter()
                .enumerate()
                .skip(x_left)
                .take(x_right - x_left)
                .filter(|&(i, _)| y_left <= y_list[i] && y_list[i] < y_right)
                .map(|(_, &w)| w)
                .sum::<u64>();
            assert_eq!(wm.rect_sum(x_left..x_right, y_left..y_right), real_sum);
        }
    }
}
