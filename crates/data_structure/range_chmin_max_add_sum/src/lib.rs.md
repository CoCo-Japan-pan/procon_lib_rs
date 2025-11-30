---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
    title: verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Range chmin, chmax, add, update query  \n//! Range min/max, sum query\
    \  \n//! \u306B\u5BFE\u5FDC\u3059\u308B(i64\u578B)  \n//! break/tag condition\
    \ \u3092\u660E\u793A\u7684\u306B\u8A18\u8FF0\u3059\u308B\u5B9F\u88C5\n\nuse internal_bits::ceil_log2;\n\
    use std::cmp::Ordering;\nuse std::ops::RangeBounds;\n\n#[derive(Debug)]\npub struct\
    \ RangeChminMaxAddSum {\n    range_size: usize,\n    leaf_size: usize,\n    log:\
    \ usize,\n    nodes: Vec<Node>,\n    lazy_add: Vec<i64>,\n}\n\nimpl RangeChminMaxAddSum\
    \ {\n    pub fn range_chmin<R: RangeBounds<usize>>(&mut self, range: R, chmin:\
    \ i64) {\n        self.apply_range(range, QueryType::Chmin(chmin));\n    }\n \
    \   pub fn range_chmax<R: RangeBounds<usize>>(&mut self, range: R, chmax: i64)\
    \ {\n        self.apply_range(range, QueryType::Chmax(chmax));\n    }\n    pub\
    \ fn range_add<R: RangeBounds<usize>>(&mut self, range: R, add: i64) {\n     \
    \   self.apply_range(range, QueryType::Add(add));\n    }\n    pub fn range_update<R:\
    \ RangeBounds<usize>>(&mut self, range: R, update: i64) {\n        self.apply_range(range,\
    \ QueryType::Update(update));\n    }\n    /// range\u306E\u9577\u3055\u304C0\u306E\
    \u5834\u5408\u306F`i64::MIN`\u3092\u8FD4\u3059\n    pub fn prod_max<R: RangeBounds<usize>>(&mut\
    \ self, range: R) -> i64 {\n        self.fold::<R, { Self::PROD_MAX }>(range)\n\
    \    }\n    /// range\u306E\u9577\u3055\u304C0\u306E\u5834\u5408\u306F`i64::MAX`\u3092\
    \u8FD4\u3059\n    pub fn prod_min<R: RangeBounds<usize>>(&mut self, range: R)\
    \ -> i64 {\n        self.fold::<R, { Self::PROD_MIN }>(range)\n    }\n    pub\
    \ fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {\n        self.fold::<R,\
    \ { Self::PROD_SUM }>(range)\n    }\n}\n\n#[derive(Debug, Clone, Copy)]\nenum\
    \ QueryType {\n    Chmin(i64),\n    Chmax(i64),\n    Add(i64),\n    Update(i64),\n\
    }\n\nimpl From<Vec<i64>> for RangeChminMaxAddSum {\n    fn from(v: Vec<i64>) ->\
    \ Self {\n        let range_size = v.len();\n        let log = ceil_log2(range_size\
    \ as u32) as usize;\n        let leaf_size = 1 << log;\n        let mut data =\
    \ vec![Node::default(); leaf_size];\n        let mut nodes = v.into_iter().map(Node::new).collect();\n\
    \        data.append(&mut nodes);\n        data.append(&mut vec![Node::default();\
    \ leaf_size - range_size]);\n        let lazy_add = vec![0; leaf_size];\n    \
    \    let mut ret = Self {\n            range_size,\n            leaf_size,\n \
    \           log,\n            nodes: data,\n            lazy_add,\n        };\n\
    \        for i in (1..leaf_size).rev() {\n            ret.update(i);\n       \
    \ }\n        ret\n    }\n}\n\nimpl RangeChminMaxAddSum {\n    const PROD_MIN:\
    \ u8 = 0;\n    const PROD_MAX: u8 = 1;\n    const PROD_SUM: u8 = 2;\n    fn get_range<R:\
    \ RangeBounds<usize>>(&self, range: R) -> (usize, usize) {\n        use std::ops::Bound::*;\n\
    \        let start = match range.start_bound() {\n            Included(&s) =>\
    \ s,\n            Excluded(&s) => s + 1,\n            Unbounded => 0,\n      \
    \  };\n        let end = match range.end_bound() {\n            Included(&e) =>\
    \ e + 1,\n            Excluded(&e) => e,\n            Unbounded => self.range_size,\n\
    \        };\n        assert!(start <= end && end <= self.range_size);\n      \
    \  (start, end)\n    }\n    fn apply_range<R: RangeBounds<usize>>(&mut self, range:\
    \ R, query: QueryType) {\n        let (mut l, mut r) = self.get_range(range);\n\
    \        if l == r {\n            return;\n        }\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n        for i in (1..=self.log).rev() {\n     \
    \       if ((l >> i) << i) != l {\n                self.push(l >> i);\n      \
    \      }\n            if ((r >> i) << i) != r {\n                self.push((r\
    \ - 1) >> i);\n            }\n        }\n        {\n            let l_copy = l;\n\
    \            let r_copy = r;\n            while l < r {\n                if l\
    \ & 1 != 0 {\n                    self.apply(l, query);\n                    l\
    \ += 1;\n                }\n                if r & 1 != 0 {\n                \
    \    r -= 1;\n                    self.apply(r, query);\n                }\n \
    \               l >>= 1;\n                r >>= 1;\n            }\n          \
    \  l = l_copy;\n            r = r_copy;\n        }\n        for i in 1..=self.log\
    \ {\n            if ((l >> i) << i) != l {\n                self.update(l >> i);\n\
    \            }\n            if ((r >> i) << i) != r {\n                self.update((r\
    \ - 1) >> i);\n            }\n        }\n    }\n    fn update(&mut self, i: usize)\
    \ {\n        self.nodes[i] = Node::binary_operation(&self.nodes[i << 1], &self.nodes[(i\
    \ << 1) | 1]);\n    }\n    fn subtree_chmin(&mut self, i: usize, chmin: i64) {\n\
    \        // break condition\n        if self.nodes[i].max() <= chmin {\n     \
    \       return;\n        }\n        // tag condition\n        let tag_condition\
    \ = match self.nodes[i] {\n            Node::Unit | Node::AllSame { .. } => true,\n\
    \            Node::TwoOrMore {\n                min_second,\n                max_second,\n\
    \                max,\n                ..\n            } => chmin <= min_second\
    \ || (max_second < chmin && chmin < max),\n        };\n        if tag_condition\
    \ {\n            self.nodes[i].tag_chmin(chmin);\n        } else {\n         \
    \   self.push(i);\n            self.subtree_chmin(i << 1, chmin);\n          \
    \  self.subtree_chmin((i << 1) | 1, chmin);\n            self.update(i);\n   \
    \     }\n    }\n    fn subtree_chmax(&mut self, i: usize, chmax: i64) {\n    \
    \    // break condition\n        if self.nodes[i].min() >= chmax {\n         \
    \   return;\n        }\n        // tag condition\n        let tag_condition =\
    \ match self.nodes[i] {\n            Node::Unit | Node::AllSame { .. } => true,\n\
    \            Node::TwoOrMore {\n                min_second,\n                max_second,\n\
    \                min,\n                ..\n            } => chmax >= max_second\
    \ || (min < chmax && chmax < min_second),\n        };\n        if tag_condition\
    \ {\n            self.nodes[i].tag_chmax(chmax);\n        } else {\n         \
    \   self.push(i);\n            self.subtree_chmax(i << 1, chmax);\n          \
    \  self.subtree_chmax((i << 1) | 1, chmax);\n            self.update(i);\n   \
    \     }\n    }\n    fn apply(&mut self, i: usize, query: QueryType) {\n      \
    \  match query {\n            QueryType::Chmin(chmin) => self.subtree_chmin(i,\
    \ chmin),\n            QueryType::Chmax(chmax) => self.subtree_chmax(i, chmax),\n\
    \            QueryType::Add(add) => {\n                if i < self.leaf_size {\n\
    \                    self.lazy_add[i] += add;\n                }\n           \
    \     self.nodes[i].tag_add(add);\n            }\n            QueryType::Update(update)\
    \ => {\n                if i < self.leaf_size {\n                    self.lazy_add[i]\
    \ = 0;\n                }\n                self.nodes[i].tag_update(update);\n\
    \            }\n        }\n    }\n    fn push(&mut self, i: usize) {\n       \
    \ let lazy = self.lazy_add[i];\n        self.lazy_add[i] = 0;\n        let left\
    \ = i << 1;\n        let right = (i << 1) | 1;\n        if let Node::AllSame {\
    \ value, .. } = self.nodes[i] {\n            if !matches!(self.nodes[left], Node::AllSame\
    \ { value: l, .. } if l == value) {\n                self.nodes[left].tag_update(value);\n\
    \            }\n            if !matches!(self.nodes[right], Node::AllSame { value:\
    \ r, .. } if r == value) {\n                self.nodes[right].tag_update(value);\n\
    \            }\n            return;\n        }\n        // \u5B50\u30CE\u30FC\u30C9\
    \u306Btag/break \u3092\u9069\u7528\n        if lazy != 0 {\n            self.nodes[left].tag_add(lazy);\n\
    \            if left < self.leaf_size {\n                self.lazy_add[left] +=\
    \ lazy;\n            }\n            self.nodes[right].tag_add(lazy);\n       \
    \     if right < self.leaf_size {\n                self.lazy_add[right] += lazy;\n\
    \            }\n        }\n        let cur_max = self.nodes[i].max();\n      \
    \  let cur_min = self.nodes[i].min();\n        if cur_max < self.nodes[left].max()\
    \ {\n            self.nodes[left].tag_chmin(cur_max);\n        }\n        if cur_min\
    \ > self.nodes[left].min() {\n            self.nodes[left].tag_chmax(cur_min);\n\
    \        }\n        if cur_max < self.nodes[right].max() {\n            self.nodes[right].tag_chmin(cur_max);\n\
    \        }\n        if cur_min > self.nodes[right].min() {\n            self.nodes[right].tag_chmax(cur_min);\n\
    \        }\n    }\n    fn id_element<const PRODTYPE: u8>() -> i64 {\n        match\
    \ PRODTYPE {\n            Self::PROD_MIN => i64::MAX,\n            Self::PROD_MAX\
    \ => i64::MIN,\n            Self::PROD_SUM => 0,\n            _ => unreachable!(),\n\
    \        }\n    }\n    fn fold_func<const PRODTYPE: u8>(node: &Node, val: i64)\
    \ -> i64 {\n        match PRODTYPE {\n            Self::PROD_MIN => node.min().min(val),\n\
    \            Self::PROD_MAX => node.max().max(val),\n            Self::PROD_SUM\
    \ => node.sum() + val,\n            _ => unreachable!(),\n        }\n    }\n \
    \   /// binary_operation \u3092\u7528\u3044\u308B\u3088\u308A\u3001\u6B32\u3057\
    \u3044\u5024\u3060\u3051\u3092\u6C42\u3081\u308Bfold\u3092\u7528\u3044\u308B\u65B9\
    \u304C\u901F\u3044\n    fn fold<R: RangeBounds<usize>, const PRODTYPE: u8>(&mut\
    \ self, range: R) -> i64 {\n        let (mut l, mut r) = self.get_range(range);\n\
    \        if l == r {\n            return Self::id_element::<PRODTYPE>();\n   \
    \     }\n        l += self.leaf_size;\n        r += self.leaf_size;\n        for\
    \ i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l {\n       \
    \         self.push(l >> i);\n            }\n            if ((r >> i) << i) !=\
    \ r {\n                self.push((r - 1) >> i);\n            }\n        }\n  \
    \      let mut sml = Self::id_element::<PRODTYPE>();\n        let mut smr = Self::id_element::<PRODTYPE>();\n\
    \        while l < r {\n            if l & 1 != 0 {\n                sml = Self::fold_func::<PRODTYPE>(&self.nodes[l],\
    \ sml);\n                l += 1;\n            }\n            if r & 1 != 0 {\n\
    \                r -= 1;\n                smr = Self::fold_func::<PRODTYPE>(&self.nodes[r],\
    \ smr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \        match PRODTYPE {\n            Self::PROD_MIN => sml.min(smr),\n     \
    \       Self::PROD_MAX => sml.max(smr),\n            Self::PROD_SUM => sml + smr,\n\
    \            _ => unreachable!(),\n        }\n    }\n}\n\n#[derive(Debug, Clone,\
    \ Copy)]\nenum Node {\n    Unit,\n    AllSame {\n        value: i64,\n       \
    \ len: usize,\n        sum: i64,\n    },\n    TwoOrMore {\n        min: i64,\n\
    \        min_cnt: usize,\n        min_second: i64,\n        max: i64,\n      \
    \  max_cnt: usize,\n        max_second: i64,\n        len: usize,\n        sum:\
    \ i64,\n    },\n}\n\nimpl Default for Node {\n    fn default() -> Self {\n   \
    \     Self::Unit\n    }\n}\n\nimpl Node {\n    fn new(val: i64) -> Self {\n  \
    \      Self::AllSame {\n            value: val,\n            len: 1,\n       \
    \     sum: val,\n        }\n    }\n    fn binary_operation(lhs: &Self, rhs: &Self)\
    \ -> Self {\n        match (lhs, rhs) {\n            (Self::Unit, node) | (node,\
    \ Self::Unit) => *node,\n            (\n                Self::AllSame {\n    \
    \                value: v1,\n                    len: l1,\n                  \
    \  sum: s1,\n                },\n                Self::AllSame {\n           \
    \         value: v2,\n                    len: l2,\n                    sum: s2,\n\
    \                },\n            ) => match v1.cmp(v2) {\n                Ordering::Less\
    \ => Self::TwoOrMore {\n                    min: *v1,\n                    min_cnt:\
    \ *l1,\n                    min_second: *v2,\n                    max: *v2,\n\
    \                    max_cnt: *l2,\n                    max_second: *v1,\n   \
    \                 len: l1 + l2,\n                    sum: s1 + s2,\n         \
    \       },\n                Ordering::Equal => Self::AllSame {\n             \
    \       value: *v1,\n                    len: l1 + l2,\n                    sum:\
    \ s1 + s2,\n                },\n                Ordering::Greater => Self::TwoOrMore\
    \ {\n                    min: *v2,\n                    min_cnt: *l2,\n      \
    \              min_second: *v1,\n                    max: *v1,\n             \
    \       max_cnt: *l1,\n                    max_second: *v2,\n                \
    \    len: l1 + l2,\n                    sum: s1 + s2,\n                },\n  \
    \          },\n            _ => {\n                let l_max = lhs.max();\n  \
    \              let r_max = rhs.max();\n                let max = l_max.max(r_max);\n\
    \                let max_cnt = lhs.max_cnt() * (l_max == max) as usize\n     \
    \               + rhs.max_cnt() * (r_max == max) as usize;\n                let\
    \ max_second = *match (lhs, rhs) {\n                    (\n                  \
    \      Self::AllSame { value, .. },\n                        Self::TwoOrMore {\n\
    \                            max, max_second, ..\n                        },\n\
    \                    )\n                    | (\n                        Self::TwoOrMore\
    \ {\n                            max, max_second, ..\n                       \
    \ },\n                        Self::AllSame { value, .. },\n                 \
    \   ) => match max.cmp(value) {\n                        Ordering::Greater =>\
    \ value.max(max_second),\n                        Ordering::Equal => max_second,\n\
    \                        Ordering::Less => max,\n                    },\n    \
    \                (\n                        Self::TwoOrMore {\n              \
    \              max: l_max,\n                            max_second: l_max_second,\n\
    \                            ..\n                        },\n                \
    \        Self::TwoOrMore {\n                            max: r_max,\n        \
    \                    max_second: r_max_second,\n                            ..\n\
    \                        },\n                    ) => match l_max.cmp(r_max) {\n\
    \                        Ordering::Greater => r_max.max(l_max_second),\n     \
    \                   Ordering::Equal => l_max_second.max(r_max_second),\n     \
    \                   Ordering::Less => l_max.max(r_max_second),\n             \
    \       },\n                    _ => unreachable!(),\n                };\n   \
    \             let l_min = lhs.min();\n                let r_min = rhs.min();\n\
    \                let min = l_min.min(r_min);\n                let min_cnt = lhs.min_cnt()\
    \ * (l_min == min) as usize\n                    + rhs.min_cnt() * (r_min == min)\
    \ as usize;\n                let min_second = *match (lhs, rhs) {\n          \
    \          (\n                        Self::AllSame { value, .. },\n         \
    \               Self::TwoOrMore {\n                            min, min_second,\
    \ ..\n                        },\n                    )\n                    |\
    \ (\n                        Self::TwoOrMore {\n                            min,\
    \ min_second, ..\n                        },\n                        Self::AllSame\
    \ { value, .. },\n                    ) => match min.cmp(value) {\n          \
    \              Ordering::Less => value.min(min_second),\n                    \
    \    Ordering::Equal => min_second,\n                        Ordering::Greater\
    \ => min,\n                    },\n                    (\n                   \
    \     Self::TwoOrMore {\n                            min: l_min,\n           \
    \                 min_second: l_min_second,\n                            ..\n\
    \                        },\n                        Self::TwoOrMore {\n     \
    \                       min: r_min,\n                            min_second: r_min_second,\n\
    \                            ..\n                        },\n                \
    \    ) => match l_min.cmp(r_min) {\n                        Ordering::Less =>\
    \ r_min.min(l_min_second),\n                        Ordering::Equal => l_min_second.min(r_min_second),\n\
    \                        Ordering::Greater => l_min.min(r_min_second),\n     \
    \               },\n                    _ => unreachable!(),\n               \
    \ };\n                Self::TwoOrMore {\n                    min,\n          \
    \          min_cnt,\n                    min_second,\n                    max,\n\
    \                    max_cnt,\n                    max_second,\n             \
    \       len: lhs.len() + rhs.len(),\n                    sum: lhs.sum() + rhs.sum(),\n\
    \                }\n            }\n        }\n    }\n    fn sum(&self) -> i64\
    \ {\n        match self {\n            Self::Unit => 0,\n            Self::AllSame\
    \ { sum, .. } | Self::TwoOrMore { sum, .. } => *sum,\n        }\n    }\n    fn\
    \ len(&self) -> usize {\n        match self {\n            Self::Unit => 0,\n\
    \            Self::AllSame { len, .. } | Self::TwoOrMore { len, .. } => *len,\n\
    \        }\n    }\n    fn max(&self) -> i64 {\n        match self {\n        \
    \    Self::Unit => i64::MIN,\n            Self::AllSame { value, .. } => *value,\n\
    \            Self::TwoOrMore { max, .. } => *max,\n        }\n    }\n    fn min(&self)\
    \ -> i64 {\n        match self {\n            Self::Unit => i64::MAX,\n      \
    \      Self::AllSame { value, .. } => *value,\n            Self::TwoOrMore { min,\
    \ .. } => *min,\n        }\n    }\n    fn max_cnt(&self) -> usize {\n        match\
    \ self {\n            Self::Unit => 0,\n            Self::AllSame { len: max_cnt,\
    \ .. } | Self::TwoOrMore { max_cnt, .. } => *max_cnt,\n        }\n    }\n    fn\
    \ min_cnt(&self) -> usize {\n        match self {\n            Self::Unit => 0,\n\
    \            Self::AllSame { len: min_cnt, .. } | Self::TwoOrMore { min_cnt, ..\
    \ } => *min_cnt,\n        }\n    }\n    fn tag_add(&mut self, add: i64) {\n  \
    \      match self {\n            Self::Unit => {}\n            Self::AllSame {\
    \ value, len, sum } => {\n                *value += add;\n                *sum\
    \ += add * *len as i64;\n            }\n            Self::TwoOrMore {\n      \
    \          min,\n                min_second,\n                max,\n         \
    \       max_second,\n                len,\n                sum,\n            \
    \    ..\n            } => {\n                *min += add;\n                *min_second\
    \ += add;\n                *max += add;\n                *max_second += add;\n\
    \                *sum += add * *len as i64;\n            }\n        }\n    }\n\
    \n    fn tag_update(&mut self, update: i64) {\n        match self {\n        \
    \    Self::Unit => {}\n            Self::AllSame { len, .. } | Self::TwoOrMore\
    \ { len, .. } => {\n                *self = Self::AllSame {\n                \
    \    value: update,\n                    len: *len,\n                    sum:\
    \ update * *len as i64,\n                }\n            }\n        }\n    }\n\n\
    \    fn tag_chmin(&mut self, chmin: i64) {\n        match self {\n           \
    \ Self::Unit => {}\n            Self::AllSame { value, .. } => {\n           \
    \     if chmin < *value {\n                    self.tag_update(chmin);\n     \
    \           } else {\n                    unreachable!(\"tag condition for chmin\
    \ is not satisfied!!!\");\n                }\n            }\n            Self::TwoOrMore\
    \ {\n                min,\n                min_cnt,\n                min_second,\n\
    \                max,\n                max_cnt,\n                max_second,\n\
    \                len,\n                sum,\n            } => {\n            \
    \    if chmin < *min {\n                    self.tag_update(chmin);\n        \
    \        } else if chmin <= *min_second {\n                    // (min, chmin)\
    \ \u306E2\u7A2E\u985E\u306E\u307F\u306B\u306A\u308B\n                    *min_second\
    \ = chmin;\n                    *max_cnt = *len - *min_cnt;\n                \
    \    *max_second = *min;\n                    *max = chmin;\n                \
    \    *sum = *min * *min_cnt as i64 + chmin * *max_cnt as i64;\n              \
    \  } else if *max_second < chmin && chmin < *max {\n                    *sum -=\
    \ (*max - chmin) * *max_cnt as i64;\n                    *max = chmin;\n     \
    \           } else {\n                    unreachable!(\"tag condition for chmin\
    \ is not satisfied!!!\");\n                }\n            }\n        }\n    }\n\
    \n    fn tag_chmax(&mut self, chmax: i64) {\n        match self {\n          \
    \  Self::Unit => {}\n            Self::AllSame { value, .. } => {\n          \
    \      if chmax > *value {\n                    self.tag_update(chmax);\n    \
    \            } else {\n                    unreachable!(\"tag condition for chmax\
    \ is not satisfied!!!\");\n                }\n            }\n            Self::TwoOrMore\
    \ {\n                min,\n                min_cnt,\n                min_second,\n\
    \                max,\n                max_cnt,\n                max_second,\n\
    \                len,\n                sum,\n            } => {\n            \
    \    if chmax > *max {\n                    self.tag_update(chmax);\n        \
    \        } else if chmax >= *max_second {\n                    // (chmax, max)\
    \ \u306E2\u7A2E\u985E\u306E\u307F\u306B\u306A\u308B\n                    *max_second\
    \ = chmax;\n                    *min_cnt = *len - *max_cnt;\n                \
    \    *min_second = *max;\n                    *min = chmax;\n                \
    \    *sum = chmax * *min_cnt as i64 + *max * *max_cnt as i64;\n              \
    \  } else if *min < chmax && chmax < *min_second {\n                    *sum -=\
    \ (*min - chmax) * *min_cnt as i64;\n                    *min = chmax;\n     \
    \           } else {\n                    unreachable!(\"tag condition for chmax\
    \ is not satisfied!!!\");\n                }\n            }\n        }\n    }\n\
    }\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \    #[test]\n    fn test_raq_rsq() {\n        use raq_rsq::RAQRSQ;\n        let\
    \ mut rng = thread_rng();\n        const SIZE: usize = 100_000;\n        const\
    \ MAX: i64 = 10000;\n        const MIN: i64 = -10000;\n        let init_vec =\
    \ (0..SIZE)\n            .map(|_| rng.gen_range(-MIN..=MAX))\n            .collect::<Vec<_>>();\n\
    \        let mut ras = RAQRSQ::new(SIZE, 0_i64);\n        for (i, &v) in init_vec.iter().enumerate()\
    \ {\n            ras.add(i..=i, v);\n        }\n        let mut seg = RangeChminMaxAddSum::from(init_vec);\n\
    \        for _ in 0..1000 {\n            let l = rng.gen_range(0..SIZE);\n   \
    \         let r = rng.gen_range(l..=SIZE);\n            let v = rng.gen_range(MIN..=MAX);\n\
    \            ras.add(l..r, v);\n            seg.range_add(l..r, v);\n        \
    \    let l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..=SIZE);\n\
    \            assert_eq!(ras.sum(l..r), seg.prod_sum(l..r));\n        }\n     \
    \   for i in 0..SIZE {\n            assert_eq!(ras.sum(i..=i), seg.prod_sum(i..=i));\n\
    \        }\n    }\n    #[test]\n    fn test_chmin_max_add() {\n        let mut\
    \ rng = thread_rng();\n        const SIZE: usize = 10000;\n        const MIN:\
    \ i64 = -10000;\n        const MAX: i64 = 10000;\n        let mut data = (0..SIZE)\n\
    \            .map(|_| rng.gen_range(MIN..=MAX))\n            .collect::<Vec<_>>();\n\
    \        let mut seg = RangeChminMaxAddSum::from(data.clone());\n        for _\
    \ in 0..1000 {\n            let l = rng.gen_range(0..SIZE);\n            let r\
    \ = rng.gen_range(l..=SIZE);\n            let v = rng.gen_range(MIN..=MAX);\n\
    \            seg.range_chmin(l..r, v);\n            for i in l..r {\n        \
    \        data[i] = data[i].min(v);\n            }\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..=SIZE);\n            let v = rng.gen_range(MIN..=MAX);\n\
    \            seg.range_chmax(l..r, v);\n            for i in l..r {\n        \
    \        data[i] = data[i].max(v);\n            }\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..=SIZE);\n            let v = rng.gen_range(MIN..=MAX);\n\
    \            seg.range_add(l..r, v);\n            for i in l..r {\n          \
    \      data[i] += v;\n            }\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..=SIZE);\n            let v = rng.gen_range(MIN..=MAX);\n\
    \            seg.range_update(l..r, v);\n            for i in l..r {\n       \
    \         data[i] = v;\n            }\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..=SIZE);\n            assert_eq!(\n     \
    \           *data[l..r].iter().min().unwrap_or(&i64::MAX),\n                seg.prod_min(l..r)\n\
    \            );\n            assert_eq!(\n                *data[l..r].iter().max().unwrap_or(&i64::MIN),\n\
    \                seg.prod_max(l..r)\n            );\n            assert_eq!(data[l..r].iter().sum::<i64>(),\
    \ seg.prod_sum(l..r));\n        }\n        for i in 0..SIZE {\n            assert_eq!(data[i],\
    \ seg.prod_sum(i..=i));\n        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/range_chmin_max_add_sum/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-31 16:00:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
documentation_of: crates/data_structure/range_chmin_max_add_sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/range_chmin_max_add_sum/src/lib.rs
- /library/crates/data_structure/range_chmin_max_add_sum/src/lib.rs.html
title: crates/data_structure/range_chmin_max_add_sum/src/lib.rs
---
