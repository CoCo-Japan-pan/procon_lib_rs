//! Range chmin, chmax, add, update query  
//! Range min/max, sum query  
//! に対応する(i64型)  
//! break/tag condition を明示的に記述する実装

use internal_bits::ceil_log2;
use std::cmp::Ordering;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct RangeChminMaxAddSum {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    nodes: Vec<Node>,
    lazy_add: Vec<i64>,
}

impl RangeChminMaxAddSum {
    pub fn range_chmin<R: RangeBounds<usize>>(&mut self, range: R, chmin: i64) {
        self.apply_range(range, QueryType::Chmin(chmin));
    }
    pub fn range_chmax<R: RangeBounds<usize>>(&mut self, range: R, chmax: i64) {
        self.apply_range(range, QueryType::Chmax(chmax));
    }
    pub fn range_add<R: RangeBounds<usize>>(&mut self, range: R, add: i64) {
        self.apply_range(range, QueryType::Add(add));
    }
    pub fn range_update<R: RangeBounds<usize>>(&mut self, range: R, update: i64) {
        self.apply_range(range, QueryType::Update(update));
    }
    /// rangeの長さが0の場合は`i64::MIN`を返す
    pub fn prod_max<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
        self.fold::<R, { Self::PROD_MAX }>(range)
    }
    /// rangeの長さが0の場合は`i64::MAX`を返す
    pub fn prod_min<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
        self.fold::<R, { Self::PROD_MIN }>(range)
    }
    pub fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
        self.fold::<R, { Self::PROD_SUM }>(range)
    }
}

#[derive(Debug, Clone, Copy)]
enum QueryType {
    Chmin(i64),
    Chmax(i64),
    Add(i64),
    Update(i64),
}

impl From<Vec<i64>> for RangeChminMaxAddSum {
    fn from(v: Vec<i64>) -> Self {
        let range_size = v.len();
        let log = ceil_log2(range_size as u32) as usize;
        let leaf_size = 1 << log;
        let mut data = vec![Node::default(); leaf_size];
        let mut nodes = v.into_iter().map(Node::new).collect();
        data.append(&mut nodes);
        data.append(&mut vec![Node::default(); leaf_size - range_size]);
        let lazy_add = vec![0; leaf_size];
        let mut ret = Self {
            range_size,
            leaf_size,
            log,
            nodes: data,
            lazy_add,
        };
        for i in (1..leaf_size).rev() {
            ret.update(i);
        }
        ret
    }
}

