//! 内部で2次元配列を持つセグメント木  
//! `O(HW)`のメモリを使うので注意  
//! 2次元なので可換性を要求    
//! <https://nasubi-blog.hatenablog.com/entry/2021/11/27/185818>の図が分かりやすかったです  

use algebra::{Commutative, Monoid};
use internal_bits::ceil_log2;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct SegTree2D<M: Monoid + Commutative> {
    height: usize,
    width: usize,
    ceil_log_h: usize,
    ceil_log_w: usize,
    leaf_height: usize,
    leaf_width: usize,
    data: Vec<M::Target>,
}

macro_rules! index {
    ($self: expr, $h:expr, $w:expr) => {
        $h * $self.leaf_width * 2 + $w
    };
}

impl<M: Monoid + Commutative> From<&Vec<Vec<M::Target>>> for SegTree2D<M> {
    fn from(v: &Vec<Vec<M::Target>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let ceil_log_h = ceil_log2(height as u32) as usize;
        let ceil_log_w = ceil_log2(width as u32) as usize;
        let leaf_height = 1 << ceil_log_h;
        let leaf_width = 1 << ceil_log_w;
        let mut data = vec![M::id_element(); leaf_width * leaf_height * 4];
        for (h, v) in v.iter().enumerate() {
            let base = (leaf_height + h) * leaf_width * 2 + leaf_width;
            data[base..base + width].clone_from_slice(v);
        }
        let mut ret = SegTree2D {
            height,
            width,
            ceil_log_h,
            ceil_log_w,
            leaf_height,
            leaf_width,
            data,
        };
        for h in (1..leaf_height).rev() {
            for w in (leaf_width..leaf_width * 2).rev() {
                ret.update_from_col_leaf(h, w);
            }
        }
        for h in (1..leaf_height * 2).rev() {
            for w in (1..leaf_width).rev() {
                ret.update_from_row_leaf(h, w);
            }
        }
        ret
    }
}

impl<M: Monoid + Commutative> SegTree2D<M> {
    pub fn new(height: usize, width: usize) -> Self {
        (&vec![vec![M::id_element(); width]; height]).into()
    }

    pub fn set(&mut self, h: usize, w: usize, x: M::Target) {
        assert!(h < self.height && w < self.width);
        let h = h + self.leaf_height;
        let w = w + self.leaf_width;
        self.data[index!(self, h, w)] = x;
        for i in 1..=self.ceil_log_h {
            self.update_from_col_leaf(h >> i, w);
        }
        for i in 0..=self.ceil_log_h {
            for j in 1..=self.ceil_log_w {
                self.update_from_row_leaf(h >> i, w >> j);
            }
        }
    }

    pub fn get(&self, h: usize, w: usize) -> M::Target {
        assert!(h < self.height && w < self.width);
        self.data[index!(self, h + self.leaf_height, w + self.leaf_width)].clone()
    }

    pub fn all_prod(&self) -> M::Target {
        self.data[index!(self, 1, 1)].clone()
    }

    pub fn prod<R1: RangeBounds<usize>, R2: RangeBounds<usize>>(
        &self,
        height_range: R1,
        width_range: R2,
    ) -> M::Target {
        let mut h_left = match height_range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut h_right = match height_range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.height,
        };
        assert!(h_left <= h_right && h_right <= self.height);
        let w_left = match width_range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let w_right = match width_range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.width,
        };
        assert!(w_left <= w_right && w_right <= self.width);
        if h_left == 0 && h_right == self.height && w_left == 0 && w_right == self.width {
            return self.all_prod();
        }
        h_left += self.leaf_height;
        h_right += self.leaf_height;
        let w_left = w_left + self.leaf_width;
        let w_right = w_right + self.leaf_width;
        let mut ret = M::id_element();
        while h_left < h_right {
            if h_left & 1 != 0 {
                ret = M::binary_operation(&ret, &self.prod_row(h_left, w_left, w_right));
                h_left += 1;
            }
            if h_right & 1 != 0 {
                h_right -= 1;
                ret = M::binary_operation(&self.prod_row(h_right, w_left, w_right), &ret);
            }
            h_left >>= 1;
            h_right >>= 1;
        }
        ret
    }
}

impl<M: Monoid + Commutative> SegTree2D<M> {
    fn update_from_col_leaf(&mut self, h: usize, w: usize) {
        self.data[index!(self, h, w)] = M::binary_operation(
            &self.data[index!(self, h * 2, w)],
            &self.data[index!(self, h * 2 + 1, w)],
        );
    }
    /// colに比べてキャッシュ効率が良いので、こっちを多く使いたい
    fn update_from_row_leaf(&mut self, h: usize, w: usize) {
        self.data[index!(self, h, w)] = M::binary_operation(
            &self.data[index!(self, h, w * 2)],
            &self.data[index!(self, h, w * 2 + 1)],
        );
    }
    fn prod_row(&self, h: usize, mut w_left: usize, mut w_right: usize) -> M::Target {
        let mut ret = M::id_element();
        while w_left < w_right {
            if w_left & 1 != 0 {
                ret = M::binary_operation(&ret, &self.data[index!(self, h, w_left)]);
                w_left += 1;
            }
            if w_right & 1 != 0 {
                w_right -= 1;
                ret = M::binary_operation(&self.data[index!(self, h, w_right)], &ret);
            }
            w_left >>= 1;
            w_right >>= 1;
        }
        ret
    }
}
