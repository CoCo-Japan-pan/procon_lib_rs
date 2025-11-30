---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5927\u304D\u3044 \u307E\u305F\u306F \u5C0F\u3055\u3044 \u9806\u306B\
    \u4E0A\u4F4DK(\u56FA\u5B9A)\u500B\u306E\u548C\u3092\u6C42\u3081\u308B\u30C7\u30FC\
    \u30BF\u69CB\u9020\nuse internal_type_traits::Zero;\nuse std::collections::BTreeMap;\n\
    use std::ops::{Add, AddAssign, Neg, SubAssign};\n\n/// \u5927\u304D\u3044 \u307E\
    \u305F\u306F \u5C0F\u3055\u3044 \u9806\u306B\u4E0A\u4F4DK\u500B\u306E\u548C\u3092\
    \u6C42\u3081\u308B  \n/// K\u304C\u56FA\u5B9A\u3055\u308C\u3066\u3044\u308B\u3053\
    \u3068\u3092\u524D\u63D0\u3068\u3057\u3066\u3044\u308B  \n/// \u540C\u3058\u5024\
    \u3092\u8907\u6570\u56DE\u633F\u5165\u53EF\u80FD\npub struct TopKSum<T: Ord, const\
    \ BIGGER: bool> {\n    top_multi_set: BTreeMap<T, usize>,\n    left_multi_set:\
    \ BTreeMap<T, usize>,\n    k: usize,\n    size: usize,\n    sum: T,\n}\n\nimpl<T,\
    \ const BIGGER: bool> TopKSum<T, BIGGER>\nwhere\n    T: Ord + Copy + Neg<Output\
    \ = T> + Add<Output = T> + AddAssign + SubAssign + Zero,\n{\n    pub fn new(k:\
    \ usize) -> Self {\n        Self {\n            top_multi_set: BTreeMap::new(),\n\
    \            left_multi_set: BTreeMap::new(),\n            k,\n            size:\
    \ 0,\n            sum: T::zero(),\n        }\n    }\n\n    pub fn insert(&mut\
    \ self, value: T) {\n        let value = if BIGGER { value } else { -value };\n\
    \        if self.size < self.k {\n            *self.top_multi_set.entry(value).or_default()\
    \ += 1;\n            self.sum += value;\n        } else if let Some((&smallest_top,\
    \ _)) = self.top_multi_set.first_key_value() {\n            if value > smallest_top\
    \ {\n                // top \u304B\u3089 smallest_top \u3092 left \u306B\u79FB\
    \u52D5\n                let count = self.top_multi_set.get_mut(&smallest_top).unwrap();\n\
    \                *count -= 1;\n                if *count == 0 {\n            \
    \        self.top_multi_set.remove(&smallest_top);\n                }\n      \
    \          *self.left_multi_set.entry(smallest_top).or_default() += 1;\n     \
    \           self.sum -= smallest_top;\n\n                // value \u3092 top \u306B\
    \u8FFD\u52A0\n                *self.top_multi_set.entry(value).or_default() +=\
    \ 1;\n                self.sum += value;\n            } else {\n             \
    \   *self.left_multi_set.entry(value).or_default() += 1;\n            }\n    \
    \    } else {\n            *self.left_multi_set.entry(value).or_default() += 1;\n\
    \        }\n        self.size += 1;\n    }\n\n    /// value \u304C\u5B58\u5728\
    \u3057\u305F\u3089\u524A\u9664\u3057\u3066 true \u3092\u8FD4\u3059\u3002\u5B58\
    \u5728\u3057\u306A\u3051\u308C\u3070 false \u3092\u8FD4\u3059\u3002\n    pub fn\
    \ remove(&mut self, value: T) -> bool {\n        let value = if BIGGER { value\
    \ } else { -value };\n        if let Some(count) = self.top_multi_set.get_mut(&value)\
    \ {\n            *count -= 1;\n            if *count == 0 {\n                self.top_multi_set.remove(&value);\n\
    \            }\n            self.sum -= value;\n\n            // left \u304B\u3089\
    \u6700\u5927\u5024\u3092 top \u306B\u79FB\u52D5\n            if let Some((&largest_left,\
    \ _)) = self.left_multi_set.last_key_value() {\n                let count = self.left_multi_set.get_mut(&largest_left).unwrap();\n\
    \                *count -= 1;\n                if *count == 0 {\n            \
    \        self.left_multi_set.remove(&largest_left);\n                }\n     \
    \           *self.top_multi_set.entry(largest_left).or_default() += 1;\n     \
    \           self.sum += largest_left;\n            }\n        } else if let Some(count)\
    \ = self.left_multi_set.get_mut(&value) {\n            *count -= 1;\n        \
    \    if *count == 0 {\n                self.left_multi_set.remove(&value);\n \
    \           }\n        } else {\n            return false;\n        }\n      \
    \  self.size -= 1;\n        true\n    }\n\n    pub fn sum(&self) -> T {\n    \
    \    if BIGGER {\n            self.sum\n        } else {\n            -self.sum\n\
    \        }\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    use rand::Rng;\n\n    use\
    \ super::*;\n    #[test]\n    fn test_topk_sum() {\n        let mut rng = rand::thread_rng();\n\
    \        const SIZE: usize = 1000;\n        const K: usize = 100;\n        let\
    \ mut topk_smaller_sum = TopKSum::<i64, false>::new(K);\n        let mut topk_bigger_sum\
    \ = TopKSum::<i64, true>::new(K);\n        let mut vec = Vec::new();\n\n     \
    \   for _ in 0..K * 2 {\n            let value: i64 = rng.gen_range(-1000..1000);\n\
    \            topk_smaller_sum.insert(value);\n            topk_bigger_sum.insert(value);\n\
    \            vec.push(value);\n        }\n\n        vec.sort();\n\n        assert_eq!(\n\
    \            topk_smaller_sum.sum(),\n            vec.iter().take(K).copied().sum::<i64>()\n\
    \        );\n        assert_eq!(\n            topk_bigger_sum.sum(),\n       \
    \     vec.iter().rev().take(K).copied().sum::<i64>()\n        );\n\n        for\
    \ _ in 0..SIZE * 10 {\n            let add: bool = rng.gen();\n            let\
    \ value: i64 = rng.gen_range(-1000..1000);\n            if add {\n           \
    \     topk_smaller_sum.insert(value);\n                topk_bigger_sum.insert(value);\n\
    \                vec.push(value);\n            } else {\n                topk_smaller_sum.remove(value);\n\
    \                topk_bigger_sum.remove(value);\n                if let Some(pos)\
    \ = vec.iter().position(|&x| x == value) {\n                    vec.remove(pos);\n\
    \                }\n            }\n            vec.sort();\n            assert_eq!(\n\
    \                topk_smaller_sum.sum(),\n                vec.iter().take(K).copied().sum::<i64>()\n\
    \            );\n            assert_eq!(\n                topk_bigger_sum.sum(),\n\
    \                vec.iter().rev().take(K).copied().sum::<i64>()\n            );\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/misc/topk_sum/src/lib.rs
  requiredBy: []
  timestamp: '2025-11-30 17:49:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/topk_sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/topk_sum/src/lib.rs
- /library/crates/misc/topk_sum/src/lib.rs.html
title: crates/misc/topk_sum/src/lib.rs
---