impl RangeChminMaxAddSum {
    const PROD_MIN: u8 = 0;
    const PROD_MAX: u8 = 1;
    const PROD_SUM: u8 = 2;
    fn get_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => self.range_size,
        };
        assert!(start <= end && end <= self.range_size);
        (start, end)
    }
    fn apply_range<R: RangeBounds<usize>>(&mut self, range: R, query: QueryType) {
        let (mut l, mut r) = self.get_range(range);
        if l == r {
            return;
        }
        l += self.leaf_size;
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l_copy = l;
            let r_copy = r;
            while l < r {
                if l & 1 != 0 {
                    self.apply(l, query);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.apply(r, query);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l_copy;
            r = r_copy;
        }
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }
    fn update(&mut self, i: usize) {
        self.nodes[i] = Node::binary_operation(&self.nodes[i << 1], &self.nodes[(i << 1) | 1]);
    }
    fn subtree_chmin(&mut self, i: usize, chmin: i64) {
        // break condition
        if self.nodes[i].max() <= chmin {
            return;
        }
        // tag condition
        let tag_condition = match self.nodes[i] {
            Node::Unit | Node::AllSame { .. } => true,
            Node::TwoOrMore {
                min_second,
                max_second,
                max,
                ..
            } => chmin <= min_second || (max_second < chmin && chmin < max),
        };
        if tag_condition {
            self.nodes[i].tag_chmin(chmin);
        } else {
            self.push(i);
            self.subtree_chmin(i << 1, chmin);
            self.subtree_chmin((i << 1) | 1, chmin);
            self.update(i);
        }
    }
    fn subtree_chmax(&mut self, i: usize, chmax: i64) {
        // break condition
        if self.nodes[i].min() >= chmax {
            return;
        }
        // tag condition
        let tag_condition = match self.nodes[i] {
            Node::Unit | Node::AllSame { .. } => true,
            Node::TwoOrMore {
                min_second,
                max_second,
                min,
                ..
            } => chmax >= max_second || (min < chmax && chmax < min_second),
        };
        if tag_condition {
            self.nodes[i].tag_chmax(chmax);
        } else {
            self.push(i);
            self.subtree_chmax(i << 1, chmax);
            self.subtree_chmax((i << 1) | 1, chmax);
            self.update(i);
        }
    }
    fn apply(&mut self, i: usize, query: QueryType) {
        match query {
            QueryType::Chmin(chmin) => self.subtree_chmin(i, chmin),
            QueryType::Chmax(chmax) => self.subtree_chmax(i, chmax),
            QueryType::Add(add) => {
                if i < self.leaf_size {
                    self.lazy_add[i] += add;
                }
                self.nodes[i].tag_add(add);
            }
            QueryType::Update(update) => {
                if i < self.leaf_size {
                    self.lazy_add[i] = 0;
                }
                self.nodes[i].tag_update(update);
            }
        }
    }
    fn push(&mut self, i: usize) {
        let lazy = self.lazy_add[i];
        self.lazy_add[i] = 0;
        let left = i << 1;
        let right = (i << 1) | 1;
        if let Node::AllSame { value, .. } = self.nodes[i] {
            if !matches!(self.nodes[left], Node::AllSame { value: l, .. } if l == value) {
                self.nodes[left].tag_update(value);
            }
            if !matches!(self.nodes[right], Node::AllSame { value: r, .. } if r == value) {
                self.nodes[right].tag_update(value);
            }
            return;
        }
        // 子ノードにtag/break を適用
        if lazy != 0 {
            self.nodes[left].tag_add(lazy);
            if left < self.leaf_size {
                self.lazy_add[left] += lazy;
            }
            self.nodes[right].tag_add(lazy);
            if right < self.leaf_size {
                self.lazy_add[right] += lazy;
            }
        }
        let cur_max = self.nodes[i].max();
        let cur_min = self.nodes[i].min();
        if cur_max < self.nodes[left].max() {
            self.nodes[left].tag_chmin(cur_max);
        }
        if cur_min > self.nodes[left].min() {
            self.nodes[left].tag_chmax(cur_min);
        }
        if cur_max < self.nodes[right].max() {
            self.nodes[right].tag_chmin(cur_max);
        }
        if cur_min > self.nodes[right].min() {
            self.nodes[right].tag_chmax(cur_min);
        }
    }
    fn id_element<const PRODTYPE: u8>() -> i64 {
        match PRODTYPE {
            Self::PROD_MIN => i64::MAX,
            Self::PROD_MAX => i64::MIN,
            Self::PROD_SUM => 0,
            _ => unreachable!(),
        }
    }
    fn fold_func<const PRODTYPE: u8>(node: &Node, val: i64) -> i64 {
        match PRODTYPE {
            Self::PROD_MIN => node.min().min(val),
            Self::PROD_MAX => node.max().max(val),
            Self::PROD_SUM => node.sum() + val,
            _ => unreachable!(),
        }
    }
    /// binary_operation を用いるより、欲しい値だけを求めるfoldを用いる方が速い
    fn fold<R: RangeBounds<usize>, const PRODTYPE: u8>(&mut self, range: R) -> i64 {
        let (mut l, mut r) = self.get_range(range);
        if l == r {
            return Self::id_element::<PRODTYPE>();
        }
        l += self.leaf_size;
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let mut sml = Self::id_element::<PRODTYPE>();
        let mut smr = Self::id_element::<PRODTYPE>();
        while l < r {
            if l & 1 != 0 {
                sml = Self::fold_func::<PRODTYPE>(&self.nodes[l], sml);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = Self::fold_func::<PRODTYPE>(&self.nodes[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        match PRODTYPE {
            Self::PROD_MIN => sml.min(smr),
            Self::PROD_MAX => sml.max(smr),
            Self::PROD_SUM => sml + smr,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Unit,
    AllSame {
        value: i64,
        len: usize,
        sum: i64,
    },
    TwoOrMore {
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

impl Default for Node {
    fn default() -> Self {
        Self::Unit
    }
}

impl Node {
    fn new(val: i64) -> Self {
        Self::AllSame {
            value: val,
            len: 1,
            sum: val,
        }
    }
    fn binary_operation(lhs: &Self, rhs: &Self) -> Self {
        match (lhs, rhs) {
            (Self::Unit, node) | (node, Self::Unit) => *node,
            (
                Self::AllSame {
                    value: v1,
                    len: l1,
                    sum: s1,
                },
                Self::AllSame {
                    value: v2,
                    len: l2,
                    sum: s2,
                },
            ) => match v1.cmp(v2) {
                Ordering::Less => Self::TwoOrMore {
                    min: *v1,
                    min_cnt: *l1,
                    min_second: *v2,
                    max: *v2,
                    max_cnt: *l2,
                    max_second: *v1,
                    len: l1 + l2,
                    sum: s1 + s2,
                },
                Ordering::Equal => Self::AllSame {
                    value: *v1,
                    len: l1 + l2,
                    sum: s1 + s2,
                },
                Ordering::Greater => Self::TwoOrMore {
                    min: *v2,
                    min_cnt: *l2,
                    min_second: *v1,
                    max: *v1,
                    max_cnt: *l1,
                    max_second: *v2,
                    len: l1 + l2,
                    sum: s1 + s2,
                },
            },
            _ => {
                let l_max = lhs.max();
                let r_max = rhs.max();
                let max = l_max.max(r_max);
                let max_cnt = lhs.max_cnt() * (l_max == max) as usize
                    + rhs.max_cnt() * (r_max == max) as usize;
                let max_second = *match (lhs, rhs) {
                    (
                        Self::AllSame { value, .. },
                        Self::TwoOrMore {
                            max, max_second, ..
                        },
                    )
                    | (
                        Self::TwoOrMore {
                            max, max_second, ..
                        },
                        Self::AllSame { value, .. },
                    ) => match max.cmp(value) {
                        Ordering::Greater => value.max(max_second),
                        Ordering::Equal => max_second,
                        Ordering::Less => max,
                    },
                    (
                        Self::TwoOrMore {
                            max: l_max,
                            max_second: l_max_second,
                            ..
                        },
                        Self::TwoOrMore {
                            max: r_max,
                            max_second: r_max_second,
                            ..
                        },
                    ) => match l_max.cmp(r_max) {
                        Ordering::Greater => r_max.max(l_max_second),
                        Ordering::Equal => l_max_second.max(r_max_second),
                        Ordering::Less => l_max.max(r_max_second),
                    },
                    _ => unreachable!(),
                };
                let l_min = lhs.min();
                let r_min = rhs.min();
                let min = l_min.min(r_min);
                let min_cnt = lhs.min_cnt() * (l_min == min) as usize
                    + rhs.min_cnt() * (r_min == min) as usize;
                let min_second = *match (lhs, rhs) {
                    (
                        Self::AllSame { value, .. },
                        Self::TwoOrMore {
                            min, min_second, ..
                        },
                    )
                    | (
                        Self::TwoOrMore {
                            min, min_second, ..
                        },
                        Self::AllSame { value, .. },
                    ) => match min.cmp(value) {
                        Ordering::Less => value.min(min_second),
                        Ordering::Equal => min_second,
                        Ordering::Greater => min,
                    },
                    (
                        Self::TwoOrMore {
                            min: l_min,
                            min_second: l_min_second,
                            ..
                        },
                        Self::TwoOrMore {
                            min: r_min,
                            min_second: r_min_second,
                            ..
                        },
                    ) => match l_min.cmp(r_min) {
                        Ordering::Less => r_min.min(l_min_second),
                        Ordering::Equal => l_min_second.min(r_min_second),
                        Ordering::Greater => l_min.min(r_min_second),
                    },
                    _ => unreachable!(),
                };
                Self::TwoOrMore {
                    min,
                    min_cnt,
                    min_second,
                    max,
                    max_cnt,
                    max_second,
                    len: lhs.len() + rhs.len(),
                    sum: lhs.sum() + rhs.sum(),
                }
            }
        }
    }
    fn sum(&self) -> i64 {
        match self {
            Self::Unit => 0,
            Self::AllSame { sum, .. } | Self::TwoOrMore { sum, .. } => *sum,
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::AllSame { len, .. } | Self::TwoOrMore { len, .. } => *len,
        }
    }
    fn max(&self) -> i64 {
        match self {
            Self::Unit => i64::MIN,
            Self::AllSame { value, .. } => *value,
            Self::TwoOrMore { max, .. } => *max,
        }
    }
    fn min(&self) -> i64 {
        match self {
            Self::Unit => i64::MAX,
            Self::AllSame { value, .. } => *value,
            Self::TwoOrMore { min, .. } => *min,
        }
    }
    fn max_cnt(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::AllSame { len: max_cnt, .. } | Self::TwoOrMore { max_cnt, .. } => *max_cnt,
        }
    }
    fn min_cnt(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::AllSame { len: min_cnt, .. } | Self::TwoOrMore { min_cnt, .. } => *min_cnt,
        }
    }
    fn tag_add(&mut self, add: i64) {
        match self {
            Self::Unit => {}
            Self::AllSame { value, len, sum } => {
                *value += add;
                *sum += add * *len as i64;
            }
            Self::TwoOrMore {
                min,
                min_second,
                max,
                max_second,
                len,
                sum,
                ..
            } => {
                *min += add;
                *min_second += add;
                *max += add;
                *max_second += add;
                *sum += add * *len as i64;
            }
        }
    }

    fn tag_update(&mut self, update: i64) {
        match self {
            Self::Unit => {}
            Self::AllSame { len, .. } | Self::TwoOrMore { len, .. } => {
                *self = Self::AllSame {
                    value: update,
                    len: *len,
                    sum: update * *len as i64,
                }
            }
        }
    }

    fn tag_chmin(&mut self, chmin: i64) {
        match self {
            Self::Unit => {}
            Self::AllSame { value, .. } => {
                if chmin < *value {
                    self.tag_update(chmin);
                } else {
                    unreachable!("tag condition for chmin is not satisfied!!!");
                }
            }
            Self::TwoOrMore {
                min,
                min_cnt,
                min_second,
                max,
                max_cnt,
                max_second,
                len,
                sum,
            } => {
                if chmin < *min {
                    self.tag_update(chmin);
                } else if chmin <= *min_second {
                    // (min, chmin) の2種類のみになる
                    *min_second = chmin;
                    *max_cnt = *len - *min_cnt;
                    *max_second = *min;
                    *max = chmin;
                    *sum = *min * *min_cnt as i64 + chmin * *max_cnt as i64;
                } else if *max_second < chmin && chmin < *max {
                    *sum -= (*max - chmin) * *max_cnt as i64;
                    *max = chmin;
                } else {
                    unreachable!("tag condition for chmin is not satisfied!!!");
                }
            }
        }
    }

    fn tag_chmax(&mut self, chmax: i64) {
        match self {
            Self::Unit => {}
            Self::AllSame { value, .. } => {
                if chmax > *value {
                    self.tag_update(chmax);
                } else {
                    unreachable!("tag condition for chmax is not satisfied!!!");
                }
            }
            Self::TwoOrMore {
                min,
                min_cnt,
                min_second,
                max,
                max_cnt,
                max_second,
                len,
                sum,
            } => {
                if chmax > *max {
                    self.tag_update(chmax);
                } else if chmax >= *max_second {
                    // (chmax, max) の2種類のみになる
                    *max_second = chmax;
                    *min_cnt = *len - *max_cnt;
                    *min_second = *max;
                    *min = chmax;
                    *sum = chmax * *min_cnt as i64 + *max * *max_cnt as i64;
                } else if *min < chmax && chmax < *min_second {
                    *sum -= (*min - chmax) * *min_cnt as i64;
                    *min = chmax;
                } else {
                    unreachable!("tag condition for chmax is not satisfied!!!");
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn test_raq_rsq() {
        use raq_rsq::RAQRSQ;
        let mut rng = thread_rng();
        const SIZE: usize = 100_000;
        const MAX: i64 = 10000;
        const MIN: i64 = -10000;
        let init_vec = (0..SIZE)
            .map(|_| rng.gen_range(-MIN..=MAX))
            .collect::<Vec<_>>();
        let mut ras = RAQRSQ::new(SIZE, 0_i64);
        for (i, &v) in init_vec.iter().enumerate() {
            ras.add(i..=i, v);
        }
        let mut seg = RangeChminMaxAddSum::from(init_vec);
        for _ in 0..1000 {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            let v = rng.gen_range(MIN..=MAX);
            ras.add(l..r, v);
            seg.range_add(l..r, v);
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            assert_eq!(ras.sum(l..r), seg.prod_sum(l..r));
        }
        for i in 0..SIZE {
            assert_eq!(ras.sum(i..=i), seg.prod_sum(i..=i));
        }
    }
    #[test]
    fn test_chmin_max_add() {
        let mut rng = thread_rng();
        const SIZE: usize = 10000;
        const MIN: i64 = -10000;
        const MAX: i64 = 10000;
        let mut data = (0..SIZE)
            .map(|_| rng.gen_range(MIN..=MAX))
            .collect::<Vec<_>>();
        let mut seg = RangeChminMaxAddSum::from(data.clone());
        for _ in 0..1000 {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            let v = rng.gen_range(MIN..=MAX);
            seg.range_chmin(l..r, v);
            for i in l..r {
                data[i] = data[i].min(v);
            }
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            let v = rng.gen_range(MIN..=MAX);
            seg.range_chmax(l..r, v);
            for i in l..r {
                data[i] = data[i].max(v);
            }
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            let v = rng.gen_range(MIN..=MAX);
            seg.range_add(l..r, v);
            for i in l..r {
                data[i] += v;
            }
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            let v = rng.gen_range(MIN..=MAX);
            seg.range_update(l..r, v);
            for i in l..r {
                data[i] = v;
            }
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..=SIZE);
            assert_eq!(
                *data[l..r].iter().min().unwrap_or(&i64::MAX),
                seg.prod_min(l..r)
            );
            assert_eq!(
                *data[l..r].iter().max().unwrap_or(&i64::MIN),
                seg.prod_max(l..r)
            );
            assert_eq!(data[l..r].iter().sum::<i64>(), seg.prod_sum(l..r));
        }
        for i in 0..SIZE {
            assert_eq!(data[i], seg.prod_sum(i..=i));
        }
    }
}
