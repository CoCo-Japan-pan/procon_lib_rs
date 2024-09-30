//! 完備辞書  
//! 2N + O(1) bit 用いているので、succintではない compactではある  
//! u64のブロックのみを使い、小ブロックは使わない  
//! selectの高速化のために、x86_64の命令を(使えれば)使う  
//! 雑なベンチマークによると、select1はbit列がランダムなら平均でrank1の2倍程度の時間がかかりそう

/// キャッシュ効率のため、ブロックとその前のブロックまでの1の数をまとめて持つ
#[derive(Debug, Clone, Copy)]
struct Block {
    block: u64,
    cum_sum_popcnt: u32,
}

#[derive(Debug, Clone)]
pub struct BitVec {
    len: usize,
    blocks: Vec<Block>,
    all_popcnt: u32,
    /// one_select[i] = 64i番目の1が属するブロックのindex
    one_select: Vec<u32>,
    /// zero_select[i] = 64i番目の0が属するブロックのindex
    zero_select: Vec<u32>,
}

impl From<&[bool]> for BitVec {
    fn from(bitvec: &[bool]) -> Self {
        let len = bitvec.len();
        let mut ret = Self::new(len);
        for (i, &b) in bitvec.iter().enumerate() {
            if b {
                ret.set(i);
            }
        }
        ret.build();
        ret
    }
}

impl BitVec {
    /// 0で初期化されたビット列を作成
    pub fn new(len: usize) -> Self {
        Self {
            len,
            blocks: vec![
                Block {
                    block: 0,
                    cum_sum_popcnt: 0
                };
                (len + 63) >> 6
            ],
            all_popcnt: 0,
            one_select: Vec::new(),
            zero_select: Vec::new(),
        }
    }

    /// i番目のビットを立てる new()で作成した場合はこちらで一つずつ立てる
    pub fn set(&mut self, i: usize) {
        debug_assert!(i < self.len);
        self.blocks[i >> 6].block |= 1 << (i & 63);
    }

    /// 直接setを用いた場合は最後にこれを必ず呼ぶ
    pub fn build(&mut self) {
        let all_popcnt = self
            .blocks
            .iter()
            .map(|b| b.block.count_ones())
            .sum::<u32>() as usize;
        let mut popcnt = 0;
        let one_num = (all_popcnt >> 6) + 1;
        let zero_num = ((self.len - all_popcnt) >> 6) + 1;
        let mut one_select = Vec::with_capacity(one_num);
        let mut zero_select = Vec::with_capacity(zero_num);
        for (i, b) in self.blocks.iter_mut().enumerate() {
            if popcnt as usize >= one_select.len() << 6 {
                one_select.push(i as u32);
            }
            if (i << 6) - popcnt as usize >= zero_select.len() << 6 {
                zero_select.push(i as u32);
            }
            b.cum_sum_popcnt = popcnt;
            popcnt += b.block.count_ones();
        }
        assert_eq!(popcnt as usize, all_popcnt);
        self.all_popcnt = popcnt;
        self.one_select = one_select;
        self.zero_select = zero_select;
    }

    /// [0..i)の1の数 O(1)
    pub fn rank1(&self, i: usize) -> usize {
        debug_assert!(i <= self.len);
        let Block {
            block,
            cum_sum_popcnt,
        } = self.blocks[i >> 6];
        let mask = (1 << (i & 63)) - 1;
        let popcnt = (block & mask).count_ones();
        (cum_sum_popcnt + popcnt) as usize
    }

