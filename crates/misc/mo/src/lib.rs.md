---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/abc384g/src/main.rs
    title: verify/AtCoder/abc384g/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/static_range_inversions_query/src/main.rs
    title: verify/yosupo/static_range_inversions_query/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://ei1333.hateblo.jp/entry/2017/09/11/211011
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [Mo's algorithm](https://ei1333.hateblo.jp/entry/2017/09/11/211011) \
    \ \n//! \u4E00\u822C\u306B\u4E8C\u3064\u306Eindex `(x, y)` \u3092\u3068\u308B\u30AF\
    \u30A8\u30EA\u9054\u306B\u5BFE\u3057\u3066\u3001`x`\u3084`y`\u3092+-1\u3059\u308B\
    \u5909\u66F4\u3092`O(\u03B1)`\u3067\u3067\u304D\u308B\u5834\u5408  \n//! `0 <=\
    \ x <= nx, 0 <= y <= ny` \u3068\u3059\u308B\u3068\u5168\u4F53\u3067`O(\u03B1\\\
    sqrt{nxnyQ})`\u3067\u51E6\u7406\u3067\u304D\u308B (Q = \u30AF\u30A8\u30EA\u306E\
    \u6570)  \n//! \u30AF\u30A8\u30EA\u5148\u8AAD\u307F\u304C\u5FC5\u8981  \n//! index`(x,\
    \ y)`\u304C\u533A\u9593\u30AF\u30A8\u30EA`[x, y)`\u306B\u5BFE\u5FDC\u3059\u308B\
    \u5834\u5408\u304C\u3088\u304F\u3042\u308B\u4F7F\u3044\u65B9\u3060\u304C\u3001\
    \n//! \u5225\u306B`x <= y`\u3067\u3042\u308B\u5FC5\u8981\u306F\u306A\u304F\u3001\
    \u4E00\u822C\u306B\u4E8C\u3064\u306Eindex\u3092\u3068\u308B\u30AF\u30A8\u30EA\u5168\
    \u822C\u306B\u4F7F\u3048\u308B\n\n/// \u73FE\u5728\u306E\u30AF\u30A8\u30EA\u306E\
    \u5024\u3001index\u306E+-1\u306E\u5909\u66F4\u3001\u7B54\u3048\u306E\u914D\u5217\
    \u3092\u7BA1\u7406\u3059\u308Btrait  \n/// \u30AF\u30A8\u30EA(0, 0)\u3067\u521D\
    \u671F\u5316\u3059\u308B\npub trait MoFuncs {\n    /// (x, y) -> (x-1, y) \u306E\
    \u5909\u66F4\n    fn x_minus(&mut self, x: usize);\n    /// (x, y) -> (x+1, y)\
    \ \u306E\u5909\u66F4\n    fn x_plus(&mut self, x: usize);\n    /// (x, y) -> (x,\
    \ y-1) \u306E\u5909\u66F4\n    fn y_minus(&mut self, y: usize);\n    /// (x, y)\
    \ -> (x, y+1) \u306E\u5909\u66F4\n    fn y_plus(&mut self, y: usize);\n    ///\
    \ \u73FE\u5728\u306E\u30AF\u30A8\u30EA\u306E\u5024\u3092\u3001\u7B54\u3048\u306E\
    \u914D\u5217\u306Eidx\u756A\u76EE\u306B\u8A18\u9332\u3059\u308B\n    fn memo(&mut\
    \ self, idx: usize);\n}\n\n#[derive(Debug)]\npub struct MoRunner<'a> {\n    querys:\
    \ &'a [(usize, usize)],\n    order: Vec<usize>,\n}\n\nimpl<'a> MoRunner<'a> {\n\
    \    /// `querys` \u306F\u30AA\u30D5\u30E9\u30A4\u30F3\u3067\u8AAD\u307F\u8FBC\
    \u3093\u3060\u30AF\u30A8\u30EA(x, y) \u306E\u914D\u5217  \n    /// `0 <= x <=\
    \ nx, 0 <= y <= ny` \u3068\u3059\u308B\n    pub fn new(querys: &'a [(usize, usize)],\
    \ nx: usize, ny: usize) -> Self {\n        let order = calc_mo_friendly_order(querys,\
    \ nx, ny);\n        Self { querys, order }\n    }\n\n    /// \u30AF\u30A8\u30EA\
    (0, 0)\u304B\u3089\u521D\u3081\u3066\u3001index\u306E+-1\u306E\u5909\u66F4\u3092\
    \u884C\u3044\u3059\u3079\u3066\u306E\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\
    \u308B  \n    /// \u30AF\u30A8\u30EA\u306E\u5024\u3092\u9806\u306B\u4FDD\u6301\
    \u3057\u305F\u914D\u5217\u3092\u8FD4\u3059\n    pub fn run<M: MoFuncs>(&self,\
    \ mo_funcs: &mut M) {\n        let mut x = 0;\n        let mut y = 0;\n      \
    \  for id in &self.order {\n            let (nx, ny) = self.querys[*id];\n   \
    \         while x > nx {\n                mo_funcs.x_minus(x);\n             \
    \   x -= 1;\n            }\n            while y < ny {\n                mo_funcs.y_plus(y);\n\
    \                y += 1;\n            }\n            while x < nx {\n        \
    \        mo_funcs.x_plus(x);\n                x += 1;\n            }\n       \
    \     while y > ny {\n                mo_funcs.y_minus(y);\n                y\
    \ -= 1;\n            }\n            mo_funcs.memo(*id);\n        }\n    }\n}\n\
    \n/// \u30AF\u30A8\u30EA\u306E\u5DE6\u53F3\u7AEF+-1\u5909\u5316\u304C\u5C11\u306A\
    \u304F\u306A\u308B\u3088\u3046\u306B\u3001\u30AF\u30A8\u30EA\u756A\u53F7[0,1,...Q)\u3092\
    \u30BD\u30FC\u30C8\u3057\u305F\u914D\u5217\u3092\u8FD4\u3059  \n/// \u5404\u30AF\
    \u30A8\u30EA`(x, y)`\u306F\u3001`0 <= x <= nx, 0 <= y <= ny` \u3068\u3059\u308B\
    \npub fn calc_mo_friendly_order(query_ranges: &[(usize, usize)], nx: usize, ny:\
    \ usize) -> Vec<usize> {\n    if query_ranges.is_empty() {\n        return vec![];\n\
    \    }\n    let query_cnt = query_ranges.len();\n    let mut order = (0..query_cnt).collect::<Vec<_>>();\n\
    \    let block_size = ((nx as f64 * ny as f64 / query_cnt as f64).sqrt().ceil()\
    \ as usize).max(1);\n    // x/block_size\u3067\u30BD\u30FC\u30C8 \u305D\u306E\u4E2D\
    \u3067\u306Fy\u3067\u30BD\u30FC\u30C8 y\u306B\u3064\u3044\u3066\u306F\u6607\u9806\
    \u3068\u964D\u9806\u3092\u4EA4\u4E92\u306B\u3059\u308B\n    order.sort_unstable_by(|&a,\
    \ &b| {\n        let (x1, y1) = query_ranges[a];\n        let (x2, y2) = query_ranges[b];\n\
    \        let block1 = x1 / block_size;\n        let block2 = x2 / block_size;\n\
    \        block1.cmp(&block2).then(if (block1 & 1) == 0 {\n            y1.cmp(&y2)\n\
    \        } else {\n            y2.cmp(&y1)\n        })\n    });\n    order\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/mo/src/lib.rs
  requiredBy:
  - verify/AtCoder/abc384g/src/main.rs
  timestamp: '2024-12-15 22:00:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/static_range_inversions_query/src/main.rs
documentation_of: crates/misc/mo/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/mo/src/lib.rs
- /library/crates/misc/mo/src/lib.rs.html
title: crates/misc/mo/src/lib.rs
---
