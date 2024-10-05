//! [Wavelet Matrix](https://miti-7.hatenablog.com/entry/2018/04/28/152259)

use bitvec::BitVec;
use internal_bits::ceil_log2;
use std::ops::RangeBounds;

/// 0以上の静的な数列を扱う  
/// 数値の種類数をσとして、様々な操作をO(logσ)で行える  
/// 追加で累積和をビットごとに持てば、range_sumもO(logσ)で求められる
/// 0-based
#[derive(Debug, Clone)]
pub struct WaveletMatrix {
    upper_bound: usize,
    len: usize,
    /// indices[i] = 下からiビット目に関する索引
    indices: Vec<BitVec>,
    /// ソートされた最終的な数列の要素の開始位置
    sorted_positions: Vec<Option<usize>>,
    /// 各数値の個数 selectで不正な操作を防ぐため
    counts: Vec<usize>,
}

impl WaveletMatrix {
    /// 0以上の数列を受け取り、WaveletMatrixを構築する O(nlogσ)  
    /// 最大値のlogだけ段数が必要なので、座標圧縮されていることを期待する
    pub fn new(compressed_list: &[usize]) -> Self {
        let len = compressed_list.len();
        let upper_bound = *compressed_list.iter().max().unwrap_or(&0) + 1;
        let log = ceil_log2(upper_bound as u32 + 1) as usize;
        let mut indices = vec![BitVec::new(len); log];
        // 注目する桁のbitが0となる数、1となる数
        let mut tmp = vec![Vec::with_capacity(len); 2];
        let mut list = compressed_list.to_vec();
        for (ln, index) in indices.iter_mut().enumerate().rev() {
            for (i, x) in list.drain(..).enumerate() {
                if (x >> ln) & 1 == 1 {
                    index.set(i);
                    tmp[1].push(x);
                } else {
                    tmp[0].push(x);
                }
            }
            index.build();
            list.append(&mut tmp[0]);
            list.append(&mut tmp[1]);
        }
        let mut sorted_positions = vec![None; upper_bound + 1];
        let mut counts = vec![0; upper_bound + 1];
        for (i, &x) in list.iter().enumerate() {
            if sorted_positions[x].is_none() {
                sorted_positions[x] = Some(i);
            }
            counts[x] += 1;
        }
        Self {
            upper_bound,
            len,
            indices,
            sorted_positions,
            counts,
        }
    }

