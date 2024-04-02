//! 2D Sparse Tableとできることは同じだが、SegmentTreeとSparseTableを組み合わせている  
//! つまりクエリにlogが一つかかる代わりに、構築のlogが一つ減る  
//! <https://maspypy.github.io/library/ds/sparse_table/sparse_table_on_segtree.hpp> で知りました  

use algebra::IdempotentMonoid;
use sparse_table::SparseTable;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct SparseTableOnSegTree<M: IdempotentMonoid + Clone> {
    range_height: usize,
    data: Vec<SparseTable<M>>,
}

impl<M: IdempotentMonoid + Clone> SparseTableOnSegTree<M> {
    pub fn new(v: Vec<Vec<M::Target>>) -> Self {
        let range_height = v.len();
        let range_width = v[0].len();
        let mut data = vec![SparseTable::<M>::new(vec![]); range_height * 2];
        for (i, v) in v.into_iter().enumerate() {
            data[range_height + i] = SparseTable::<M>::new(v);
        }
        for i in (1..range_height).rev() {
            data[i] = SparseTable::<M>::new(
                (0..range_width)
                    .map(|j| {
                        M::binary_operation(
                            &data[i * 2].prod(j..j + 1),
                            &data[i * 2 + 1].prod(j..j + 1),
                        )
                    })
                    .collect(),
            );
        }
        Self { range_height, data }
    }

    pub fn prod<R1: RangeBounds<usize>, R2: RangeBounds<usize> + Clone>(
        &self,
        height_range: R1,
        width_range: R2,
    ) -> M::Target {
        let mut height_left = match height_range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut height_right = match height_range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.range_height,
        };
        assert!(height_left <= height_right && height_right <= self.range_height);
        let mut ret = M::id_element();
        while height_left < height_right {
            if height_left & 1 != 0 {
                ret = M::binary_operation(&ret, &self.data[height_left].prod(width_range.clone()));
                height_left += 1;
            }
            if height_right & 1 != 0 {
                height_right -= 1;
                ret = M::binary_operation(&ret, &self.data[height_right].prod(width_range.clone()));
            }
        }
        ret
    }
}
