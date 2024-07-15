//! Li Chao treeによるCHT  
//! i64型で収まる前提  
//! ax + b の直線群を追加して、特定のxにおける最小値または最大値をlogNで求める  
//! クエリで聞かれるx座標達が既知(オフライン)、またはその範囲が10^5ぐらいに収まっている場合に限る  
//! [CHT](https://hcpc-hokudai.github.io/archive/algorithm_convex_hull_trick_001.pdf)
use internal_bits::ceil_log2;
use std::ops::{Bound::*, RangeBounds};

/// 最大値クエリと最小値クエリの両方に対応するために便宜的に導入したトレイト
pub trait Compare {
    fn identity() -> i64;
    /// lhsをrhsで更新するべきならtrue
    fn update(lhs: i64, rhs: i64) -> bool;
}

#[derive(Debug)]
pub enum MaxCompare {}
impl Compare for MaxCompare {
    fn identity() -> i64 {
        i64::MIN
    }
    #[inline]
    fn update(lhs: i64, rhs: i64) -> bool {
        lhs < rhs
    }
}
#[derive(Debug)]
pub enum MinCompare {}
impl Compare for MinCompare {
    fn identity() -> i64 {
        i64::MAX
    }
    #[inline]
    fn update(lhs: i64, rhs: i64) -> bool {
        lhs > rhs
    }
}

/// 最大値クエリの場合は`T = MaxCompare`、最小値クエリの場合は`T = MinCompare`  
/// 最大値クエリなら`i64::MIN`, 最小値クエリなら`i64::MAX`で初期化しています
#[derive(Debug)]
pub struct CHTOffline<T: Compare> {
    sorted_points: Vec<i64>,
    log: usize,
    leaf_size: usize,
    line_per_nodes: Vec<(i64, i64)>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Compare> CHTOffline<T> {
    pub fn new(mut points: Vec<i64>) -> Self {
        points.sort_unstable();
        points.dedup();
        let log = ceil_log2(points.len() as u32) as usize;
        let leaf_size = 1 << log;
        // 完全二分木にするために、足りない分はその最大値で埋める
        let max_point = *points.last().unwrap_or(&0);
        points.extend(std::iter::repeat(max_point).take(leaf_size - points.len()));
        Self {
            sorted_points: points,
            log,
            leaf_size,
            line_per_nodes: vec![(0, T::identity()); leaf_size * 2],
            _phantom: std::marker::PhantomData,
        }
    }

    /// xにおける最小値または最大値を求める
    pub fn get(&self, x: i64) -> i64 {
        let mut id = self
            .sorted_points
            .binary_search(&x)
            .expect("x is not in points!!!");
        id += self.leaf_size;
        let mut ret = T::identity();
        while id > 0 {
            let (a, b) = self.line_per_nodes[id];
            let new_num = a * x + b;
            if T::update(ret, new_num) {
                ret = new_num;
            }
            id >>= 1;
        }
        ret
    }

    fn add_line_in_node(&mut self, mut a: i64, mut b: i64, mut node_id: usize) {
        let (mut left, mut right) = {
            let floor_log = 32 - (node_id as u32).leading_zeros() - 1;
            let block_size = 1 << (self.log - floor_log as usize);
            let idx = node_id - (1 << floor_log);
            (idx * block_size, (idx + 1) * block_size)
        };
        // [left, right)で考える
        while right - left > 0 {
            let (cur_a, cur_b) = self.line_per_nodes[node_id];
            // まず完全に上回る、下回る場合
            let left_point = cur_a * self.sorted_points[left] + cur_b;
            let left_new_point = a * self.sorted_points[left] + b;
            let right_point = cur_a * self.sorted_points[right - 1] + cur_b;
            let right_new_point = a * self.sorted_points[right - 1] + b;
            let left_update = T::update(left_point, left_new_point);
            let right_update = T::update(right_point, right_new_point);
            match (left_update, right_update) {
                (true, true) => {
                    self.line_per_nodes[node_id] = (a, b);
                    return;
                }
                (false, false) => {
                    return;
                }
                _ => {}
            }
            let mid = (left + right) / 2;
            if left_update {
                let mid_point = cur_a * self.sorted_points[mid] + cur_b;
                let mid_new_point = a * self.sorted_points[mid] + b;
                let mid_update = T::update(mid_point, mid_new_point);
                if !mid_update {
                    node_id <<= 1;
                    right = mid;
                } else {
                    // 直線を交換
                    self.line_per_nodes[node_id] = (a, b);
                    a = cur_a;
                    b = cur_b;
                    node_id = (node_id << 1) | 1;
                    left = mid;
                }
            } else {
                let mid_point = cur_a * self.sorted_points[mid - 1] + cur_b;
                let mid_new_point = a * self.sorted_points[mid - 1] + b;
                let mid_update = T::update(mid_point, mid_new_point);
                if !mid_update {
                    node_id = (node_id << 1) | 1;
                    left = mid;
                } else {
                    // 直線を交換
                    self.line_per_nodes[node_id] = (a, b);
                    a = cur_a;
                    b = cur_b;
                    node_id <<= 1;
                    right = mid;
                }
            }
        }
    }

    /// 直線`ax + b`を追加する
    #[inline]
    pub fn add_line(&mut self, a: i64, b: i64) {
        self.add_line_in_node(a, b, 1);
    }

    /// 線分 `ax + b (xはrangeの範囲内で有効)` を追加する
    pub fn add_line_segment<R: RangeBounds<i64>>(&mut self, a: i64, b: i64, range: R) {
        // いくつかのノードに分割してそれぞれで処理する
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => self.sorted_points[0],
        };
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => *self.sorted_points.last().unwrap() + 1,
        };
        let mut left = self.sorted_points.partition_point(|&x| x < l);
        let mut right = self.sorted_points.partition_point(|&x| x < r);
        if left == right {
            return;
        }
        left += self.leaf_size;
        right += self.leaf_size;
        while left < right {
            if left & 1 == 1 {
                self.add_line_in_node(a, b, left);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                self.add_line_in_node(a, b, right);
            }
            left >>= 1;
            right >>= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test() {
        fn do_test(point_size: usize) {
            let mut rng = thread_rng();
            let points: Vec<i64> = (0..point_size)
                .map(|_| rng.gen_range(-10000..10000))
                .collect();
            let mut cht_max = CHTOffline::<MaxCompare>::new(points.clone());
            let mut cht_min = CHTOffline::<MinCompare>::new(points.clone());
            let mut lines = vec![];
            for _ in 0..1000 {
                let a = rng.gen_range(-10000..10000);
                let b = rng.gen_range(-10000..10000);
                cht_max.add_line(a, b);
                cht_min.add_line(a, b);
                lines.push((a, b));
            }
            for x in points.iter() {
                let mut max = i64::MIN;
                let mut min = i64::MAX;
                for (a, b) in lines.iter() {
                    max = max.max(a * x + b);
                    min = min.min(a * x + b);
                }
                assert_eq!(cht_max.get(*x), max);
                assert_eq!(cht_min.get(*x), min);
            }
        }
        do_test(10);
        do_test(100);
        do_test(1000);
        do_test(10000);
    }
}
