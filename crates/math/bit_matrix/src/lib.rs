//! mod 2の世界での行列  

use bitset::BitSet;
use std::ops::Index;

pub trait BitMatrixOps {
    fn add(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix;
    fn mul(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// `+ = or, * = and`
pub struct PlusOrMulAnd {}
impl BitMatrixOps for PlusOrMulAnd {
    fn add(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix {
        assert_eq!(lhs.width, rhs.width);
        assert_eq!(lhs.height, rhs.height);
        let mut ret = BitMatrix::new(lhs.height, lhs.width);
        for row in 0..lhs.height {
            ret.data[row] = &lhs.data[row] | &rhs.data[row];
        }
        ret
    }
    fn mul(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix {
        assert_eq!(lhs.width, rhs.height);
        let mut ret = BitMatrix::new(lhs.height, rhs.width);
        let rhs = rhs.transpose();
        for i in 0..lhs.height {
            for j in 0..rhs.height {
                let val = lhs.data[i]
                    .buffer()
                    .iter()
                    .zip(rhs.data[j].buffer())
                    .fold(false, |acc, (l, r)| acc | ((l & r).count_ones() > 0));
                ret.set(i, j, val);
            }
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// `+ = xor, * = and`
pub struct PlusXorMulAnd {}
impl BitMatrixOps for PlusXorMulAnd {
    fn add(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix {
        assert_eq!(lhs.width, rhs.width);
        assert_eq!(lhs.height, rhs.height);
        let mut ret = BitMatrix::new(lhs.height, lhs.width);
        for row in 0..lhs.height {
            ret.data[row] = &lhs.data[row] ^ &rhs.data[row];
        }
        ret
    }
    fn mul(lhs: &BitMatrix, rhs: &BitMatrix) -> BitMatrix {
        assert_eq!(lhs.width, rhs.height);
        let mut ret = BitMatrix::new(lhs.height, rhs.width);
        let rhs = rhs.transpose();
        for i in 0..lhs.height {
            for j in 0..rhs.height {
                let val = lhs.data[i]
                    .buffer()
                    .iter()
                    .zip(rhs.data[j].buffer())
                    .fold(false, |acc, (l, r)| acc ^ ((l & r).count_ones() & 1 > 0));
                ret.set(i, j, val);
            }
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitMatrix {
    height: usize,
    width: usize,
    data: Vec<BitSet>,
}

impl From<Vec<Vec<bool>>> for BitMatrix {
    fn from(v: Vec<Vec<bool>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let data = v.into_iter().map(BitSet::from).collect();
        Self {
            height,
            width,
            data,
        }
    }
}

impl<const H: usize, const W: usize> From<[[bool; W]; H]> for BitMatrix {
    fn from(v: [[bool; W]; H]) -> Self {
        let height = H;
        let width = W;
        let data = v.into_iter().map(BitSet::from).collect();
        Self {
            height,
            width,
            data,
        }
    }
}

impl From<Vec<BitSet>> for BitMatrix {
    fn from(v: Vec<BitSet>) -> Self {
        let height = v.len();
        let width = v[0].size();
        Self {
            height,
            width,
            data: v,
        }
    }
}

impl<const H: usize> From<[BitSet; H]> for BitMatrix {
    fn from(v: [BitSet; H]) -> Self {
        let height = H;
        let width = v[0].size();
        Self {
            height,
            width,
            data: v.to_vec(),
        }
    }
}

impl BitMatrix {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            data: vec![BitSet::new(width); height],
        }
    }

    /// indexでアクセスしてもよい
    pub fn get(&self, row: usize, col: usize) -> bool {
        assert!(row < self.height && col < self.width);
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, b: bool) {
        assert!(row < self.height && col < self.width);
        self.data[row].set(col, b);
    }

    /// 掃き出し法(行基本変形)を行い、rankと線形独立な行のindexの集合を返す  
    /// is_extendedがtrueの場合は拡大係数行列として扱い、係数行列の部分のrankを返す
    pub fn gauss_jordan(&mut self, is_extended: bool) -> (usize, Vec<usize>) {
        let mut rank = 0;
        let col_end = if is_extended {
            self.width - 1
        } else {
            self.width
        };
        let mut independent = vec![];
        let mut ids = (0..self.height).collect::<Vec<_>>();
        for col in 0..col_end {
            let mut pivot = None;
            for row in rank..self.height {
                if self.data[row][col] {
                    pivot = Some(row);
                    break;
                }
            }
            if let Some(pivot) = pivot {
                self.data.swap(rank, pivot);
                ids.swap(rank, pivot);
                for row in 0..self.height {
                    if row != rank && self.data[row][col] {
                        unsafe {
                            *self.data.as_mut_ptr().add(row) ^= &self.data[rank];
                        }
                    }
                }
                independent.push(ids[rank]);
                rank += 1;
            }
        }
        assert_eq!(rank, independent.len());
        (rank, independent)
    }

    /// 連立一次方程式 Ax = bを解く(Aがselfの行列、bが引数のベクトル)  
    /// 解が存在する場合は`Some((freedom, solution))`を返し、存在しない場合は`None`を返す  
    /// freedomは解の自由度、solutionは解の一つを表すベクトル  
    /// 解の個数は2^freedom個となる
    pub fn linear_equation(&self, b: &[bool]) -> Option<(usize, Vec<bool>)> {
        assert_eq!(self.height, b.len());
        let mut mat = BitMatrix::new(self.height, self.width + 1);
        #[allow(clippy::needless_range_loop)]
        for i in 0..self.height {
            for j in 0..self.width {
                mat.set(i, j, self.get(i, j));
            }
            mat.set(i, self.width, b[i]);
        }
        let (rank, _) = mat.gauss_jordan(true);
        for i in rank..self.height {
            if mat.get(i, self.width) {
                return None;
            }
        }
        let mut ans = vec![false; self.width];
        let mut cur_col = 0;
        for r in 0..rank {
            while cur_col < self.width && !mat.get(r, cur_col) {
                cur_col += 1;
            }
            assert!(cur_col < self.width);
            ans[cur_col] = mat.get(r, self.width);
            cur_col += 1;
        }
        // 解の自由度
        let freedom = self.width - rank;
        Some((freedom, ans))
    }

    pub fn unit(n: usize) -> Self {
        let mut res = Self::new(n, n);
        for i in 0..n {
            res.set(i, i, true);
        }
        res
    }

    pub fn transpose(&self) -> Self {
        let mut res = Self::new(self.width, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                res.set(j, i, self.get(i, j));
            }
        }
        res
    }

    pub fn add<F: BitMatrixOps>(&self, rhs: &Self) -> Self {
        F::add(self, rhs)
    }

    pub fn mul<F: BitMatrixOps>(&self, rhs: &Self) -> Self {
        F::mul(self, rhs)
    }

    /// 行列のべき乗を計算する  
    pub fn pow<F: BitMatrixOps>(&self, mut n: u64) -> Self {
        assert_eq!(self.height, self.width);
        let mut res = Self::unit(self.height);
        let mut a = self.clone();
        while n > 0 {
            if (n & 1) == 1 {
                res = F::mul(&res, &a);
            }
            a = F::mul(&a, &a);
            n >>= 1;
        }
        res
    }
}

impl Index<usize> for BitMatrix {
    type Output = BitSet;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn independent_test() {
        let mut rng = thread_rng();
        for _ in 0..10 {
            let w = rng.gen_range(1..=10);
            let h = rng.gen_range(w..=3 * w);
            let mut mat = BitMatrix::new(h, w);
            for i in 0..h {
                for j in 0..w {
                    mat.set(i, j, rng.gen());
                }
            }
            let nums = {
                let mut nums = vec![0; h];
                for i in 0..h {
                    for j in 0..w {
                        if mat.get(i, j) {
                            nums[i] |= 1 << j;
                        }
                    }
                }
                nums
            };
            let (rank, independent) = mat.gauss_jordan(false);
            for i in 0..rank {
                let cur_num = nums[independent[i]];
                for bit in 0..(1 << rank) {
                    let mut num = 0;
                    for j in 0..rank {
                        if j == i {
                            continue;
                        }
                        if bit & (1 << j) > 0 {
                            num ^= nums[independent[j]];
                        }
                    }
                    assert_ne!(num, cur_num);
                }
            }
        }
    }

    #[test]
    fn test_independent_manual() {
        let mut mat = BitMatrix::from([
            [true, false, false, true],
            [false, false, true, false],
            [true, false, true, true],
            [false, true, false, true],
            [true, true, true, false],
        ]);
        let (rank, mut independent) = mat.gauss_jordan(false);
        assert_eq!(rank, 3);
        independent.sort();
        assert_eq!(independent, vec![0, 2, 3]);

        let mut mat = BitMatrix::from([
            [true, false, false],
            [true, false, false],
            [true, false, false],
            [true, true, false],
            [false, true, false],
            [false, false, false],
            [false, true, false],
            [false, true, true],
        ]);
        let (rank, mut independent) = mat.gauss_jordan(false);
        assert_eq!(rank, 3);
        independent.sort();
        assert_eq!(independent, vec![0, 3, 7]);
    }

    #[test]
    fn linear_eq_test() {
        let mut rng = thread_rng();
        let mut no_ans_cnt = 0;
        for _ in 0..10 {
            let n = rng.gen_range(1..=300);
            let m = rng.gen_range(n..=300);
            let mut mat = BitMatrix::new(n, m);
            let mut b = vec![false; n];
            for i in 0..n {
                for j in 0..m {
                    mat.set(i, j, rng.gen());
                }
                b[i] = rng.gen();
            }
            let Some((rank, ans)) = mat.linear_equation(&b) else {
                no_ans_cnt += 1;
                continue;
            };
            assert!(rank <= ans.len());
            for i in 0..n {
                let mut sum = false;
                for j in 0..m {
                    sum ^= mat.get(i, j) && ans[j];
                }
                assert_eq!(sum, b[i]);
            }

            // 行列の掛け算でも確認
            let b_mat = BitMatrix::from(vec![b]).transpose();
            let ans_mat = BitMatrix::from(vec![ans]).transpose();
            assert_eq!(mat.mul::<PlusXorMulAnd>(&ans_mat), b_mat);
        }
        eprintln!("no_ans_cnt: {}", no_ans_cnt);
    }

    #[test]
    fn test_skip_col() {
        // 3つめのpivotが3列目を飛ばして4列目にくる例
        let mat = BitMatrix::from([
            [true, false, true, true, false],
            [false, true, false, true, true],
            [false, false, false, true, true],
            [false, false, false, false, true],
        ]);
        let b = [true, false, true, false];
        let (freedom, ans) = mat.linear_equation(&b).unwrap();
        assert_eq!(freedom, 1);
        assert_eq!(ans, vec![false, true, false, true, false]);
    }

    #[test]
    fn test_pow() {
        let mut rng = thread_rng();
        let mat = BitMatrix::from([[true, true], [false, true]]);
        for _ in 0..100 {
            let beki = rng.gen_range(0_u64..10_u64.pow(18));
            let ans = mat.pow::<PlusXorMulAnd>(beki);
            if (beki & 1) > 0 {
                assert_eq!(ans, mat);
            } else {
                assert_eq!(ans, BitMatrix::unit(2));
            }
        }
    }
}
