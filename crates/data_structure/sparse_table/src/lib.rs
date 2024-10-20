//! 冪等モノイドが乗った静的な区間クエリを処理する  
//! Disjoint Sparse Table に比べて定数倍早い  

use algebra::IdempotentMonoid;
use std::ops::RangeBounds;

#[derive(Debug, Clone)]
pub struct SparseTable<M: IdempotentMonoid> {
    range_size: usize,
    data: Vec<Vec<M::Target>>,
}

impl<M: IdempotentMonoid> SparseTable<M> {
    /// `O(nlogn)`
    pub fn new(v: Vec<M::Target>) -> Self {
        let range_size = v.len();
        let mut data = vec![v];
        let log_floor = if range_size == 0 {
            0
        } else {
            range_size.ilog2() as usize
        };
        for i in 1..=log_floor {
            let mut row = vec![M::id_element(); range_size - (1 << i) + 1];
            for (j, r) in row.iter_mut().enumerate() {
                *r = M::binary_operation(&data[i - 1][j], &data[i - 1][j + (1 << (i - 1))]);
            }
            data.push(row);
        }
        Self { range_size, data }
    }

    /// `O(1)`
    pub fn prod<R: RangeBounds<usize>>(&self, range: R) -> M::Target {
        use std::ops::Bound::*;
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        if l == r {
            return M::id_element();
        }
        let k = (r - l).ilog2() as usize;
        M::binary_operation(&self.data[k][l], &self.data[k][r - (1 << k)])
    }
}
