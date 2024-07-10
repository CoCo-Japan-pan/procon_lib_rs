//! mod 2の世界での行列  

use bitset::BitSet;
use std::ops::Index;

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

    /// 掃き出し法を行い、rankを返す  
    /// is_extendedがtrueの場合は拡大係数行列として扱い、係数行列の部分のrankを返す
    pub fn gauss_jordan(&mut self, is_extended: bool) -> usize {
        let mut rank = 0;
        let col_end = if is_extended {
            self.width - 1
        } else {
            self.width
        };
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
                for row in 0..self.height {
                    if row != rank && self.data[row][col] {
                        unsafe {
                            *self.data.as_mut_ptr().add(row) ^= &self.data[rank];
                        }
                    }
                }
                rank += 1;
            }
        }
        rank
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
        let rank = mat.gauss_jordan(true);
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

    /// `+ = xor, * = and` による行列積
    pub fn xor_and_mul(lhs: &Self, rhs: &Self) -> Self {
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

    /// `+ = or, * = and` による行列積
    pub fn or_and_mul(lhs: &Self, rhs: &Self) -> Self {
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

    /// 行列のべき乗を計算する  
    /// `mul_func`は行列積を計算する関数を指定する  
    /// 足し算がxor/or, 掛け算がandの場合はメソッド関数の`xor_and_mul`/`or_and_mul`を指定すればよい
    pub fn pow<F>(&self, mut n: u64, mul_func: F) -> Self
    where
        F: Fn(&Self, &Self) -> Self,
    {
        assert_eq!(self.height, self.width);
        let mut res = Self::unit(self.height);
        let mut a = self.clone();
        while n > 0 {
            if (n & 1) == 1 {
                res = mul_func(&res, &a);
            }
            a = mul_func(&a, &a);
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
    fn linear_eq_test() {
        let mut rng = thread_rng();
        let mut no_ans_cnt = 0;
        for _ in 0..10 {
            let n = rng.gen_range(1..=1000);
            let m = rng.gen_range(n..=1000);
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
            assert_eq!(BitMatrix::xor_and_mul(&mat, &ans_mat), b_mat);
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
            let ans = mat.pow(beki, BitMatrix::xor_and_mul);
            if (beki & 1) > 0 {
                assert_eq!(ans, mat);
            } else {
                assert_eq!(ans, BitMatrix::unit(2));
            }
        }
    }
}
