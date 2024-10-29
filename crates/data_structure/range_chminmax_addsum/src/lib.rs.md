---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/lazy_segtree_beats/src/lib.rs
    title: crates/data_structure/lazy_segtree_beats/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Range chmin/chMax, add, update query  \n//! Range min/max, sum query\
    \  \n//! \u306B\u5BFE\u5FDC\u3059\u308B(i64\u578B)\n\nuse lazy_segtree_beats::{BeatsNode,\
    \ LazySegtreeBeats};\nuse std::cmp::Ordering;\nuse InnerMonoid::*;\nuse RangeActions::*;\n\
    \n#[derive(Debug, Clone, Copy)]\npub enum InnerMonoid {\n    ZeroValue,\n    OneValue\
    \ {\n        val: i64,\n        len: usize,\n    },\n    TwoValues {\n       \
    \ min: i64,\n        min_cnt: usize,\n        max: i64,\n        max_cnt: usize,\n\
    \    },\n    ThreeOrMoreValues {\n        min: i64,\n        min_cnt: usize,\n\
    \        min_second: i64,\n        max: i64,\n        max_cnt: usize,\n      \
    \  max_second: i64,\n        len: usize,\n        sum: i64,\n    },\n}\n\nimpl\
    \ InnerMonoid {\n    pub fn get_sum(&self) -> i64 {\n        match self {\n  \
    \          ZeroValue => 0,\n            OneValue { val, len } => *val * *len as\
    \ i64,\n            TwoValues {\n                min,\n                min_cnt,\n\
    \                max,\n                max_cnt,\n            } => min * *min_cnt\
    \ as i64 + max * *max_cnt as i64,\n            ThreeOrMoreValues { sum, .. } =>\
    \ *sum,\n        }\n    }\n    pub fn get_max(&self) -> i64 {\n        match self\
    \ {\n            ZeroValue => i64::MIN,\n            OneValue { val: max, .. }\
    \ | TwoValues { max, .. } | ThreeOrMoreValues { max, .. } => {\n             \
    \   *max\n            }\n        }\n    }\n    fn get_max_cnt(&self) -> usize\
    \ {\n        match self {\n            ZeroValue => 0,\n            OneValue {\
    \ len: max_cnt, .. }\n            | TwoValues { max_cnt, .. }\n            | ThreeOrMoreValues\
    \ { max_cnt, .. } => *max_cnt,\n        }\n    }\n    pub fn get_min(&self) ->\
    \ i64 {\n        match self {\n            ZeroValue => i64::MAX,\n          \
    \  OneValue { val: min, .. } | TwoValues { min, .. } | ThreeOrMoreValues { min,\
    \ .. } => {\n                *min\n            }\n        }\n    }\n    fn get_min_cnt(&self)\
    \ -> usize {\n        match self {\n            ZeroValue => 0,\n            OneValue\
    \ { len: min_cnt, .. }\n            | TwoValues { min_cnt, .. }\n            |\
    \ ThreeOrMoreValues { min_cnt, .. } => *min_cnt,\n        }\n    }\n    fn len(&self)\
    \ -> usize {\n        match self {\n            ZeroValue => 0,\n            OneValue\
    \ { len, .. } | ThreeOrMoreValues { len, .. } => *len,\n            TwoValues\
    \ {\n                min_cnt, max_cnt, ..\n            } => min_cnt + max_cnt,\n\
    \        }\n    }\n    fn get_min_second(&self) -> Option<i64> {\n        match\
    \ self {\n            ZeroValue | OneValue { .. } => None,\n            TwoValues\
    \ {\n                max: min_second, ..\n            }\n            | ThreeOrMoreValues\
    \ { min_second, .. } => Some(*min_second),\n        }\n    }\n    fn get_max_second(&self)\
    \ -> Option<i64> {\n        match self {\n            ZeroValue | OneValue { ..\
    \ } => None,\n            TwoValues {\n                min: max_second, ..\n \
    \           }\n            | ThreeOrMoreValues { max_second, .. } => Some(*max_second),\n\
    \        }\n    }\n}\n\n#[derive(Debug, Clone, Copy)]\n/// (\u30E2\u30CE\u30A4\
    \u30C9\u3001\u9045\u5EF6\u3057\u305FAdd)\npub struct InnerNode(InnerMonoid, i64);\n\
    \n#[derive(Debug, Clone, Copy)]\npub enum RangeActions {\n    /// Chmin\n    UpperBound(i64),\n\
    \    /// ChMax\n    LowerBound(i64),\n    AddAll(i64),\n    Update(i64),\n}\n\n\
    impl BeatsNode for InnerNode {\n    type Action = RangeActions;\n    fn id_node()\
    \ -> Self {\n        Self(ZeroValue, 0)\n    }\n    fn binary_operation(lhs: &Self,\
    \ rhs: &Self) -> Self {\n        let monoid = match (lhs.0, rhs.0) {\n       \
    \     (ZeroValue, node) | (node, ZeroValue) => node,\n            (\n        \
    \        OneValue {\n                    val: l_val,\n                    len:\
    \ l_len,\n                },\n                OneValue {\n                   \
    \ val: r_val,\n                    len: r_len,\n                },\n         \
    \   ) => match l_val.cmp(&r_val) {\n                Ordering::Equal => OneValue\
    \ {\n                    val: l_val,\n                    len: l_len + r_len,\n\
    \                },\n                Ordering::Less => TwoValues {\n         \
    \           min: l_val,\n                    min_cnt: l_len,\n               \
    \     max: r_val,\n                    max_cnt: r_len,\n                },\n \
    \               Ordering::Greater => TwoValues {\n                    min: r_val,\n\
    \                    min_cnt: r_len,\n                    max: l_val,\n      \
    \              max_cnt: l_len,\n                },\n            },\n         \
    \   (\n                OneValue { val, len },\n                TwoValues {\n \
    \                   min,\n                    min_cnt,\n                    max,\n\
    \                    max_cnt,\n                },\n            )\n           \
    \ | (\n                TwoValues {\n                    min,\n               \
    \     min_cnt,\n                    max,\n                    max_cnt,\n     \
    \           },\n                OneValue { val, len },\n            ) => {\n \
    \               if val == min {\n                    TwoValues {\n           \
    \             min,\n                        min_cnt: min_cnt + len,\n        \
    \                max,\n                        max_cnt,\n                    }\n\
    \                } else if val == max {\n                    TwoValues {\n   \
    \                     min,\n                        min_cnt,\n               \
    \         max,\n                        max_cnt: max_cnt + len,\n            \
    \        }\n                } else {\n                    let mid = {\n      \
    \                  let mut ary = [min, val, max];\n                        ary.sort_unstable();\n\
    \                        ary[1]\n                    };\n                    ThreeOrMoreValues\
    \ {\n                        min: val.min(min),\n                        min_cnt:\
    \ if min < val { min_cnt } else { len },\n                        min_second:\
    \ mid,\n                        max: val.max(max),\n                        max_cnt:\
    \ if max > val { max_cnt } else { len },\n                        max_second:\
    \ mid,\n                        len: min_cnt + max_cnt + len,\n              \
    \          sum: val * len as i64 + min * min_cnt as i64 + max * max_cnt as i64,\n\
    \                    }\n                }\n            }\n            (\n    \
    \            TwoValues {\n                    min: l_min,\n                  \
    \  min_cnt: l_min_cnt,\n                    max: l_max,\n                    max_cnt:\
    \ l_max_cnt,\n                },\n                TwoValues {\n              \
    \      min: r_min,\n                    min_cnt: r_min_cnt,\n                \
    \    max: r_max,\n                    max_cnt: r_max_cnt,\n                },\n\
    \            ) => {\n                if l_min == r_min && l_max == r_max {\n \
    \                   TwoValues {\n                        min: l_min,\n       \
    \                 min_cnt: l_min_cnt + r_min_cnt,\n                        max:\
    \ l_max,\n                        max_cnt: l_max_cnt + r_max_cnt,\n          \
    \          }\n                } else {\n                    let min_cnt = match\
    \ l_min.cmp(&r_min) {\n                        Ordering::Equal => l_min_cnt +\
    \ r_min_cnt,\n                        Ordering::Less => l_min_cnt,\n         \
    \               Ordering::Greater => r_min_cnt,\n                    };\n    \
    \                let max_cnt = match l_max.cmp(&r_max) {\n                   \
    \     Ordering::Equal => l_max_cnt + r_max_cnt,\n                        Ordering::Greater\
    \ => l_max_cnt,\n                        Ordering::Less => r_max_cnt,\n      \
    \              };\n                    let mut vals = vec![l_min, l_max, r_min,\
    \ r_max];\n                    vals.sort_unstable();\n                    vals.dedup();\n\
    \                    ThreeOrMoreValues {\n                        min: l_min.min(r_min),\n\
    \                        min_cnt,\n                        min_second: vals[1],\n\
    \                        max: l_max.max(r_max),\n                        max_cnt,\n\
    \                        max_second: vals[vals.len() - 2],\n                 \
    \       len: lhs.0.len() + rhs.0.len(),\n                        sum: lhs.0.get_sum()\
    \ + rhs.0.get_sum(),\n                    }\n                }\n            }\n\
    \            // \u3042\u3068\u306F\u3069\u3046\u3084\u3063\u3066\u30823\u7A2E\u5FC5\
    \u8981\n            _ => {\n                let mut vals = vec![\n           \
    \         Some(lhs.0.get_min()),\n                    Some(rhs.0.get_min()),\n\
    \                    lhs.0.get_min_second(),\n                    rhs.0.get_min_second(),\n\
    \                ]\n                .into_iter()\n                .flatten()\n\
    \                .collect::<Vec<_>>();\n                vals.sort_unstable();\n\
    \                vals.dedup();\n                let (min, min_second) = (vals[0],\
    \ vals[1]);\n                let mut vals = vec![\n                    Some(lhs.0.get_max()),\n\
    \                    Some(rhs.0.get_max()),\n                    lhs.0.get_max_second(),\n\
    \                    rhs.0.get_max_second(),\n                ]\n            \
    \    .into_iter()\n                .flatten()\n                .collect::<Vec<_>>();\n\
    \                vals.sort_unstable();\n                vals.reverse();\n    \
    \            let (max, max_second) = (vals[0], vals[1]);\n                ThreeOrMoreValues\
    \ {\n                    min,\n                    min_cnt: match lhs.0.get_min().cmp(&rhs.0.get_min())\
    \ {\n                        Ordering::Equal => lhs.0.get_min_cnt() + rhs.0.get_min_cnt(),\n\
    \                        Ordering::Less => lhs.0.get_min_cnt(),\n            \
    \            Ordering::Greater => rhs.0.get_min_cnt(),\n                    },\n\
    \                    min_second,\n                    max,\n                 \
    \   max_cnt: match lhs.0.get_max().cmp(&rhs.0.get_max()) {\n                 \
    \       Ordering::Equal => lhs.0.get_max_cnt() + rhs.0.get_max_cnt(),\n      \
    \                  Ordering::Greater => lhs.0.get_max_cnt(),\n               \
    \         Ordering::Less => rhs.0.get_max_cnt(),\n                    },\n   \
    \                 max_second,\n                    len: lhs.0.len() + rhs.0.len(),\n\
    \                    sum: lhs.0.get_sum() + rhs.0.get_sum(),\n               \
    \ }\n            }\n        };\n        Self(monoid, 0)\n    }\n    fn apply(&mut\
    \ self, action: &Self::Action) -> bool {\n        if matches!(self.0, ZeroValue)\
    \ {\n            return true;\n        }\n        match *action {\n          \
    \  Update(val) => {\n                *self = Self(\n                    match\
    \ self.0 {\n                        ZeroValue => ZeroValue,\n                \
    \        OneValue { len, .. } | ThreeOrMoreValues { len, .. } => {\n         \
    \                   OneValue { val, len }\n                        }\n       \
    \                 TwoValues {\n                            min_cnt, max_cnt, ..\n\
    \                        } => OneValue {\n                            val,\n \
    \                           len: min_cnt + max_cnt,\n                        },\n\
    \                    },\n                    0,\n                )\n         \
    \   }\n            AddAll(add) => {\n                *self = Self(\n         \
    \           match self.0 {\n                        ZeroValue => ZeroValue,\n\
    \                        OneValue { val, len } => OneValue {\n               \
    \             val: val + add,\n                            len,\n            \
    \            },\n                        TwoValues {\n                       \
    \     min,\n                            min_cnt,\n                           \
    \ max,\n                            max_cnt,\n                        } => TwoValues\
    \ {\n                            min: min + add,\n                           \
    \ min_cnt,\n                            max: max + add,\n                    \
    \        max_cnt,\n                        },\n                        ThreeOrMoreValues\
    \ {\n                            min,\n                            min_cnt,\n\
    \                            min_second,\n                            max,\n \
    \                           max_cnt,\n                            max_second,\n\
    \                            len,\n                            sum,\n        \
    \                } => ThreeOrMoreValues {\n                            min: min\
    \ + add,\n                            min_cnt,\n                            min_second:\
    \ min_second + add,\n                            max: max + add,\n           \
    \                 max_cnt,\n                            max_second: max_second\
    \ + add,\n                            len,\n                            sum: sum\
    \ + add * len as i64,\n                        },\n                    },\n  \
    \                  self.1 + add,\n                )\n            }\n         \
    \   LowerBound(lb) => {\n                if self.0.get_min() < lb {\n        \
    \            let new_monoid = match self.0 {\n                        ZeroValue\
    \ => ZeroValue,\n                        OneValue { val, len } => OneValue {\n\
    \                            val: val.max(lb),\n                            len,\n\
    \                        },\n                        TwoValues {\n           \
    \                 min_cnt,\n                            max,\n               \
    \             max_cnt,\n                            ..\n                     \
    \   } => {\n                            if max <= lb {\n                     \
    \           OneValue {\n                                    val: lb,\n       \
    \                             len: min_cnt + max_cnt,\n                      \
    \          }\n                            } else {\n                         \
    \       TwoValues {\n                                    min: lb,\n          \
    \                          min_cnt,\n                                    max,\n\
    \                                    max_cnt,\n                              \
    \  }\n                            }\n                        }\n             \
    \           ThreeOrMoreValues {\n                            max,\n          \
    \                  max_cnt,\n                            max_second,\n       \
    \                     len,\n                            ..\n                 \
    \       } => {\n                            if max <= lb {\n                 \
    \               OneValue { val: lb, len }\n                            } else\
    \ if max_second <= lb {\n                                TwoValues {\n       \
    \                             min: lb,\n                                    min_cnt:\
    \ len - max_cnt,\n                                    max,\n                 \
    \                   max_cnt,\n                                }\n            \
    \                } else {\n                                return false;\n   \
    \                         }\n                        }\n                    };\n\
    \                    self.0 = new_monoid;\n                }\n            }\n\
    \            UpperBound(ub) => {\n                if self.0.get_max() > ub {\n\
    \                    let new_monoid = match self.0 {\n                       \
    \ ZeroValue => ZeroValue,\n                        OneValue { val, len } => OneValue\
    \ {\n                            val: val.min(ub),\n                         \
    \   len,\n                        },\n                        TwoValues {\n  \
    \                          min,\n                            min_cnt,\n      \
    \                      max_cnt,\n                            ..\n            \
    \            } => {\n                            if ub <= min {\n            \
    \                    OneValue {\n                                    val: ub,\n\
    \                                    len: min_cnt + max_cnt,\n               \
    \                 }\n                            } else {\n                  \
    \              TwoValues {\n                                    min,\n       \
    \                             min_cnt,\n                                    max:\
    \ ub,\n                                    max_cnt,\n                        \
    \        }\n                            }\n                        }\n       \
    \                 ThreeOrMoreValues {\n                            min,\n    \
    \                        min_cnt,\n                            min_second,\n \
    \                           len,\n                            ..\n           \
    \             } => {\n                            if ub <= min {\n           \
    \                     OneValue { val: ub, len }\n                            }\
    \ else if ub <= min_second {\n                                TwoValues {\n  \
    \                                  min,\n                                    min_cnt,\n\
    \                                    max: ub,\n                              \
    \      max_cnt: len - min_cnt,\n                                }\n          \
    \                  } else {\n                                return false;\n \
    \                           }\n                        }\n                   \
    \ };\n                    self.0 = new_monoid;\n                }\n          \
    \  }\n        }\n        true\n    }\n    fn push(&mut self, child_node_left:\
    \ &mut Self, child_node_right: &mut Self) {\n        if self.1 != 0 {\n      \
    \      child_node_left.apply(&AddAll(self.1));\n            child_node_right.apply(&AddAll(self.1));\n\
    \            self.1 = 0;\n        }\n        match &self.0 {\n            ZeroValue\
    \ => (),\n            OneValue { val, .. } => {\n                child_node_left.apply(&Update(*val));\n\
    \                child_node_right.apply(&Update(*val));\n            }\n     \
    \       TwoValues { min, max, .. } | ThreeOrMoreValues { min, max, .. } => {\n\
    \                child_node_left.apply(&LowerBound(*min));\n                child_node_left.apply(&UpperBound(*max));\n\
    \                child_node_right.apply(&LowerBound(*min));\n                child_node_right.apply(&UpperBound(*max));\n\
    \            }\n        }\n    }\n}\n\nuse std::ops::RangeBounds;\npub type RangeChminMaxAddSum\
    \ = LazySegtreeBeats<InnerNode>;\npub trait QueryWrapper {\n    fn from_vec(v:\
    \ Vec<i64>) -> Self;\n    fn range_chmin<R: RangeBounds<usize>>(&mut self, range:\
    \ R, chmin: i64);\n    fn range_chmax<R: RangeBounds<usize>>(&mut self, range:\
    \ R, chmax: i64);\n    fn range_update<R: RangeBounds<usize>>(&mut self, range:\
    \ R, update: i64);\n    fn range_add<R: RangeBounds<usize>>(&mut self, range:\
    \ R, add: i64);\n    fn prod<R: RangeBounds<usize>>(&mut self, range: R) -> InnerMonoid;\n\
    }\n\nimpl QueryWrapper for RangeChminMaxAddSum {\n    fn from_vec(v: Vec<i64>)\
    \ -> Self {\n        Self::from(\n            v.into_iter()\n                .map(|val|\
    \ InnerNode(OneValue { val, len: 1 }, 0))\n                .collect::<Vec<_>>(),\n\
    \        )\n    }\n    fn range_add<R: RangeBounds<usize>>(&mut self, range: R,\
    \ add: i64) {\n        self.apply_range(range, &AddAll(add));\n    }\n    fn range_chmax<R:\
    \ RangeBounds<usize>>(&mut self, range: R, chmax: i64) {\n        self.apply_range(range,\
    \ &LowerBound(chmax));\n    }\n    fn range_chmin<R: RangeBounds<usize>>(&mut\
    \ self, range: R, chmin: i64) {\n        self.apply_range(range, &UpperBound(chmin));\n\
    \    }\n    fn range_update<R: RangeBounds<usize>>(&mut self, range: R, update:\
    \ i64) {\n        self.apply_range(range, &Update(update));\n    }\n    fn prod<R:\
    \ RangeBounds<usize>>(&mut self, range: R) -> InnerMonoid {\n        self.prod(range).0\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/lazy_segtree_beats/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/range_chminmax_addsum/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-29 23:43:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/range_chminmax_addsum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/range_chminmax_addsum/src/lib.rs
- /library/crates/data_structure/range_chminmax_addsum/src/lib.rs.html
title: crates/data_structure/range_chminmax_addsum/src/lib.rs
---
