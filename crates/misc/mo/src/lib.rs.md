---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/static_range_inversions_query/src/main.rs
    title: verify/yosupo/static_range_inversions_query/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u533A\u9593[L, R)\u306B\u3064\u3044\u3066\u3001\u5143\u306E\u7D50\u679C\
    \u3092\u7528\u3044\u3066\u3001L\u3084R\u3092+-1\u3057\u305F\u533A\u9593\u306E\u7D50\
    \u679C\u3092`O(\u03B1)`\u3067\u3067\u304D\u308B\u5834\u5408  \n//! \u5168\u4F53\
    \u3092`O(\u03B1N\u221AQ)`\u3067\u51E6\u7406\u3067\u304D\u308B (N = \u533A\u9593\
    \u5168\u4F53\u306E\u9577\u3055, Q = \u30AF\u30A8\u30EA\u306E\u6570)  \n//! \u30AF\
    \u30A8\u30EA\u5148\u8AAD\u307F\u304C\u5FC5\u8981  \n\n/// \u30AF\u30A8\u30EA\u306E\
    \u5DE6\u53F3\u7AEF+-1\u5909\u5316\u304C\u5C11\u306A\u304F\u306A\u308B\u3088\u3046\
    \u306B\u3001\u30AF\u30A8\u30EA\u756A\u53F7[0,1,...Q)\u3092\u30BD\u30FC\u30C8\u3057\
    \u305F\u914D\u5217\u3092\u8FD4\u3059\npub fn calc_mo_friendly_order(range_size:\
    \ usize, query_ranges: &Vec<(usize, usize)>) -> Vec<usize> {\n    if query_ranges.is_empty()\
    \ {\n        return vec![];\n    }\n    let query_cnt = query_ranges.len();\n\
    \    let mut order = (0..query_cnt).collect::<Vec<_>>();\n    let block_size =\
    \ (range_size / ((query_cnt as f64).sqrt().ceil() as usize)).max(1);\n    // left/block_size\u3067\
    \u30BD\u30FC\u30C8 \u305D\u306E\u4E2D\u3067\u306F\u53F3\u7AEF\u3067\u30BD\u30FC\
    \u30C8 \u53F3\u7AEF\u306B\u3064\u3044\u3066\u306F\u6607\u9806\u3068\u964D\u9806\
    \u3092\u4EA4\u4E92\u306B\u3059\u308B\n    order.sort_by(|&a, &b| {\n        let\
    \ (l1, r1) = query_ranges[a];\n        let (l2, r2) = query_ranges[b];\n     \
    \   let block1 = l1 / block_size;\n        let block2 = l2 / block_size;\n   \
    \     block1.cmp(&block2).then(if (block1 & 1) == 0 {\n            r1.cmp(&r2)\n\
    \        } else {\n            r2.cmp(&r1)\n        })\n    });\n    order\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/mo/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 18:44:32+09:00'
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
