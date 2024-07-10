//! mod 2の世界での(一般の足し算、掛け算に関する)行列  
//! つまり足し算がXOR、掛け算がANDである行列を扱う  

use bitset::BitSet;

pub struct BitMatrix {
    height: usize,
    width: usize,
    data: Vec<BitSet>,
}

impl BitMatrix {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            data: vec![BitSet::new(width); height],
        }
    }

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
    /// 解が存在する場合はrankと解(の一つ)を返し、存在しない場合はNoneを返す
    pub fn linear_equation(&self, b: &[bool]) -> Option<(usize, Vec<bool>)> {
        assert_eq!(self.height, b.len());
        todo!()
    }
}