    /// 数列のpos番目の要素を復元する O(logσ)
    pub fn access(&self, mut pos: usize) -> usize {
        assert!(pos < self.len);
        let mut ret = 0;
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = index.access(pos);
            if bit {
                ret |= 1 << ln;
                pos = index.rank0_all() + index.rank_1(pos);
            } else {
                pos = index.rank_0(pos);
            }
        }
        ret
    }

    /// 数列の[0, pos)区間に含まれるnumの数を求める O(logσ)
    pub fn rank(&self, num: usize, mut pos: usize) -> usize {
        if self.sorted_positions.get(num).unwrap_or(&None).is_none() {
            return 0;
        }
        assert!(pos <= self.len);
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (num >> ln) & 1;
            if bit == 1 {
                pos = index.rank0_all() + index.rank_1(pos);
            } else {
                pos = index.rank_0(pos);
            }
        }
        pos - self.sorted_positions[num].unwrap()
    }

    /// 数列の区間rangeのうち、numより小さい数の個数、numと等しい数の個数、numより大きい数の個数を求める O(logσ)
    pub fn rank_less_eq_more<R: RangeBounds<usize>>(
        &self,
        num: usize,
        range: R,
    ) -> (usize, usize, usize) {
        let (mut begin, mut end) = self.get_pos_range(range);
        let range_len = end - begin;
        if num >= self.upper_bound {
            return (range_len, 0, 0);
        }
        let mut less = 0;
        let mut more = 0;
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let bit = (num >> ln) & 1;
            let rank1_begin = index.rank_1(begin);
            let rank1_end = index.rank_1(end);
            let rank0_begin = begin - rank1_begin;
            let rank0_end = end - rank1_end;
            if bit == 1 {
                less += rank0_end - rank0_begin;
                begin = index.rank0_all() + rank1_begin;
                end = index.rank0_all() + rank1_end;
            } else {
                more += rank1_end - rank1_begin;
                begin = rank0_begin;
                end = rank0_end;
            }
        }
        let eq = range_len - less - more;
        (less, eq, more)
    }

    /// 数列のpos番目の数値numの位置を求める O(logσ)
    pub fn select(&self, num: usize, pos: usize) -> Option<usize> {
        if pos >= *self.counts.get(num)? {
            return None;
        }
        let mut ret = self.sorted_positions[num].unwrap() + pos;
        for (ln, index) in self.indices.iter().enumerate() {
            let bit = (num >> ln) & 1;
            if bit == 1 {
                ret = index.select_1(ret - index.rank0_all()).unwrap();
            } else {
                ret = index.select_0(ret).unwrap();
            }
        }
        Some(ret)
    }

    fn get_pos_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let left = match range.start_bound() {
            Included(&x) => x,
            Excluded(&x) => x + 1,
            Unbounded => 0,
        };
        let right = match range.end_bound() {
            Included(&x) => x + 1,
            Excluded(&x) => x,
            Unbounded => self.len,
        };
        assert!(left <= right && right <= self.len);
        (left, right)
    }

    /// 数列の区間rangeのうち、k番目に小さい値を返す O(logσ)
    pub fn quantile<R: RangeBounds<usize>>(&self, range: R, mut k: usize) -> usize {
        let (mut begin, mut end) = self.get_pos_range(range);
        assert!(k < end - begin);
        let mut ret = 0;
        for (ln, index) in self.indices.iter().enumerate().rev() {
            let one_cnt = index.rank_1(end) - index.rank_1(begin);
            let zero_cnt = end - begin - one_cnt;
            if k < zero_cnt {
                begin = index.rank_0(begin);
                end = index.rank_0(end);
            } else {
                ret |= 1 << ln;
                k -= zero_cnt;
                begin = index.rank0_all() + index.rank_1(begin);
                end = index.rank0_all() + index.rank_1(end);
            }
        }
        ret
    }

    fn get_num_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let left = match range.start_bound() {
            Included(&x) => x,
            Excluded(&x) => x + 1,
            Unbounded => 0,
        }
        .min(self.upper_bound);
        let right = match range.end_bound() {
            Included(&x) => x + 1,
            Excluded(&x) => x,
            Unbounded => self.upper_bound,
        }
        .min(self.upper_bound);
        assert!(left <= right);
        (left, right)
    }

    /// 数列の区間pos_rangeのうち、num_rangeに含まれる数の個数を求める O(logσ)
    pub fn range_freq<R1: RangeBounds<usize> + Clone, R2: RangeBounds<usize>>(
        &self,
        pos_range: R1,
        num_range: R2,
    ) -> usize {
        let (num_begin, num_end) = self.get_num_range(num_range);
        if num_begin >= num_end {
            return 0;
        }
        let cnt_begin = self.rank_less_eq_more(num_begin, pos_range.clone()).0;
        let cnt_end = self.rank_less_eq_more(num_end, pos_range).0;
        cnt_end - cnt_begin
    }

    /// 数列の区間rangeのうち、lower以上の値の中で最小の値を求める O(logσ)
    pub fn next_value<R: RangeBounds<usize> + Clone>(
        &self,
        range: R,
        lower: usize,
    ) -> Option<usize> {
        let (less, eq, upper) = self.rank_less_eq_more(lower, range.clone());
        if eq > 0 {
            return Some(lower);
        }
        if upper == 0 {
            return None;
        }
        Some(self.quantile(range, less))
    }

    /// 数列の区間rangeのうち、upper未満の値の中で最大の値を求める O(logσ)
    pub fn prev_value<R: RangeBounds<usize> + Clone>(
        &self,
        range: R,
        upper: usize,
    ) -> Option<usize> {
        let less = self.rank_less_eq_more(upper, range.clone()).0;
        if less == 0 {
            return None;
        }
        Some(self.quantile(range, less - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_access() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for i in 0..SIZE {
            assert_eq!(wm.access(i), list[i]);
        }
    }

    #[test]
    fn test_rank() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for num in 0..=MAX + 10 {
            let pos = rng.gen_range(0..SIZE);
            let real_cnt = list.iter().take(pos).filter(|&&x| x == num).count();
            assert_eq!(wm.rank(num, pos), real_cnt);
        }
    }

    #[test]
    fn test_rank_less_eq_more() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left..SIZE);
            let num = rng.gen_range(0..=MAX * 2);
            let (less, eq, more) = wm.rank_less_eq_more(num, left..right);
            let real_less = list[left..right].iter().filter(|&&x| x < num).count();
            let real_eq = list[left..right].iter().filter(|&&x| x == num).count();
            let real_more = list[left..right].iter().filter(|&&x| x > num).count();
            assert_eq!(less, real_less);
            assert_eq!(eq, real_eq);
            assert_eq!(more, real_more);
        }
    }

    #[test]
    fn test_select() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for num in 0..=MAX + 10 {
            let pos = rng.gen_range(0..=SIZE / MAX);
            let real_pos = list
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == num)
                .nth(pos)
                .map(|(i, _)| i);
            assert_eq!(wm.select(num, pos), real_pos);
        }
    }

    #[test]
    fn test_quantile() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left + 1..=SIZE);
            let k = rng.gen_range(0..=right - left - 1);
            let mut sorted = list[left..right].to_vec();
            sorted.sort();
            assert_eq!(wm.quantile(left..right, k), sorted[k]);
        }
    }

    #[test]
    fn test_range_freq() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left..SIZE);
            let num_left = rng.gen_range(0..=MAX * 2);
            let num_right = rng.gen_range(num_left..=MAX * 2);
            let real_cnt = list[left..right]
                .iter()
                .filter(|&&x| num_left <= x && x < num_right)
                .count();
            assert_eq!(wm.range_freq(left..right, num_left..num_right), real_cnt);
        }
    }

    #[test]
    fn test_next_value() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left..SIZE);
            let lower = rng.gen_range(0..=MAX * 2);
            let mut sorted = list[left..right].to_vec();
            sorted.sort();
            let real = sorted.iter().filter(|&&x| x >= lower).next().copied();
            assert_eq!(wm.next_value(left..right, lower), real);
        }
    }

    #[test]
    fn test_prev_value() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 128;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(&list);
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left..SIZE);
            let upper = rng.gen_range(0..=MAX * 2);
            let mut sorted = list[left..right].to_vec();
            sorted.sort();
            let real = sorted.iter().filter(|&&x| x < upper).last().copied();
            assert_eq!(wm.prev_value(left..right, upper), real);
        }
    }
}
