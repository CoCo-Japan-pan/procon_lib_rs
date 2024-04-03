---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc345/tasks/abc345_e)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [abc345e](https://atcoder.jp/contests/abc345/tasks/abc345_e) \u3067Top2\u306E\
    \u307F\u4FDD\u6301\u3059\u308B\u3082\u306E\u304C\u6B32\u3057\u304F\u306A\u3063\
    \u305F\u306E\u3067\u30E9\u30A4\u30D6\u30E9\u30EA\u5316\n\n#[derive(Debug, Clone,\
    \ Copy)]\nenum Inner<K: Copy, V: Copy> {\n    Empty,\n    One(K, V),\n    Two([(K,\
    \ V); 2]),\n}\n\n/// Top2(\u5927\u304D\u3044\u9806)\u307E\u3067\u306EMap\u3092\
    \u6301\u3064 \u305F\u3060\u3057\u540C\u3058Key\u306F\u4E00\u3064\u307E\u3067\n\
    #[derive(Debug, Clone, Copy)]\npub struct Top2Map<K: Eq + Copy, V: Ord + Copy>\
    \ {\n    map: Inner<K, V>,\n}\n\nimpl<K: Eq + Copy, V: Ord + Copy> Default for\
    \ Top2Map<K, V> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\
    \nimpl<K: Eq + Copy, V: Ord + Copy> Top2Map<K, V> {\n    pub fn new() -> Self\
    \ {\n        Self { map: Inner::Empty }\n    }\n    #[allow(clippy::collapsible_else_if)]\n\
    \    pub fn insert(&mut self, key: K, value: V) {\n        match self.map {\n\
    \            Inner::Empty => {\n                self.map = Inner::One(key, value);\n\
    \            }\n            Inner::One(k, v) => {\n                if key == k\
    \ {\n                    self.map = Inner::One(key, v.max(value));\n         \
    \       } else {\n                    if value > v {\n                       \
    \ self.map = Inner::Two([(key, value), (k, v)]);\n                    } else {\n\
    \                        self.map = Inner::Two([(k, v), (key, value)]);\n    \
    \                }\n                }\n            }\n            Inner::Two([(k1,\
    \ v1), (k2, v2)]) => {\n                if key == k1 {\n                    self.map\
    \ = Inner::Two([(key, v1.max(value)), (k2, v2)]);\n                } else if key\
    \ == k2 {\n                    let new_k2_val = v2.max(value);\n             \
    \       if new_k2_val > v1 {\n                        self.map = Inner::Two([(k2,\
    \ new_k2_val), (k1, v1)]);\n                    } else {\n                   \
    \     self.map = Inner::Two([(k1, v1), (k2, new_k2_val)]);\n                 \
    \   }\n                } else {\n                    if value > v1 {\n       \
    \                 self.map = Inner::Two([(key, value), (k1, v1)]);\n         \
    \           } else if value > v2 {\n                        self.map = Inner::Two([(k1,\
    \ v1), (key, value)]);\n                    }\n                }\n           \
    \ }\n        }\n    }\n    pub fn first(&self) -> Option<(K, V)> {\n        match\
    \ self.map {\n            Inner::Empty => None,\n            Inner::One(k, v)\
    \ => Some((k, v)),\n            Inner::Two([(k, v), _]) => Some((k, v)),\n   \
    \     }\n    }\n    pub fn second(&self) -> Option<(K, V)> {\n        match self.map\
    \ {\n            Inner::Empty => None,\n            Inner::One(_, _) => None,\n\
    \            Inner::Two([_, (k, v)]) => Some((k, v)),\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/top2/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-17 16:41:57+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/top2/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/top2/src/lib.rs
- /library/crates/misc/top2/src/lib.rs.html
title: crates/misc/top2/src/lib.rs
---
