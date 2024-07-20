---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc290f/src/main.rs
    title: verify/AtCoder/abc290f/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_modint::ModInt;\n\npub struct Binom<T: ModInt> {\n    max_n:\
    \ usize,\n    fact: Vec<T>,\n    ifact: Vec<T>,\n}\n\nimpl<T: ModInt> Binom<T>\
    \ {\n    pub fn new(max_n: usize) -> Self {\n        let mut fact = vec![T::raw(0);\
    \ max_n + 1];\n        let mut ifact = vec![T::raw(0); max_n + 1];\n        fact[0]\
    \ = T::raw(1);\n        for i in 1..=max_n {\n            fact[i] = fact[i - 1]\
    \ * T::new(i as u64);\n        }\n        ifact[max_n] = fact[max_n].inv();\n\
    \        for i in (1..=max_n).rev() {\n            ifact[i - 1] = ifact[i] * T::new(i\
    \ as u64);\n        }\n        Self { max_n, fact, ifact }\n    }\n\n    /// nCk\u306E\
    \u8A08\u7B97(n<k\u306E\u5834\u5408\u306F0\u3068\u3059\u308B)\n    pub fn cmp(&self,\
    \ n: usize, k: usize) -> T {\n        assert!(n <= self.max_n);\n        if n\
    \ < k {\n            T::raw(0)\n        } else {\n            self.fact[n] * self.ifact[k]\
    \ * self.ifact[n - k]\n        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  isVerificationFile: false
  path: crates/math/binom/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-20 13:46:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc290f/src/main.rs
documentation_of: crates/math/binom/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/binom/src/lib.rs
- /library/crates/math/binom/src/lib.rs.html
title: crates/math/binom/src/lib.rs
---
