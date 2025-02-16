---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/abc290f/src/main.rs
    title: verify/AtCoder/abc290f/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u968E\u4E57\u3068\u305D\u306E\u9006\u5143\u3092\u524D\u8A08\u7B97\u3057\
    \u3001\u4E8C\u9805\u4FC2\u6570\u3092\u8A08\u7B97\u3059\u308B\n\nuse internal_modint::ModInt;\n\
    \npub struct Binom<T: ModInt> {\n    max_n: usize,\n    fact: Vec<T>,\n    ifact:\
    \ Vec<T>,\n}\n\nimpl<T: ModInt> Binom<T> {\n    pub fn new(max_n: usize) -> Self\
    \ {\n        let mut fact = vec![T::raw(0); max_n + 1];\n        let mut ifact\
    \ = vec![T::raw(0); max_n + 1];\n        fact[0] = T::raw(1);\n        for i in\
    \ 1..=max_n {\n            fact[i] = fact[i - 1] * T::new(i as u64);\n       \
    \ }\n        ifact[max_n] = fact[max_n].inv();\n        for i in (1..=max_n).rev()\
    \ {\n            ifact[i - 1] = ifact[i] * T::new(i as u64);\n        }\n    \
    \    Self { max_n, fact, ifact }\n    }\n\n    /// nCk\u306E\u8A08\u7B97(n<k\u306E\
    \u5834\u5408\u306F0\u3068\u3059\u308B)\n    pub fn comb(&self, n: usize, k: usize)\
    \ -> T {\n        assert!(n <= self.max_n);\n        if n < k {\n            T::raw(0)\n\
    \        } else {\n            self.fact[n] * self.ifact[k] * self.ifact[n - k]\n\
    \        }\n    }\n\n    /// nPk\u306E\u8A08\u7B97(n<k\u306E\u5834\u5408\u306F\
    0\u3068\u3059\u308B)\n    pub fn perm(&self, n: usize, k: usize) -> T {\n    \
    \    assert!(n <= self.max_n);\n        if n < k {\n            T::raw(0)\n  \
    \      } else {\n            self.fact[n] * self.ifact[n - k]\n        }\n   \
    \ }\n\n    /// n!\n    pub fn get_fact(&self, n: usize) -> T {\n        assert!(n\
    \ <= self.max_n);\n        self.fact[n]\n    }\n\n    /// n!\u306E\u9006\u5143\
    \n    pub fn get_ifact(&self, n: usize) -> T {\n        assert!(n <= self.max_n);\n\
    \        self.ifact[n]\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  isVerificationFile: false
  path: crates/math/binom/src/lib.rs
  requiredBy:
  - verify/AtCoder/abc290f/src/main.rs
  timestamp: '2025-01-13 11:38:32+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/binom/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/binom/src/lib.rs
- /library/crates/math/binom/src/lib.rs.html
title: crates/math/binom/src/lib.rs
---
