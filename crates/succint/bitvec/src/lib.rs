//! 完備辞書  
//! 1.5N bit 用いているので、succintではない compactではある  
//! u64のブロックのみを使い、小ブロックは使わない  
//! selectの高速化のために、x86_64の命令を(使えれば)使う  
//! しかしselect用の索引を持たせてないので、selectはO(logN)  
//! 雑なベンチマークによると、select1はrank1の4~5倍程度の時間がかかりそう

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
}

impl BitVec {
    pub fn new(bitvec: &[bool]) -> Self {
        let len = bitvec.len();
        let b_len = (len + 63) / 64;
        let mut blocks = vec![
            Block {
                block: 0,
                cum_sum_popcnt: 0
            };
            b_len
        ];
        for i in 0..len {
            if bitvec[i] {
                blocks[i >> 6].block |= 1 << (i & 63);
            }
        }
        let mut popcnt = 0;
        for b in blocks.iter_mut() {
            b.cum_sum_popcnt = popcnt;
            popcnt += b.block.count_ones();
        }
        Self {
            len,
            blocks,
            all_popcnt: popcnt,
        }
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

    /// 0-basedでi番目の1の位置 O(logN)
    pub fn select1(&self, i: usize) -> Option<usize> {
        if i >= self.all_popcnt as usize {
            return None;
        }
        let i = i as u32;
        let mut ok = 0;
        let mut ng = self.blocks.len();
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

    /// 0-basedでi番目の0の位置 O(logN)
    pub fn select0(&self, i: usize) -> Option<usize> {
        let all_0 = self.len - self.all_popcnt as usize;
        if i >= all_0 {
            return None;
        }
        let mut ok = 0;
        let mut ng = self.blocks.len();
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
        let mut rng = thread_rng();
        const SIZE: usize = 250000;
        let bool_vec = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();
        let bit_vec = BitVec::new(&bool_vec);
        let mut ans1 = vec![0; SIZE + 1];
        let mut ans0 = vec![0; SIZE + 1];
        for i in 0..SIZE {
            ans1[i + 1] = ans1[i] + bool_vec[i] as usize;
            ans0[i + 1] = ans0[i] + !bool_vec[i] as usize;
        }
        for i in 0..SIZE {
            assert_eq!(bit_vec.rank1(i), ans1[i]);
            assert_eq!(bit_vec.rank0(i), ans0[i]);
        }
    }

    #[test]
    fn test_select() {
        let mut rng = thread_rng();
        const SIZE: usize = 250000;
        let bool_vec = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();
        let bit_vec = BitVec::new(&bool_vec);
        let mut one_indices = Vec::with_capacity(bit_vec.all_popcnt as usize);
        let mut zero_indices = Vec::with_capacity(SIZE - bit_vec.all_popcnt as usize);
        for i in 0..SIZE {
            if bool_vec[i] {
                one_indices.push(i);
            } else {
                zero_indices.push(i);
            }
        }
        for i in 0..SIZE {
            assert_eq!(bit_vec.select1(i), one_indices.get(i).copied());
            assert_eq!(bit_vec.select0(i), zero_indices.get(i).copied());
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
        let bit_vec = BitVec::new(&bool_vec);
        let rand_nums = {
            let mut rand_nums = (0..SIZE).collect::<Vec<_>>();
            rand_nums.shuffle(&mut rng);
            rand_nums
        };
        stop_watch();
        for &i in &rand_nums {
            assert!(bit_vec.rank1(i) <= i);
        }
        println!("rank1: {:.6}", stop_watch());
        for &i in &rand_nums {
            if let Some(pos) = bit_vec.select1(i) {
                assert_eq!(bit_vec.rank1(pos), i);
            }
        }
        println!("select1, rank1: {:.6}", stop_watch());
    }
}
