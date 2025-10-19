---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Rust\u306B\u306FMultiSet\u304C\u6A19\u6E96\u30E9\u30A4\u30D6\u30E9\u30EA\
    \u306B\u306A\u3044\u306E\u3067\u3001BTreeMap\u3067\u4EE3\u7528\u3057\u3066\u3044\
    \u305F\u304C\u3001\u9762\u5012\u306A\u306E\u3067\u30E9\u30A4\u30D6\u30E9\u30EA\
    \u5316  \n//! \u6700\u4F4E\u9650\u306E\u6A5F\u80FD\u3057\u304B\u63D0\u4F9B\u3057\
    \u3066\u3044\u306A\u3044\u306E\u3067\u3001`range`\u7B49\u4F7F\u3044\u305F\u3051\
    \u308C\u3070`buf`\u3084`buf_mut`\u3067\u5185\u90E8\u306EBTreeMap\u3092\u53D6\u308A\
    \u51FA\u3057\u3066\u4F7F\u3046\n\nuse std::borrow::Borrow;\nuse std::collections::BTreeMap;\n\
    \n#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]\npub struct MultiSet<K:\
    \ Ord> {\n    map: BTreeMap<K, usize>,\n}\n\nimpl<K: Ord> From<Vec<K>> for MultiSet<K>\
    \ {\n    fn from(vec: Vec<K>) -> Self {\n        let mut ret = MultiSet::new();\n\
    \        for v in vec {\n            ret.insert_one(v);\n        }\n        ret\n\
    \    }\n}\n\nimpl Default for MultiSet<usize> {\n    fn default() -> Self {\n\
    \        Self::new()\n    }\n}\n\nimpl<K: Ord> MultiSet<K> {\n    pub fn new()\
    \ -> Self {\n        Self {\n            map: BTreeMap::default(),\n        }\n\
    \    }\n\n    pub fn buf_mut(&mut self) -> &mut BTreeMap<K, usize> {\n       \
    \ &mut self.map\n    }\n\n    pub fn buf(&self) -> &BTreeMap<K, usize> {\n   \
    \     &self.map\n    }\n\n    /// key\u30921\u500B\u8FFD\u52A0\n    pub fn insert_one(&mut\
    \ self, key: K) {\n        self.map.entry(key).and_modify(|e| *e += 1).or_insert(1);\n\
    \    }\n\n    /// key\u3092c\u500B\u8FFD\u52A0\n    pub fn insert_bunch(&mut self,\
    \ key: K, c: usize) {\n        self.map.entry(key).and_modify(|e| *e += c).or_insert(c);\n\
    \    }\n\n    /// key\u3092\u4E00\u3064\u524A\u9664\u3059\u308B \u3082\u3068\u3082\
    \u3068key\u304C\u4E00\u3064\u4EE5\u4E0A\u3042\u308C\u3070true\u3092\u8FD4\u3059\
    \n    /// \u3082\u3068\u3082\u3068key\u304C\u306A\u3051\u308C\u3070false\u3092\
    \u8FD4\u3059\n    pub fn remove_one<Q>(&mut self, key: &Q) -> bool\n    where\n\
    \        K: Borrow<Q>,\n        Q: Ord + ?Sized,\n    {\n        if let Some(v)\
    \ = self.map.get_mut(key) {\n            *v -= 1;\n            if *v == 0 {\n\
    \                self.map.remove(key);\n            }\n            true\n    \
    \    } else {\n            false\n        }\n    }\n\n    /// key\u3092c\u500B\
    \u524A\u9664\u3059\u308B\n    pub fn remove_bunch<Q>(&mut self, key: &Q, c: usize)\n\
    \    where\n        K: Borrow<Q>,\n        Q: Ord + ?Sized,\n    {\n        if\
    \ let Some(v) = self.map.get_mut(key) {\n            *v = v.saturating_sub(c);\n\
    \            if *v == 0 {\n                self.map.remove(key);\n           \
    \ }\n        }\n    }\n\n    /// key\u3092\u3059\u3079\u3066\u524A\u9664\u3059\
    \u308B\n    pub fn remove_all<Q>(&mut self, key: &Q)\n    where\n        K: Borrow<Q>,\n\
    \        Q: Ord + ?Sized,\n    {\n        self.map.remove(key);\n    }\n\n   \
    \ pub fn contains_key<Q>(&self, key: &Q) -> bool\n    where\n        K: Borrow<Q>,\n\
    \        Q: Ord + ?Sized,\n    {\n        self.map.contains_key(key)\n    }\n\n\
    \    pub fn count<Q>(&self, key: &Q) -> usize\n    where\n        K: Borrow<Q>,\n\
    \        Q: Ord + ?Sized,\n    {\n        self.map.get(key).copied().unwrap_or(0)\n\
    \    }\n\n    pub fn is_empty(&self) -> bool {\n        self.map.is_empty()\n\
    \    }\n\n    pub fn min_key(&self) -> Option<&K> {\n        self.map.first_key_value().map(|(k,\
    \ _)| k)\n    }\n\n    pub fn max_key(&self) -> Option<&K> {\n        self.map.last_key_value().map(|(k,\
    \ _)| k)\n    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test() {\n        let mut rng = thread_rng();\n        let\
    \ mut ms = MultiSet::new();\n        let mut v = vec![];\n        for _ in 0..1000\
    \ {\n            let x = rng.gen_range(0..10);\n            let cnt = rng.gen_range(1..=10);\n\
    \            if rng.gen() {\n                ms.insert_one(x);\n             \
    \   v.push(x);\n            } else {\n                ms.insert_bunch(x, cnt);\n\
    \                v.extend(std::iter::repeat(x).take(cnt));\n            }\n  \
    \          let x = rng.gen_range(0..10);\n            let cnt = rng.gen_range(1..=5);\n\
    \            if rng.gen() {\n                ms.remove_one(&x);\n            \
    \    if let Some(pos) = v.iter().position(|&y| y == x) {\n                   \
    \ v.remove(pos);\n                }\n            } else {\n                ms.remove_bunch(&x,\
    \ cnt);\n                for _ in 0..cnt {\n                    if let Some(pos)\
    \ = v.iter().position(|&y| y == x) {\n                        v.remove(pos);\n\
    \                    }\n                }\n            }\n        }\n        for\
    \ x in v.iter() {\n            assert_eq!(ms.count(x), v.iter().filter(|&&y| y\
    \ == *x).count());\n        }\n        for x in v.iter() {\n            ms.remove_one(x);\n\
    \        }\n        assert!(ms.is_empty());\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/multiset/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-07 00:26:50+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/multiset/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/multiset/src/lib.rs
- /library/crates/misc/multiset/src/lib.rs.html
title: crates/misc/multiset/src/lib.rs
---
