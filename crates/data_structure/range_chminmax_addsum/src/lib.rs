//! Range chmin/chMax, add, update query  
//! Range min/max, sum query  
//! に対応する(i64型)

use lazy_segtree_beats::{BeatsNode, LazySegtreeBeats};
use std::cmp::Ordering;
use InnerMonoid::*;
use RangeActions::*;

/// 内部で持つモノイド 遅延情報は別に持つ
#[derive(Debug, Clone, Copy)]
pub enum InnerMonoid {
    ZeroValue,
    OneValue {
        val: i64,
        len: usize,
    },
    TwoValues {
        min: i64,
        min_cnt: usize,
        max: i64,
        max_cnt: usize,
    },
    ThreeOrMoreValues {
        min: i64,
        min_cnt: usize,
        min_second: i64,
        max: i64,
        max_cnt: usize,
        max_second: i64,
        len: usize,
        sum: i64,
    },
}

impl InnerMonoid {
    pub fn get_sum(&self) -> i64 {
        match self {
            ZeroValue => 0,
            OneValue { val, len } => *val * *len as i64,
            TwoValues {
                min,
                min_cnt,
                max,
                max_cnt,
            } => min * *min_cnt as i64 + max * *max_cnt as i64,
            ThreeOrMoreValues { sum, .. } => *sum,
        }
    }
    pub fn get_max(&self) -> i64 {
        match self {
            ZeroValue => i64::MIN,
            OneValue { val: max, .. } | TwoValues { max, .. } | ThreeOrMoreValues { max, .. } => {
                *max
            }
        }
    }
    fn get_max_cnt(&self) -> usize {
        match self {
            ZeroValue => 0,
            OneValue { len: max_cnt, .. }
            | TwoValues { max_cnt, .. }
            | ThreeOrMoreValues { max_cnt, .. } => *max_cnt,
        }
    }
    pub fn get_min(&self) -> i64 {
        match self {
            ZeroValue => i64::MAX,
            OneValue { val: min, .. } | TwoValues { min, .. } | ThreeOrMoreValues { min, .. } => {
                *min
            }
        }
    }
    fn get_min_cnt(&self) -> usize {
        match self {
            ZeroValue => 0,
            OneValue { len: min_cnt, .. }
            | TwoValues { min_cnt, .. }
            | ThreeOrMoreValues { min_cnt, .. } => *min_cnt,
        }
    }
    fn len(&self) -> usize {
        match self {
            ZeroValue => 0,
            OneValue { len, .. } | ThreeOrMoreValues { len, .. } => *len,
            TwoValues {
                min_cnt, max_cnt, ..
            } => min_cnt + max_cnt,
        }
    }
    fn get_min_second(&self) -> Option<i64> {
        match self {
            ZeroValue | OneValue { .. } => None,
            TwoValues {
                max: min_second, ..
            }
            | ThreeOrMoreValues { min_second, .. } => Some(*min_second),
        }
    }
    fn get_max_second(&self) -> Option<i64> {
        match self {
            ZeroValue | OneValue { .. } => None,
            TwoValues {
                min: max_second, ..
            }
            | ThreeOrMoreValues { max_second, .. } => Some(*max_second),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// (モノイド、遅延したAdd)
pub struct InnerNode(InnerMonoid, i64);

#[derive(Debug, Clone, Copy)]
pub enum RangeActions {
    /// Chmin
    UpperBound(i64),
    /// ChMax
    LowerBound(i64),
    AddAll(i64),
    Update(i64),
}

impl BeatsNode for InnerNode {
    type Action = RangeActions;
    fn id_node() -> Self {
        Self(ZeroValue, 0)
    }
    fn binary_operation(lhs: &Self, rhs: &Self) -> Self {
        let monoid = match (lhs.0, rhs.0) {
            (ZeroValue, node) | (node, ZeroValue) => node,
            (
                OneValue {
                    val: l_val,
                    len: l_len,
                },
                OneValue {
                    val: r_val,
                    len: r_len,
                },
            ) => match l_val.cmp(&r_val) {
                Ordering::Equal => OneValue {
                    val: l_val,
                    len: l_len + r_len,
                },
                Ordering::Less => TwoValues {
                    min: l_val,
                    min_cnt: l_len,
                    max: r_val,
                    max_cnt: r_len,
                },
                Ordering::Greater => TwoValues {
                    min: r_val,
                    min_cnt: r_len,
                    max: l_val,
                    max_cnt: l_len,
                },
            },
            (
                OneValue { val, len },
                TwoValues {
                    min,
                    min_cnt,
                    max,
                    max_cnt,
                },
            )
            | (
                TwoValues {
                    min,
                    min_cnt,
                    max,
                    max_cnt,
                },
                OneValue { val, len },
            ) => {
                if val == min {
                    TwoValues {
                        min,
                        min_cnt: min_cnt + len,
                        max,
                        max_cnt,
                    }
                } else if val == max {
                    TwoValues {
                        min,
                        min_cnt,
                        max,
                        max_cnt: max_cnt + len,
                    }
                } else {
                    let mid = {
                        let mut ary = [min, val, max];
                        ary.sort_unstable();
                        ary[1]
                    };
                    ThreeOrMoreValues {
                        min: val.min(min),
                        min_cnt: if min < val { min_cnt } else { len },
                        min_second: mid,
                        max: val.max(max),
                        max_cnt: if max > val { max_cnt } else { len },
                        max_second: mid,
                        len: min_cnt + max_cnt + len,
                        sum: val * len as i64 + min * min_cnt as i64 + max * max_cnt as i64,
                    }
                }
            }
            (
                TwoValues {
                    min: l_min,
                    min_cnt: l_min_cnt,
                    max: l_max,
                    max_cnt: l_max_cnt,
                },
                TwoValues {
                    min: r_min,
                    min_cnt: r_min_cnt,
                    max: r_max,
                    max_cnt: r_max_cnt,
                },
            ) => {
                if l_min == r_min && l_max == r_max {
                    TwoValues {
                        min: l_min,
                        min_cnt: l_min_cnt + r_min_cnt,
                        max: l_max,
                        max_cnt: l_max_cnt + r_max_cnt,
                    }
                } else {
                    let min_cnt = match l_min.cmp(&r_min) {
                        Ordering::Equal => l_min_cnt + r_min_cnt,
                        Ordering::Less => l_min_cnt,
                        Ordering::Greater => r_min_cnt,
                    };
                    let max_cnt = match l_max.cmp(&r_max) {
                        Ordering::Equal => l_max_cnt + r_max_cnt,
                        Ordering::Greater => l_max_cnt,
                        Ordering::Less => r_max_cnt,
                    };
                    let mut vals = vec![l_min, l_max, r_min, r_max];
                    vals.sort_unstable();
                    vals.dedup();
                    ThreeOrMoreValues {
                        min: l_min.min(r_min),
                        min_cnt,
                        min_second: vals[1],
                        max: l_max.max(r_max),
                        max_cnt,
                        max_second: vals[vals.len() - 2],
                        len: lhs.0.len() + rhs.0.len(),
                        sum: lhs.0.get_sum() + rhs.0.get_sum(),
                    }
                }
            }
            // あとはどうやっても3種必要
            _ => {
                let mut vals = vec![
                    Some(lhs.0.get_min()),
                    Some(rhs.0.get_min()),
                    lhs.0.get_min_second(),
                    rhs.0.get_min_second(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
                vals.sort_unstable();
                vals.dedup();
                let (min, min_second) = (vals[0], vals[1]);
                let mut vals = vec![
                    Some(lhs.0.get_max()),
                    Some(rhs.0.get_max()),
                    lhs.0.get_max_second(),
                    rhs.0.get_max_second(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
                vals.sort_unstable();
                vals.dedup();
                vals.reverse();
                let (max, max_second) = (vals[0], vals[1]);
                ThreeOrMoreValues {
                    min,
                    min_cnt: match lhs.0.get_min().cmp(&rhs.0.get_min()) {
                        Ordering::Equal => lhs.0.get_min_cnt() + rhs.0.get_min_cnt(),
                        Ordering::Less => lhs.0.get_min_cnt(),
                        Ordering::Greater => rhs.0.get_min_cnt(),
                    },
                    min_second,
                    max,
                    max_cnt: match lhs.0.get_max().cmp(&rhs.0.get_max()) {
                        Ordering::Equal => lhs.0.get_max_cnt() + rhs.0.get_max_cnt(),
                        Ordering::Greater => lhs.0.get_max_cnt(),
                        Ordering::Less => rhs.0.get_max_cnt(),
                    },
                    max_second,
                    len: lhs.0.len() + rhs.0.len(),
                    sum: lhs.0.get_sum() + rhs.0.get_sum(),
                }
            }
        };
        Self(monoid, 0)
    }
    fn apply(&mut self, action: &Self::Action) -> bool {
        if matches!(self.0, ZeroValue) {
            return true;
        }
        match *action {
            Update(val) => {
                *self = Self(
                    match self.0 {
                        ZeroValue => ZeroValue,
                        OneValue { len, .. } | ThreeOrMoreValues { len, .. } => {
                            OneValue { val, len }
                        }
                        TwoValues {
                            min_cnt, max_cnt, ..
                        } => OneValue {
                            val,
                            len: min_cnt + max_cnt,
                        },
                    },
                    0,
                )
            }
            AddAll(add) => {
                self.1 += add;
                self.0 = match self.0 {
                    ZeroValue => ZeroValue,
                    OneValue { val, len } => OneValue {
                        val: val + add,
                        len,
                    },
                    TwoValues {
                        min,
                        min_cnt,
                        max,
                        max_cnt,
                    } => TwoValues {
                        min: min + add,
                        min_cnt,
                        max: max + add,
                        max_cnt,
                    },
                    ThreeOrMoreValues {
                        min,
                        min_cnt,
                        min_second,
                        max,
                        max_cnt,
                        max_second,
                        len,
                        sum,
                    } => ThreeOrMoreValues {
                        min: min + add,
                        min_cnt,
                        min_second: min_second + add,
                        max: max + add,
                        max_cnt,
                        max_second: max_second + add,
                        len,
                        sum: sum + add * len as i64,
                    },
                };
            }
            LowerBound(lb) => {
                if self.0.get_min() < lb {
                    let new_monoid = match self.0 {
                        ZeroValue => ZeroValue,
                        OneValue { val, len } => OneValue {
                            val: val.max(lb),
                            len,
                        },
                        TwoValues {
                            min_cnt,
                            max,
                            max_cnt,
                            ..
                        } => {
                            if max <= lb {
                                OneValue {
                                    val: lb,
                                    len: min_cnt + max_cnt,
                                }
                            } else {
                                TwoValues {
                                    min: lb,
                                    min_cnt,
                                    max,
                                    max_cnt,
                                }
                            }
                        }
                        ThreeOrMoreValues {
                            max,
                            max_cnt,
                            max_second,
                            len,
                            min_second,
                            min,
                            min_cnt,
                            sum,
                        } => {
                            if max <= lb {
                                OneValue { val: lb, len }
                            } else if max_second <= lb {
                                TwoValues {
                                    min: lb,
                                    min_cnt: len - max_cnt,
                                    max,
                                    max_cnt,
                                }
                            } else if lb < min_second {
                                ThreeOrMoreValues {
                                    min: lb,
                                    min_cnt,
                                    min_second,
                                    max,
                                    max_cnt,
                                    max_second,
                                    len,
                                    sum: sum + (lb - min) * min_cnt as i64,
                                }
                            } else {
                                return false;
                            }
                        }
                    };
                    self.0 = new_monoid;
                }
            }
            UpperBound(ub) => {
                if self.0.get_max() > ub {
                    let new_monoid = match self.0 {
                        ZeroValue => ZeroValue,
                        OneValue { val, len } => OneValue {
                            val: val.min(ub),
                            len,
                        },
                        TwoValues {
                            min,
                            min_cnt,
                            max_cnt,
                            ..
                        } => {
                            if ub <= min {
                                OneValue {
                                    val: ub,
                                    len: min_cnt + max_cnt,
                                }
                            } else {
                                TwoValues {
                                    min,
                                    min_cnt,
                                    max: ub,
                                    max_cnt,
                                }
                            }
                        }
                        ThreeOrMoreValues {
                            min,
                            min_cnt,
                            min_second,
                            len,
                            max,
                            max_cnt,
                            max_second,
                            sum,
                        } => {
                            if ub <= min {
                                OneValue { val: ub, len }
                            } else if ub <= min_second {
                                TwoValues {
                                    min,
                                    min_cnt,
                                    max: ub,
                                    max_cnt: len - min_cnt,
                                }
                            } else if max_second < ub {
                                ThreeOrMoreValues {
                                    max: ub,
                                    max_cnt,
                                    max_second,
                                    min,
                                    min_cnt,
                                    min_second,
                                    len,
                                    sum: sum + (ub - max) * max_cnt as i64,
                                }
                            } else {
                                return false;
                            }
                        }
                    };
                    self.0 = new_monoid;
                }
            }
        }
        true
    }
    fn push(&mut self, child_node_left: &mut Self, child_node_right: &mut Self) {
        if let OneValue { val, .. } = self.0 {
            child_node_left.apply(&Update(val));
            child_node_right.apply(&Update(val));
            return;
        }
        if self.1 != 0 {
            child_node_left.apply(&AddAll(self.1));
            child_node_right.apply(&AddAll(self.1));
            self.1 = 0;
        }
        if self.0.get_max() < child_node_left.0.get_max() {
            assert!(
                child_node_left.apply(&UpperBound(self.0.get_max())),
                "parent:{:?}, left:{:?}",
                self,
                child_node_left
            );
        }
        if self.0.get_max() < child_node_right.0.get_max() {
            assert!(
                child_node_right.apply(&UpperBound(self.0.get_max())),
                "parent:{:?}, right:{:?}",
                self,
                child_node_right
            );
        }
        if self.0.get_min() > child_node_left.0.get_min() {
            assert!(
                child_node_left.apply(&LowerBound(self.0.get_min())),
                "parent:{:?}, left:{:?}",
                self,
                child_node_left
            );
        }
        if self.0.get_min() > child_node_right.0.get_min() {
            assert!(
                child_node_right.apply(&LowerBound(self.0.get_min())),
                "parent:{:?}, right:{:?}",
                self,
                child_node_right
            );
        }
    }
}

use std::ops::RangeBounds;
pub type RangeChminMaxAddSum = LazySegtreeBeats<InnerNode>;
pub trait QueryWrapper {
    fn from_vec(v: Vec<i64>) -> Self;
    fn range_chmin<R: RangeBounds<usize>>(&mut self, range: R, chmin: i64);
    fn range_chmax<R: RangeBounds<usize>>(&mut self, range: R, chmax: i64);
    fn range_update<R: RangeBounds<usize>>(&mut self, range: R, update: i64);
    fn range_add<R: RangeBounds<usize>>(&mut self, range: R, add: i64);
    fn prod_monoid<R: RangeBounds<usize>>(&mut self, range: R) -> InnerMonoid;
}

impl QueryWrapper for RangeChminMaxAddSum {
    fn from_vec(v: Vec<i64>) -> Self {
        Self::from(
            v.into_iter()
                .map(|val| InnerNode(OneValue { val, len: 1 }, 0))
                .collect::<Vec<_>>(),
        )
    }
    fn range_add<R: RangeBounds<usize>>(&mut self, range: R, add: i64) {
        self.apply_range(range, &AddAll(add));
    }
    fn range_chmax<R: RangeBounds<usize>>(&mut self, range: R, chmax: i64) {
        self.apply_range(range, &LowerBound(chmax));
    }
    fn range_chmin<R: RangeBounds<usize>>(&mut self, range: R, chmin: i64) {
        self.apply_range(range, &UpperBound(chmin));
    }
    fn range_update<R: RangeBounds<usize>>(&mut self, range: R, update: i64) {
        self.apply_range(range, &Update(update));
    }
    fn prod_monoid<R: RangeBounds<usize>>(&mut self, range: R) -> InnerMonoid {
        self.prod(range).0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn test() {
        const SIZE: usize = 1000;
        let mut rng = thread_rng();
        let mut vec = (0..SIZE)
            .map(|_| rng.gen_range(-1000..=1000))
            .collect::<Vec<_>>();
        let mut seg = RangeChminMaxAddSum::from_vec(vec.clone());
        for _ in 0..1000 {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let t = rng.gen_range(0..3);
            match t {
                0 => {
                    let chmin = rng.gen_range(-1000..=1000);
                    for i in l..r {
                        vec[i] = vec[i].min(chmin);
                    }
                    seg.range_chmin(l..r, chmin);
                }
                1 => {
                    let chmax = rng.gen_range(-1000..=1000);
                    for i in l..r {
                        vec[i] = vec[i].max(chmax);
                    }
                    seg.range_chmax(l..r, chmax);
                }
                2 => {
                    let add = rng.gen_range(-100..=100);
                    for i in l..r {
                        vec[i] += add;
                    }
                    seg.range_add(l..r, add);
                }
                _ => unreachable!(),
            }
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            assert_eq!(
                seg.prod_monoid(l..r).get_sum(),
                vec[l..r].iter().sum::<i64>()
            );
        }
        for i in 0..SIZE {
            assert_eq!(seg.prod_monoid(i..i + 1).get_sum(), vec[i]);
        }
    }
}