    /// [0..i)の0の数 O(1)
    pub fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }

    /// 0-basedでi番目の1の位置 最悪O(logN) 平均O(1)
    pub fn select1(&self, i: usize) -> Option<usize> {
        if i >= self.all_popcnt as usize {
            return None;
        }
        // ブロックで二分探索を行うが、その範囲は索引で絞る
        // self.blocks[ok].cum_sum_popcnt <= i < self.blocks[ng].cum_sum_popcnt
        let mut ok = if let Some(&ok) = self.one_select.get(i >> 6) {
            ok.saturating_sub(1) as usize
        } else {
            self.blocks.len().saturating_sub(1)
        };
        let mut ng = if let Some(&ng) = self.one_select.get((i >> 6) + 1) {
            ng as usize
        } else {
            self.blocks.len()
        };
        let i = i as u32;
        while ng - ok > 1 {
            let mid = (ok + ng) >> 1;
            if self.blocks[mid].cum_sum_popcnt <= i {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let rem = i - self.blocks[ok].cum_sum_popcnt;
        // okのブロックの中でのrem番目の1の位置
        let offset = select1_u64(self.blocks[ok].block, rem as usize);
        Some((ok << 6) + offset as usize)
    }

    /// 0-basedでi番目の0の位置 最悪O(logN) 平均O(1)
    pub fn select0(&self, i: usize) -> Option<usize> {
        let all_0 = self.len - self.all_popcnt as usize;
        if i >= all_0 {
            return None;
        }
        let mut ok = if let Some(&ok) = self.zero_select.get(i >> 6) {
            ok.saturating_sub(1) as usize
        } else {
            self.blocks.len().saturating_sub(1)
        };
        let mut ng = if let Some(&ng) = self.zero_select.get((i >> 6) + 1) {
            ng as usize
        } else {
            self.blocks.len()
        };
        while ng - ok > 1 {
            let mid = (ok + ng) >> 1;
            if ((mid << 6) - self.blocks[mid].cum_sum_popcnt as usize) <= i {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let rem = i - ((ok << 6) - self.blocks[ok].cum_sum_popcnt as usize);
        // okのブロックの中でのrem番目の0の位置
        let offset = select1_u64(!self.blocks[ok].block, rem);
        Some((ok << 6) + offset as usize)
    }
}

#[cfg(target_arch = "x86_64")]
fn select1_u64(x: u64, index: usize) -> u32 {
    use std::arch::x86_64::_pdep_u64;
    let z = 1 << index;
    let y = unsafe { _pdep_u64(z, x) };
    y.trailing_zeros()
}

#[cfg(not(target_arch = "x86_64"))]
fn select1_u64(x: u64, index: usize) -> u32 {
    let mut left = 0;
    let mut right = 64;
    while right - left > 1 {
        let mid = (left + right) >> 1;
        if (x & ((1 << mid) - 1)).count_ones() > index as u32 {
            right = mid;
        } else {
            left = mid;
        }
    }
    left
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_rank() {
        fn test(size: usize) {
            let mut rng = thread_rng();
            let bool_vec = (0..size).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();
            let bit_vec = BitVec::from(&bool_vec[..]);
            let mut ans1 = vec![0; size + 1];
            let mut ans0 = vec![0; size + 1];
            for i in 0..size {
                ans1[i + 1] = ans1[i] + bool_vec[i] as usize;
                ans0[i + 1] = ans0[i] + !bool_vec[i] as usize;
            }
            for i in 0..size {
                assert_eq!(bit_vec.rank1(i), ans1[i]);
                assert_eq!(bit_vec.rank0(i), ans0[i]);
            }
        }
        for size in [0, 1, 63, 64, 65, 100, 1000, 10000, 100000, 250000] {
            test(size);
        }
    }

    #[test]
    fn test_select() {
        fn test(size: usize) {
            let mut rng = thread_rng();
            let bool_vec = (0..size).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();
            let bit_vec = BitVec::from(&bool_vec[..]);
            let mut one_indices = Vec::with_capacity(bit_vec.all_popcnt as usize);
            let mut zero_indices = Vec::with_capacity(size - bit_vec.all_popcnt as usize);
            for i in 0..size {
                if bool_vec[i] {
                    one_indices.push(i);
                } else {
                    zero_indices.push(i);
                }
            }
            for i in 0..size {
                assert_eq!(bit_vec.select1(i), one_indices.get(i).copied());
                assert_eq!(bit_vec.select0(i), zero_indices.get(i).copied());
            }
        }
        for size in [0, 1, 63, 64, 65, 100, 1000, 10000, 100000, 250000] {
            test(size);
        }
    }

    #[test]
    fn bench() {
        fn stop_watch() -> f64 {
            use std::time::{SystemTime, UNIX_EPOCH};
            static mut START: f64 = 0.0;
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let current = time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9;
            unsafe {
                let ret = current - START;
                START = current;
                ret
            }
        }

        let mut rng = thread_rng();
        const SIZE: usize = 250000;
        let bool_vec = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();
        let bit_vec = BitVec::from(&bool_vec[..]);
        let rand_nums = {
            let mut rand_nums = (0..SIZE).collect::<Vec<_>>();
            rand_nums.shuffle(&mut rng);
            rand_nums
        };
        stop_watch();
        use std::hint::black_box;
        for &i in &rand_nums {
            black_box(bit_vec.rank1(i));
        }
        println!("rank1: {:.6}", stop_watch());
        for &i in &rand_nums {
            black_box(bit_vec.select1(i));
        }
        println!("select1: {:.6}", stop_watch());
        for &i in &rand_nums {
            black_box(bit_vec.rank0(i));
        }
        println!("rank0: {:.6}", stop_watch());
        for &i in &rand_nums {
            black_box(bit_vec.select0(i));
        }
        println!("select0: {:.6}", stop_watch());
    }
}
