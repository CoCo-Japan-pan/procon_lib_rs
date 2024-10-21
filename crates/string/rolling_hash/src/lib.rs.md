---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/hash/modint_mersenne/src/lib.rs
    title: crates/hash/modint_mersenne/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/zalgorithm_rolling_hash/src/main.rs
    title: verify/yosupo/zalgorithm_rolling_hash/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint_mersenne::ModIntMersenne;\nuse std::iter::once;\nuse std::ops::RangeBounds;\n\
    use std::time::SystemTime;\n\n/// \u5404\u63A5\u982D\u8F9E\u306E\u30CF\u30C3\u30B7\
    \u30E5\u5024\u3092\u4E8B\u524D\u8A08\u7B97\u3057\u3066\u304A\u304D\u3001\u9023\
    \u7D9A\u90E8\u5206\u5217\u306E\u30CF\u30C3\u30B7\u30E5\u5024\u3092`O(1)`\u3067\
    \u6C42\u3081\u308B\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct RollingHash\
    \ {\n    /// \u6587\u5B57\u5217\u306E\u9577\u3055\n    len: usize,\n    /// base\u306E\
    \u7D2F\u4E57\u306E\u30C6\u30FC\u30D6\u30EB[0..s.len()]\n    base_pow_table: Vec<ModIntMersenne>,\n\
    \    /// s\u306Eprefix\u306Ehash\u5024\u306E\u30C6\u30FC\u30D6\u30EB[0..s.len()]\n\
    \    prefix_hash_table: Vec<ModIntMersenne>,\n}\n\nimpl RollingHash {\n    ///\
    \ base\u7528\u306E\u4E71\u6570\u751F\u6210(\u6307\u5B9A\u3059\u308Bbase\u306E\u751F\
    \u6210\u306B\u3053\u308C\u3092\u4F7F\u3048\u307E\u3059)\n    pub fn get_random_base()\
    \ -> u64 {\n        let rand_duration = SystemTime::now()\n            .duration_since(SystemTime::UNIX_EPOCH)\n\
    \            .unwrap();\n        rand_duration.subsec_nanos() as u64 + rand_duration.as_secs()\n\
    \    }\n    /// s\u306Erolling hash\u3092\u69CB\u7BC9 `O(|s|)`  \n    /// \u8907\
    \u6570\u306E\u6587\u5B57\u5217\u306B\u7528\u3044\u3089\u308C\u308B\u5834\u5408\
    \u306Fbase\u3092\u6307\u5B9A\u3059\u308B\n    pub fn new<T: Into<u64> + Copy>(s:\
    \ &[T], base: Option<u64>) -> Self {\n        // base\u3068\u3057\u3066None\u304C\
    \u6307\u5B9A\u3055\u308C\u3066\u305F\u3089\u4E71\u6570\u3092\u751F\u6210(rand\u30AF\
    \u30EC\u30FC\u30C8\u3092\u4F7F\u3048\u306A\u3044\u5834\u5408\u3092\u8003\u616E\
    \u3057\u3066\u6642\u9593\u3067)\n        let base = if let Some(base) = base {\n\
    \            assert!(base > 1 && base < ModIntMersenne::modulus() - 1);\n    \
    \        base\n        } else {\n            Self::get_random_base()\n       \
    \ };\n        let base = ModIntMersenne::new(base);\n        let base_pow_table:\
    \ Vec<ModIntMersenne> = once(ModIntMersenne::new(1))\n            .chain((0..s.len()).scan(ModIntMersenne::new(1),\
    \ |acc, _| {\n                *acc *= base;\n                Some(*acc)\n    \
    \        }))\n            .collect();\n        let prefix_hash_table: Vec<ModIntMersenne>\
    \ = once(ModIntMersenne::new(0))\n            .chain(s.iter().scan(ModIntMersenne::new(0),\
    \ |acc, s| {\n                *acc = *acc * base + ModIntMersenne::new(Into::<u64>::into(*s));\n\
    \                Some(*acc)\n            }))\n            .collect();\n      \
    \  Self {\n            len: s.len(),\n            base_pow_table,\n          \
    \  prefix_hash_table,\n        }\n    }\n\n    pub fn len(&self) -> usize {\n\
    \        self.len\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.len\
    \ == 0\n    }\n\n    /// \u90E8\u5206\u5217`s[range]`\u306Ehash\u5024\u3092\u8FD4\
    \u3059 `O(1)`\n    pub fn get_hash<R: RangeBounds<usize>>(&self, range: R) ->\
    \ ModIntMersenne {\n        let l = match range.start_bound() {\n            std::ops::Bound::Included(&l)\
    \ => l,\n            std::ops::Bound::Excluded(&l) => l + 1,\n            std::ops::Bound::Unbounded\
    \ => 0,\n        };\n        let r = match range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.len,\n        };\n        assert!(l <= r && r <= self.len);\n      \
    \  self.prefix_hash_table[r] - self.prefix_hash_table[l] * self.base_pow_table[r\
    \ - l]\n    }\n\n    /// `base^i`\u3092\u8FD4\u3059\n    pub fn get_base_pow(&self,\
    \ i: usize) -> ModIntMersenne {\n        assert!(i <= self.len);\n        self.base_pow_table[i]\n\
    \    }\n\n    /// \u63A5\u982D\u8F9E\u306Ehash\u5024\u3092\u8FD4\u3059(`get_hash(0..i)`\u3068\
    \u540C\u3058\u3060\u304C\u3001\u30C6\u30FC\u30D6\u30EB\u304B\u3089\u5F15\u304F\
    )\n    pub fn get_prefix_hash(&self, i: usize) -> ModIntMersenne {\n        assert!(i\
    \ <= self.len);\n        self.prefix_hash_table[i]\n    }\n}\n"
  dependsOn:
  - crates/hash/modint_mersenne/src/lib.rs
  isVerificationFile: false
  path: crates/string/rolling_hash/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-21 15:52:33+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/zalgorithm_rolling_hash/src/main.rs
documentation_of: crates/string/rolling_hash/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/rolling_hash/src/lib.rs
- /library/crates/string/rolling_hash/src/lib.rs.html
title: crates/string/rolling_hash/src/lib.rs
---
