//! [Wavelet Matrix](https://miti-7.hatenablog.com/entry/2018/04/28/152259)

use bitvec::BitVec;
use internal_bits::ceil_log2;
use std::ops::RangeBounds;

/// 0以上の静的な数列を扱う  
/// 数値の種類数をσとして、様々な操作をO(logσ)で行える  
/// 0-based
#[derive(Debug, Clone)]
pub struct WaveletMatrix {
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
    pub fn new(mut list: Vec<usize>) -> Self {
        let len = list.len();
        let max = *list.iter().max().unwrap();
        let log = ceil_log2(max as u32) as usize;
        let mut indices = vec![BitVec::new(len); log + 1];
        for (ln, index) in indices.iter_mut().enumerate().rev() {
            for (i, &x) in list.iter().enumerate() {
                if (x >> ln) & 1 == 1 {
                    index.set(i);
                }
            }
            index.build();
            list.sort_by_key(|&x| (x >> ln) & 1);
        }
        let mut sorted_positions = vec![None; max + 1];
        let mut counts = vec![0; max + 1];
        for (i, &x) in list.iter().enumerate() {
            if sorted_positions[x].is_none() {
                sorted_positions[x] = Some(i);
            }
            counts[x] += 1;
        }
        Self {
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
        if self.sorted_positions[num].is_none() {
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

    /// 数列のpos番目の数値numの位置を求める O(logσ)
    pub fn select(&self, num: usize, pos: usize) -> Option<usize> {
        if self.counts[num] <= pos {
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

    fn get_begin_end<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
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
        let (mut begin, mut end) = self.get_begin_end(range);
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
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_access() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 100;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(list.clone());
        for i in 0..SIZE {
            assert_eq!(wm.access(i), list[i]);
        }
    }

    #[test]
    fn test_rank() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 100;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(list.clone());
        for num in 0..=MAX {
            let pos = rng.gen_range(0..SIZE);
            let real_cnt = list.iter().take(pos).filter(|&&x| x == num).count();
            assert_eq!(wm.rank(num, pos), real_cnt);
        }
    }

    #[test]
    fn test_select() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MAX: usize = 100;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(list.clone());
        for num in 0..=MAX {
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
        const SIZE: usize = 10000;
        const MAX: usize = 100;
        let list = (0..SIZE)
            .map(|_| rng.gen_range(0..=MAX))
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::new(list.clone());
        for _ in 0..100 {
            let left = rng.gen_range(0..SIZE);
            let right = rng.gen_range(left + 1..SIZE);
            let k = rng.gen_range(0..=right - left - 1);
            let mut sorted = list[left..right].to_vec();
            sorted.sort();
            assert_eq!(wm.quantile(left..right, k), sorted[k]);
        }
    }
}
