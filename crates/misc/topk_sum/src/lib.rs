//! 大きい または 小さい 順に上位K(固定)個の和を求めるデータ構造
use internal_type_traits::Zero;
use std::collections::BTreeMap;
use std::ops::{Add, AddAssign, Neg, SubAssign};

/// 大きい または 小さい 順に上位K個の和を求める  
/// Kが固定されていることを前提としている  
/// 同じ値を複数回挿入可能
pub struct TopKSum<T: Ord, const BIGGER: bool> {
    top_multi_set: BTreeMap<T, usize>,
    left_multi_set: BTreeMap<T, usize>,
    k: usize,
    size: usize,
    sum: T,
}

impl<T, const BIGGER: bool> TopKSum<T, BIGGER>
where
    T: Ord + Copy + Neg<Output = T> + Add<Output = T> + AddAssign + SubAssign + Zero,
{
    pub fn new(k: usize) -> Self {
        Self {
            top_multi_set: BTreeMap::new(),
            left_multi_set: BTreeMap::new(),
            k,
            size: 0,
            sum: T::zero(),
        }
    }

    pub fn insert(&mut self, value: T) {
        let value = if BIGGER { value } else { -value };
        if self.size < self.k {
            *self.top_multi_set.entry(value).or_default() += 1;
            self.sum += value;
        } else if let Some((&smallest_top, _)) = self.top_multi_set.first_key_value() {
            if value > smallest_top {
                // top から smallest_top を left に移動
                let count = self.top_multi_set.get_mut(&smallest_top).unwrap();
                *count -= 1;
                if *count == 0 {
                    self.top_multi_set.remove(&smallest_top);
                }
                *self.left_multi_set.entry(smallest_top).or_default() += 1;
                self.sum -= smallest_top;

                // value を top に追加
                *self.top_multi_set.entry(value).or_default() += 1;
                self.sum += value;
            } else {
                *self.left_multi_set.entry(value).or_default() += 1;
            }
        } else {
            *self.left_multi_set.entry(value).or_default() += 1;
        }
        self.size += 1;
    }

    /// value が存在したら削除して true を返す。存在しなければ false を返す。
    pub fn remove(&mut self, value: T) -> bool {
        let value = if BIGGER { value } else { -value };
        if let Some(count) = self.top_multi_set.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.top_multi_set.remove(&value);
            }
            self.sum -= value;

            // left から最大値を top に移動
            if let Some((&largest_left, _)) = self.left_multi_set.last_key_value() {
                let count = self.left_multi_set.get_mut(&largest_left).unwrap();
                *count -= 1;
                if *count == 0 {
                    self.left_multi_set.remove(&largest_left);
                }
                *self.top_multi_set.entry(largest_left).or_default() += 1;
                self.sum += largest_left;
            }
        } else if let Some(count) = self.left_multi_set.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.left_multi_set.remove(&value);
            }
        } else {
            return false;
        }
        self.size -= 1;
        true
    }

    pub fn sum(&self) -> T {
        if BIGGER {
            self.sum
        } else {
            -self.sum
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    #[test]
    fn test_topk_sum() {
        let mut rng = rand::thread_rng();
        const SIZE: usize = 1000;
        const K: usize = 100;
        let mut topk_smaller_sum = TopKSum::<i64, false>::new(K);
        let mut topk_bigger_sum = TopKSum::<i64, true>::new(K);
        let mut vec = Vec::new();

        for _ in 0..K * 2 {
            let value: i64 = rng.gen_range(-1000..1000);
            topk_smaller_sum.insert(value);
            topk_bigger_sum.insert(value);
            vec.push(value);
        }

        vec.sort();

        assert_eq!(
            topk_smaller_sum.sum(),
            vec.iter().take(K).copied().sum::<i64>()
        );
        assert_eq!(
            topk_bigger_sum.sum(),
            vec.iter().rev().take(K).copied().sum::<i64>()
        );

        for _ in 0..SIZE * 10 {
            let add: bool = rng.gen();
            let value: i64 = rng.gen_range(-1000..1000);
            if add {
                topk_smaller_sum.insert(value);
                topk_bigger_sum.insert(value);
                vec.push(value);
            } else {
                topk_smaller_sum.remove(value);
                topk_bigger_sum.remove(value);
                if let Some(pos) = vec.iter().position(|&x| x == value) {
                    vec.remove(pos);
                }
            }
            vec.sort();
            assert_eq!(
                topk_smaller_sum.sum(),
                vec.iter().take(K).copied().sum::<i64>()
            );
            assert_eq!(
                topk_bigger_sum.sum(),
                vec.iter().rev().take(K).copied().sum::<i64>()
            );
        }
    }
}
