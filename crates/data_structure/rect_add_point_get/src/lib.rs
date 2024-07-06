//! 矩形範囲への可換作用の適用、1点取得のクエリを処理できます  
//! 点の追加と削除はできません(オフライン前提)  
//! [kd-tree](https://trap.jp/post/1489/)を参考に、kd-treeで実装しています  

use algebra::{Action, Commutative};
use internal_type_traits::Integral;
use std::ops::{Bound::*, RangeBounds};

/// Aは可換作用、Tは座標の型
#[derive(Debug)]
pub struct RectActPointGet<A: Action + Commutative, T: Integral> {
    x_min: T,
    x_max: T,
    y_min: T,
    y_max: T,
    lazy: A,
    left: Option<Box<RectActPointGet<A, T>>>,
    right: Option<Box<RectActPointGet<A, T>>>,
}

impl<A: Action + Commutative, T: Integral> RectActPointGet<A, T> {
    /// 点取得のクエリを先読みして、その点を用いて構築
    pub fn new(mut points: Vec<(T, T)>) -> Self {
        // 重複除去
        points.sort();
        points.dedup();
        Self::new_sub(&mut points)
    }

    fn new_sub(points: &mut [(T, T)]) -> Self {
        let mut x_min = T::max_value();
        let mut x_max = T::min_value();
        let mut y_min = T::max_value();
        let mut y_max = T::min_value();
        for (x, y) in points.iter() {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
            y_min = y_min.min(*y);
            y_max = y_max.max(*y);
        }
        let size = points.len();
        let mut ret = RectActPointGet {
            x_min,
            x_max,
            y_min,
            y_max,
            lazy: A::id_map(),
            left: None,
            right: None,
        };
        if size <= 1 {
            return ret;
        }
        let mid = size / 2;
        let x_idx = {
            let x_mid = points.select_nth_unstable_by_key(mid, |(x, _)| *x).1 .0;
            let excluded_cnt = points.iter().filter(|(x, _)| *x < x_mid).count();
            let included_cnt = points.iter().filter(|(x, _)| *x <= x_mid).count();
            if (size - excluded_cnt).abs_diff(excluded_cnt)
                < (size - included_cnt).abs_diff(included_cnt)
            {
                excluded_cnt
            } else {
                included_cnt
            }
        };
        let y_idx = {
            let y_mid = points.select_nth_unstable_by_key(mid, |(_, y)| *y).1 .1;
            let excluded_cnt = points.iter().filter(|(_, y)| *y < y_mid).count();
            let included_cnt = points.iter().filter(|(_, y)| *y <= y_mid).count();
            if (size - excluded_cnt).abs_diff(excluded_cnt)
                < (size - included_cnt).abs_diff(included_cnt)
            {
                excluded_cnt
            } else {
                included_cnt
            }
        };
        if (size - x_idx).abs_diff(x_idx) < (size - y_idx).abs_diff(y_idx) {
            points.select_nth_unstable_by_key(x_idx, |(x, _)| *x);
            ret.left = Some(Box::new(Self::new_sub(&mut points[..x_idx])));
            ret.right = Some(Box::new(Self::new_sub(&mut points[x_idx..])));
        } else {
            points.select_nth_unstable_by_key(y_idx, |(_, y)| *y);
            ret.left = Some(Box::new(Self::new_sub(&mut points[..y_idx])));
            ret.right = Some(Box::new(Self::new_sub(&mut points[y_idx..])));
        }
        ret
    }

    pub fn add_range<R1: RangeBounds<T>, R2: RangeBounds<T>>(
        &mut self,
        x_range: &R1,
        y_range: &R2,
        action: &A,
    ) {
        // 今回は内部では閉区間で扱う
        let x_min = match x_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => self.x_min,
        };
        let x_max = match x_range.end_bound() {
            Included(&r) => r,
            Excluded(&r) => r - T::one(),
            Unbounded => self.x_max,
        };
        let y_min = match y_range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + T::one(),
            Unbounded => self.y_min,
        };
        let y_max = match y_range.end_bound() {
            Included(&r) => r,
            Excluded(&r) => r - T::one(),
            Unbounded => self.y_max,
        };

        if x_max < self.x_min || self.x_max < x_min || y_max < self.y_min || self.y_max < y_min {
            return;
        }
        // 領域内のすべての点に作用を適用できる場合
        if x_min <= self.x_min && self.x_max <= x_max && y_min <= self.y_min && self.y_max <= y_max
        {
            self.lazy.composition(action);
            return;
        }
        if let Some(left) = &mut self.left {
            left.add_range(x_range, y_range, action);
        }
        if let Some(right) = &mut self.right {
            right.add_range(x_range, y_range, action);
        }
    }

    pub fn get_composition(&self, x: T, y: T) -> A {
        if x < self.x_min || self.x_max < x || y < self.y_min || self.y_max < y {
            return A::id_map();
        }
        // 1点のみの葉ノード
        if self.x_min == self.x_max
            && self.y_min == self.y_max
            && self.x_min == x
            && self.y_min == y
        {
            return self.lazy.clone();
        }
        let mut res = self.lazy.clone();
        if let Some(left) = &self.left {
            res.composition(&left.get_composition(x, y));
        }
        if let Some(right) = &self.right {
            res.composition(&right.get_composition(x, y));
        }
        res
    }
}
