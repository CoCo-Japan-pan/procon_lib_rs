---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/line_add_get_min/src/main.rs
    title: verify/yosupo/line_add_get_min/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://hcpc-hokudai.github.io/archive/algorithm_convex_hull_trick_001.pdf)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Li Chao tree\u306B\u3088\u308BCHT\n//! i64\u578B\u3067\u53CE\u307E\u308B\
    \u524D\u63D0  \n//! ax + b \u306E\u76F4\u7DDA\u7FA4\u3092\u8FFD\u52A0\u3057\u3066\
    \u3001\u7279\u5B9A\u306Ex\u306B\u304A\u3051\u308B\u6700\u5C0F\u5024\u307E\u305F\
    \u306F\u6700\u5927\u5024\u3092logN\u3067\u6C42\u3081\u308B  \n//! \u30AF\u30A8\
    \u30EA\u3067\u805E\u304B\u308C\u308Bx\u5EA7\u6A19\u9054\u304C\u65E2\u77E5(\u30AA\
    \u30D5\u30E9\u30A4\u30F3)\u3001\u307E\u305F\u306F\u305D\u306E\u7BC4\u56F2\u304C\
    10^5\u3050\u3089\u3044\u306B\u53CE\u307E\u3063\u3066\u3044\u308B\u5834\u5408\u306B\
    \u9650\u308B  \n//! [CHT](https://hcpc-hokudai.github.io/archive/algorithm_convex_hull_trick_001.pdf)\n\
    use internal_bits::ceil_log2;\n\n/// \u6700\u5927\u5024\u30AF\u30A8\u30EA\u3068\
    \u6700\u5C0F\u5024\u30AF\u30A8\u30EA\u306E\u4E21\u65B9\u306B\u5BFE\u5FDC\u3059\
    \u308B\u305F\u3081\u306B\u4FBF\u5B9C\u7684\u306B\u5C0E\u5165\u3057\u305F\u30C8\
    \u30EC\u30A4\u30C8\npub trait Compare {\n    fn identity() -> i64;\n    /// lhs\u3092\
    rhs\u3067\u66F4\u65B0\u3059\u308B\u3079\u304D\u306A\u3089true\n    fn update(lhs:\
    \ i64, rhs: i64) -> bool;\n}\n\n#[derive(Debug)]\npub enum MaxCompare {}\nimpl\
    \ Compare for MaxCompare {\n    fn identity() -> i64 {\n        i64::MIN\n   \
    \ }\n    #[inline]\n    fn update(lhs: i64, rhs: i64) -> bool {\n        lhs <\
    \ rhs\n    }\n}\n#[derive(Debug)]\npub enum MinCompare {}\nimpl Compare for MinCompare\
    \ {\n    fn identity() -> i64 {\n        i64::MAX\n    }\n    #[inline]\n    fn\
    \ update(lhs: i64, rhs: i64) -> bool {\n        lhs > rhs\n    }\n}\n\n/// \u6700\
    \u5927\u5024\u30AF\u30A8\u30EA\u306E\u5834\u5408\u306F`T = MaxCompare`\u3001\u6700\
    \u5C0F\u5024\u30AF\u30A8\u30EA\u306E\u5834\u5408\u306F`T = MinCompare`\n#[derive(Debug)]\n\
    pub struct CHTOffline<T: Compare> {\n    sorted_points: Vec<i64>,\n    leaf_size:\
    \ usize,\n    line_per_nodes: Vec<(i64, i64)>,\n    _phantom: std::marker::PhantomData<T>,\n\
    }\n\nimpl<T: Compare> CHTOffline<T> {\n    pub fn new(mut points: Vec<i64>) ->\
    \ Self {\n        points.sort_unstable();\n        points.dedup();\n        let\
    \ log = ceil_log2(points.len() as u32) as usize;\n        let leaf_size = 1 <<\
    \ log;\n        // \u5B8C\u5168\u4E8C\u5206\u6728\u306B\u3059\u308B\u305F\u3081\
    \u306B\u3001\u8DB3\u308A\u306A\u3044\u5206\u306F\u305D\u306E\u6700\u5927\u5024\
    \u3067\u57CB\u3081\u308B\n        let max_point = *points.last().unwrap();\n \
    \       points.extend(std::iter::repeat(max_point).take(leaf_size - points.len()));\n\
    \        Self {\n            sorted_points: points,\n            leaf_size,\n\
    \            line_per_nodes: vec![(0, T::identity()); leaf_size * 2],\n      \
    \      _phantom: std::marker::PhantomData,\n        }\n    }\n\n    /// x\u306B\
    \u304A\u3051\u308B\u6700\u5C0F\u5024\u307E\u305F\u306F\u6700\u5927\u5024\u3092\
    \u6C42\u3081\u308B\n    pub fn get(&self, x: i64) -> i64 {\n        let mut id\
    \ = self\n            .sorted_points\n            .binary_search(&x)\n       \
    \     .expect(\"x is not in points!!!\");\n        id += self.leaf_size;\n   \
    \     let mut ret = T::identity();\n        while id > 0 {\n            let (a,\
    \ b) = self.line_per_nodes[id];\n            let new_num = a * x + b;\n      \
    \      if T::update(ret, new_num) {\n                ret = new_num;\n        \
    \    }\n            id >>= 1;\n        }\n        ret\n    }\n\n    /// \u76F4\
    \u7DDA`a x + b`\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_line(&mut self,\
    \ mut a: i64, mut b: i64) {\n        let mut node_id = 1;\n        let mut left\
    \ = 0;\n        let mut right = self.leaf_size;\n        // [left, right)\u3067\
    \u8003\u3048\u308B\n        while right - left > 0 {\n            let (mut cur_a,\
    \ mut cur_b) = self.line_per_nodes[node_id];\n            // \u307E\u305A\u5B8C\
    \u5168\u306B\u4E0A\u56DE\u308B\u3001\u4E0B\u56DE\u308B\u5834\u5408\n         \
    \   let left_point = cur_a * self.sorted_points[left] + cur_b;\n            let\
    \ left_new_point = a * self.sorted_points[left] + b;\n            let right_point\
    \ = cur_a * self.sorted_points[right - 1] + cur_b;\n            let right_new_point\
    \ = a * self.sorted_points[right - 1] + b;\n            let left_update = T::update(left_point,\
    \ left_new_point);\n            let right_update = T::update(right_point, right_new_point);\n\
    \            match (left_update, right_update) {\n                (true, true)\
    \ => {\n                    self.line_per_nodes[node_id] = (a, b);\n         \
    \           return;\n                }\n                (false, false) => {\n\
    \                    return;\n                }\n                _ => {}\n   \
    \         }\n            let mid = (left + right) / 2;\n            if left_update\
    \ {\n                let mid_point = cur_a * self.sorted_points[mid] + cur_b;\n\
    \                let mid_new_point = a * self.sorted_points[mid] + b;\n      \
    \          let mid_update = T::update(mid_point, mid_new_point);\n           \
    \     if !mid_update {\n                    node_id <<= 1;\n                 \
    \   right = mid;\n                } else {\n                    // \u76F4\u7DDA\
    \u3092\u4EA4\u63DB\n                    self.line_per_nodes[node_id] = (a, b);\n\
    \                    std::mem::swap(&mut a, &mut cur_a);\n                   \
    \ std::mem::swap(&mut b, &mut cur_b);\n                    node_id = (node_id\
    \ << 1) | 1;\n                    left = mid;\n                }\n           \
    \ } else {\n                let mid_point = cur_a * self.sorted_points[mid - 1]\
    \ + cur_b;\n                let mid_new_point = a * self.sorted_points[mid - 1]\
    \ + b;\n                let mid_update = T::update(mid_point, mid_new_point);\n\
    \                if !mid_update {\n                    node_id = (node_id << 1)\
    \ | 1;\n                    left = mid;\n                } else {\n          \
    \          // \u76F4\u7DDA\u3092\u4EA4\u63DB\n                    self.line_per_nodes[node_id]\
    \ = (a, b);\n                    std::mem::swap(&mut a, &mut cur_a);\n       \
    \             std::mem::swap(&mut b, &mut cur_b);\n                    node_id\
    \ <<= 1;\n                    right = mid;\n                }\n            }\n\
    \        }\n    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test() {\n        fn do_test(point_size: usize) {\n    \
    \        let mut rng = thread_rng();\n            let points: Vec<i64> = (0..point_size)\n\
    \                .map(|_| rng.gen_range(-10000..10000))\n                .collect();\n\
    \            let mut cht_max = CHTOffline::<MaxCompare>::new(points.clone());\n\
    \            let mut cht_min = CHTOffline::<MinCompare>::new(points.clone());\n\
    \            let mut lines = vec![];\n            for _ in 0..1000 {\n       \
    \         let a = rng.gen_range(-10000..10000);\n                let b = rng.gen_range(-10000..10000);\n\
    \                cht_max.add_line(a, b);\n                cht_min.add_line(a,\
    \ b);\n                lines.push((a, b));\n            }\n            for x in\
    \ points.iter() {\n                let mut max = i64::MIN;\n                let\
    \ mut min = i64::MAX;\n                for (a, b) in lines.iter() {\n        \
    \            max = max.max(a * x + b);\n                    min = min.min(a *\
    \ x + b);\n                }\n                assert_eq!(cht_max.get(*x), max);\n\
    \                assert_eq!(cht_min.get(*x), min);\n            }\n        }\n\
    \        do_test(10);\n        do_test(100);\n        do_test(1000);\n       \
    \ do_test(10000);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/cht_offline/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-15 14:45:26+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/line_add_get_min/src/main.rs
documentation_of: crates/data_structure/cht_offline/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/cht_offline/src/lib.rs
- /library/crates/data_structure/cht_offline/src/lib.rs.html
title: crates/data_structure/cht_offline/src/lib.rs
---
