---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/yosupo/rectangle_add_point_get/src/main.rs
    title: verify/yosupo/rectangle_add_point_get/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - "https://trap.jp/post/1489/)\u3092\u53C2\u8003\u306B\u3001kd-tree\u3067\u5B9F\
      \u88C5\u3057\u3066\u3044\u307E\u3059"
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u77E9\u5F62\u7BC4\u56F2\u3078\u306E\u53EF\u63DB\u4F5C\u7528\u306E\u9069\
    \u7528\u30011\u70B9\u53D6\u5F97\u306E\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3067\
    \u304D\u307E\u3059  \n//! \u70B9\u306E\u8FFD\u52A0\u3068\u524A\u9664\u306F\u3067\
    \u304D\u307E\u305B\u3093(\u30AA\u30D5\u30E9\u30A4\u30F3\u524D\u63D0)  \n//! [kd-tree](https://trap.jp/post/1489/)\u3092\
    \u53C2\u8003\u306B\u3001kd-tree\u3067\u5B9F\u88C5\u3057\u3066\u3044\u307E\u3059\
    \  \n\nuse algebra::{Action, Commutative};\nuse internal_type_traits::Integral;\n\
    use std::ops::{Bound::*, RangeBounds};\n\n/// A\u306F\u53EF\u63DB\u4F5C\u7528\u3001\
    T\u306F\u5EA7\u6A19\u306E\u578B\n#[derive(Debug)]\npub struct RectActPointGet<A:\
    \ Action + Commutative, T: Integral> {\n    x_min: T,\n    x_max: T,\n    y_min:\
    \ T,\n    y_max: T,\n    lazy: A,\n    left: Option<Box<RectActPointGet<A, T>>>,\n\
    \    right: Option<Box<RectActPointGet<A, T>>>,\n}\n\nimpl<A: Action + Commutative,\
    \ T: Integral> RectActPointGet<A, T> {\n    /// \u70B9\u53D6\u5F97\u306E\u30AF\
    \u30A8\u30EA\u3092\u5148\u8AAD\u307F\u3057\u3066\u3001\u305D\u306E\u70B9\u3092\
    \u7528\u3044\u3066\u69CB\u7BC9\n    pub fn new(mut points: Vec<(T, T)>) -> Self\
    \ {\n        // \u91CD\u8907\u9664\u53BB\n        points.sort();\n        points.dedup();\n\
    \        Self::new_sub(&mut points)\n    }\n\n    fn new_sub(points: &mut [(T,\
    \ T)]) -> Self {\n        let mut x_min = T::max_value();\n        let mut x_max\
    \ = T::min_value();\n        let mut y_min = T::max_value();\n        let mut\
    \ y_max = T::min_value();\n        for (x, y) in points.iter() {\n           \
    \ x_min = x_min.min(*x);\n            x_max = x_max.max(*x);\n            y_min\
    \ = y_min.min(*y);\n            y_max = y_max.max(*y);\n        }\n        let\
    \ size = points.len();\n        let mut ret = RectActPointGet {\n            x_min,\n\
    \            x_max,\n            y_min,\n            y_max,\n            lazy:\
    \ A::id_map(),\n            left: None,\n            right: None,\n        };\n\
    \        if size <= 1 {\n            return ret;\n        }\n        let mid =\
    \ size / 2;\n        let x_idx = {\n            let x_mid = points.select_nth_unstable_by_key(mid,\
    \ |(x, _)| *x).1 .0;\n            let excluded_cnt = points.iter().filter(|(x,\
    \ _)| *x < x_mid).count();\n            let included_cnt = points.iter().filter(|(x,\
    \ _)| *x <= x_mid).count();\n            if (size - excluded_cnt).abs_diff(excluded_cnt)\n\
    \                < (size - included_cnt).abs_diff(included_cnt)\n            {\n\
    \                excluded_cnt\n            } else {\n                included_cnt\n\
    \            }\n        };\n        let y_idx = {\n            let y_mid = points.select_nth_unstable_by_key(mid,\
    \ |(_, y)| *y).1 .1;\n            let excluded_cnt = points.iter().filter(|(_,\
    \ y)| *y < y_mid).count();\n            let included_cnt = points.iter().filter(|(_,\
    \ y)| *y <= y_mid).count();\n            if (size - excluded_cnt).abs_diff(excluded_cnt)\n\
    \                < (size - included_cnt).abs_diff(included_cnt)\n            {\n\
    \                excluded_cnt\n            } else {\n                included_cnt\n\
    \            }\n        };\n        if (size - x_idx).abs_diff(x_idx) < (size\
    \ - y_idx).abs_diff(y_idx) {\n            points.select_nth_unstable_by_key(x_idx,\
    \ |(x, _)| *x);\n            ret.left = Some(Box::new(Self::new_sub(&mut points[..x_idx])));\n\
    \            ret.right = Some(Box::new(Self::new_sub(&mut points[x_idx..])));\n\
    \        } else {\n            points.select_nth_unstable_by_key(y_idx, |(_, y)|\
    \ *y);\n            ret.left = Some(Box::new(Self::new_sub(&mut points[..y_idx])));\n\
    \            ret.right = Some(Box::new(Self::new_sub(&mut points[y_idx..])));\n\
    \        }\n        ret\n    }\n\n    pub fn add_range<R1: RangeBounds<T>, R2:\
    \ RangeBounds<T>>(\n        &mut self,\n        x_range: &R1,\n        y_range:\
    \ &R2,\n        action: &A,\n    ) {\n        // \u4ECA\u56DE\u306F\u5185\u90E8\
    \u3067\u306F\u9589\u533A\u9593\u3067\u6271\u3046\n        let x_min = match x_range.start_bound()\
    \ {\n            Included(&l) => l,\n            Excluded(&l) => l + T::one(),\n\
    \            Unbounded => self.x_min,\n        };\n        let x_max = match x_range.end_bound()\
    \ {\n            Included(&r) => r,\n            Excluded(&r) => r - T::one(),\n\
    \            Unbounded => self.x_max,\n        };\n        let y_min = match y_range.start_bound()\
    \ {\n            Included(&l) => l,\n            Excluded(&l) => l + T::one(),\n\
    \            Unbounded => self.y_min,\n        };\n        let y_max = match y_range.end_bound()\
    \ {\n            Included(&r) => r,\n            Excluded(&r) => r - T::one(),\n\
    \            Unbounded => self.y_max,\n        };\n\n        if x_max < self.x_min\
    \ || self.x_max < x_min || y_max < self.y_min || self.y_max < y_min {\n      \
    \      return;\n        }\n        // \u9818\u57DF\u5185\u306E\u3059\u3079\u3066\
    \u306E\u70B9\u306B\u4F5C\u7528\u3092\u9069\u7528\u3067\u304D\u308B\u5834\u5408\
    \n        if x_min <= self.x_min && self.x_max <= x_max && y_min <= self.y_min\
    \ && self.y_max <= y_max\n        {\n            self.lazy.composition(action);\n\
    \            return;\n        }\n        if let Some(left) = &mut self.left {\n\
    \            left.add_range(x_range, y_range, action);\n        }\n        if\
    \ let Some(right) = &mut self.right {\n            right.add_range(x_range, y_range,\
    \ action);\n        }\n    }\n\n    pub fn get_composition(&self, x: T, y: T)\
    \ -> A {\n        if x < self.x_min || self.x_max < x || y < self.y_min || self.y_max\
    \ < y {\n            return A::id_map();\n        }\n        // 1\u70B9\u306E\u307F\
    \u306E\u8449\u30CE\u30FC\u30C9\n        if self.x_min == self.x_max\n        \
    \    && self.y_min == self.y_max\n            && self.x_min == x\n           \
    \ && self.y_min == y\n        {\n            return self.lazy.clone();\n     \
    \   }\n        let mut res = self.lazy.clone();\n        if let Some(left) = &self.left\
    \ {\n            res.composition(&left.get_composition(x, y));\n        }\n  \
    \      if let Some(right) = &self.right {\n            res.composition(&right.get_composition(x,\
    \ y));\n        }\n        res\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/rect_add_point_get/src/lib.rs
  requiredBy:
  - verify/yosupo/rectangle_add_point_get/src/main.rs
  timestamp: '2024-07-06 23:41:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/rect_add_point_get/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/rect_add_point_get/src/lib.rs
- /library/crates/data_structure/rect_add_point_get/src/lib.rs.html
title: crates/data_structure/rect_add_point_get/src/lib.rs
---
