//! Wavelet Matrix の索引として、ビットごとの累積和を載せることで、重み付きの点の矩形区間和を高速に求めることができる。  
//! 数列の区間`[l, r)`のうちの`x <= c < y`を満たす数値の和を求める`range_sum`クエリは、各点の重みを
//! y座標と同じものとすることで、矩形和のクエリに帰着できる。

use bitvec::BitVec;
use internal_bits::ceil_log2;
use internal_type_traits::Integral;
use std::ops::RangeBounds;

/// Tは重さの型
#[derive(Debug, Clone)]
pub struct WaveletMatrixRectSum<T: Integral> {
    max: usize,
    len: usize,
    /// indices[i] = 下からiビット目に関する索引
    indices: Vec<BitVec>,
    /// ビットごとの累積和
    cum_sum: Vec<Vec<T>>,
    /// 普通の累積和 maxより大きいときはこちらを使う
    raw_cum_sum: Vec<T>,
}

impl<T: Integral> WaveletMatrixRectSum<T> {
    /// `compressed_list[x] = y` が点(x, y)に、`weight_list[x] = w` が点(x, y)の重みwに対応する  
    /// compressed_listは座標圧縮されていることを期待する  
    /// xは重複不可なので、順番を振りなおしてもらうことになる  
    /// 全て0以上
    pub fn new(compressed_list: &[usize], weight_list: &[T]) -> Self {
        let len = compressed_list.len();
        let mut raw_cum_sum = vec![T::zero(); len + 1];
        for (i, &w) in weight_list.iter().enumerate() {
            raw_cum_sum[i + 1] = raw_cum_sum[i] + w;
        }
        let max = *compressed_list.iter().max().unwrap_or(&0);
        let log = ceil_log2(max as u32 + 1) as usize;
        let mut indices = vec![BitVec::new(len); log];
        // 注目する桁のbitが0となる数、1となる数
        let mut tmp = vec![Vec::with_capacity(len); 2];
        let mut list = compressed_list.to_vec();
        let mut weight_list = weight_list.to_vec();
        let mut tmp_weight = vec![Vec::with_capacity(len); 2];
        let mut cum_sum = vec![vec![T::zero(); len + 1]; log];
        for (ln, index) in indices.iter_mut().enumerate().rev() {
            for (x, (&y, &w)) in list.iter().zip(weight_list.iter()).enumerate() {
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
            list.clear();
            weight_list.clear();
            list.append(&mut tmp[0]);
            list.append(&mut tmp[1]);
            weight_list.append(&mut tmp_weight[0]);
            weight_list.append(&mut tmp_weight[1]);
            for (i, &w) in weight_list.iter().enumerate() {
                cum_sum[ln][i + 1] = cum_sum[ln][i] + w;
            }
        }
        Self {
            max,
            len,
            indices,
            cum_sum,
            raw_cum_sum,
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
        .min(self.max + 1);
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.max + 1,
        }
        .min(self.max + 1);
        assert!(l <= r);
        (l, r)
    }

    /// x座標がx_range内、y座標はupper未満の点の重みの和を求める
    fn rect_sum_sub<R: RangeBounds<usize>>(&self, x_range: R, upper: usize) -> T {
        if upper == 0 {
            return T::zero();
        }
        let (mut begin, mut end) = self.get_pos_range(x_range);
        // 例外処理!!! 普通の累積和を使う
        if upper > self.max {
            return self.raw_cum_sum[end] - self.raw_cum_sum[begin];
        }
        let mut ret = T::zero();
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (upper >> ln) & 1;
            let rank1_begin = index.rank_1(begin);
            let rank1_end = index.rank_1(end);
            let rank0_begin = begin - rank1_begin;
            let rank0_end = end - rank1_end;
            if bit == 1 {
                ret += self.cum_sum[ln][rank0_end] - self.cum_sum[ln][rank0_begin];
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
        self.rect_sum_sub(x_range.clone(), end) - self.rect_sum_sub(x_range, begin)
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
        let wm = WaveletMatrixRectSum::new(&y_list, &weight_list);
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
        let wm = WaveletMatrixRectSum::new(&y_list, &weight_list);
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
