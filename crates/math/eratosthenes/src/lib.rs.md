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
    - https://qiita.com/drken/items/3beb679e54266f20ab63
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [\u30A8\u30E9\u30C8\u30B9\u30C6\u30CD\u30B9\u306E\u7BE9](https://qiita.com/drken/items/3beb679e54266f20ab63)\n\
    \npub struct Eratosthenes {\n    max_n: usize,\n    primes: Vec<usize>,\n    min_factor:\
    \ Vec<usize>,\n}\n\nimpl Eratosthenes {\n    /// `O(NloglogN)`\n    pub fn new(max_n:\
    \ usize) -> Self {\n        let mut min_factor = vec![0; max_n + 1];\n       \
    \ let mut primes = vec![];\n        min_factor[1] = 1;\n        for num in 2..=max_n\
    \ {\n            if min_factor[num] != 0 {\n                continue;\n      \
    \      }\n            primes.push(num);\n            let mut cur_num = num;\n\
    \            while cur_num <= max_n {\n                if min_factor[cur_num]\
    \ == 0 {\n                    min_factor[cur_num] = num;\n                }\n\
    \                cur_num += num;\n            }\n        }\n        Self {\n \
    \           max_n,\n            primes,\n            min_factor,\n        }\n\
    \    }\n\n    pub fn is_prime(&self, n: usize) -> bool {\n        assert!(n <=\
    \ self.max_n);\n        n >= 2 && self.min_factor[n] == n\n    }\n\n    pub fn\
    \ get_primes(&self) -> &[usize] {\n        &self.primes\n    }\n\n    /// `O(log\
    \ N)` \u3067\u7D20\u56E0\u6570\u5206\u89E3  \n    /// (\u7D20\u56E0\u6570\u3001\
    \u3079\u304D) \u306E\u914D\u5217\u3092\u8FD4\u3059\n    pub fn factorize(&self,\
    \ mut n: usize) -> Vec<(usize, usize)> {\n        assert!(n <= self.max_n);\n\
    \        let mut res = vec![];\n        while n > 1 {\n            let p = self.min_factor[n];\n\
    \            let mut cnt = 0;\n            while self.min_factor[n] == p {\n \
    \               cnt += 1;\n                n /= p;\n            }\n          \
    \  res.push((p, cnt));\n        }\n        res\n    }\n\n    /// \u7D04\u6570\u306E\
    \u500B\u6570\u30AA\u30FC\u30C0\u30FC\u3067\u7D04\u6570\u5217\u6319 \u6700\u5F8C\
    \u306B\u30BD\u30FC\u30C8\u3057\u3066\u3044\u308B\n    pub fn divisors(&self, n:\
    \ usize) -> Vec<usize> {\n        let mut ret = vec![1];\n        let pc = self.factorize(n);\n\
    \        for (p, c) in pc {\n            let cur_size = ret.len();\n         \
    \   for i in 0..cur_size {\n                let mut new_num = ret[i];\n      \
    \          for _ in 0..c {\n                    new_num *= p;\n              \
    \      ret.push(new_num);\n                }\n            }\n        }\n     \
    \   ret.sort_unstable();\n        ret\n    }\n}\n\n#[cfg(test)]\nmod test {\n\
    \    use super::*;\n\n    #[test]\n    fn test_divisors_60() {\n        let era\
    \ = Eratosthenes::new(60);\n        let divisors_60 = era.divisors(60);\n    \
    \    assert_eq!(divisors_60, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60])\n   \
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/eratosthenes/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-06 16:15:33+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/eratosthenes/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/eratosthenes/src/lib.rs
- /library/crates/math/eratosthenes/src/lib.rs.html
title: crates/math/eratosthenes/src/lib.rs
---
