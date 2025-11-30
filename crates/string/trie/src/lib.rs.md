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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u30C8\u30E9\u30A4\u6728(\u306E\u30D9\u30FC\u30B9\u5B9F\u88C5)  \n//!\
    \ \u540C\u3058prefix\u3092\u307E\u3068\u3081\u3066\u51E6\u7406\u3067\u304D\u308B\
    \  \n//! Rolling Hash\u3067\u4EE3\u66FF\u3067\u304D\u308B\u3053\u3068\u3082\u3042\
    \u308B\n\n/// \u30A2\u30EB\u30D5\u30A1\u30D9\u30C3\u30C8\u5C0F\u6587\u5B57\u3092\
    Bytes\u5F62\u5F0F\u3067\u53D7\u3051\u53D6\u308B\u3053\u3068\u3092\u4EEE\u5B9A\u3057\
    \u3066\u3044\u308B\n#[derive(Debug, Clone, Default)]\npub struct Trie {\n    chidlren:\
    \ [Option<Box<Trie>>; 26],\n    /// \u305D\u306E\u90E8\u5206\u6728\u5185\u306B\
    \u542B\u307E\u308C\u308B\u6587\u5B57\u5217\u306E\u500B\u6570\n    cnt: usize,\n\
    \    /// \u53D7\u7406\u3059\u308B\u304B\n    is_end: bool,\n}\n\nimpl Trie {\n\
    \    pub fn new() -> Self {\n        Self::default()\n    }\n\n    pub fn insert(&mut\
    \ self, s: &[u8]) {\n        let mut node = self;\n        for &c in s {\n   \
    \         let idx = (c - b'a') as usize;\n            if node.chidlren[idx].is_none()\
    \ {\n                node.chidlren[idx] = Some(Box::new(Trie::new()));\n     \
    \       }\n            node = node.chidlren[idx].as_mut().unwrap();\n        \
    \    node.cnt += 1;\n        }\n        node.is_end = true;\n    }\n\n    ///\
    \ s\u3092\u63A5\u982D\u8F9E\u306B\u3082\u3064\u6587\u5B57\u5217\u3092\u5168\u3066\
    \u6D88\u3059(\u90E8\u5206\u6728\u3092\u524A\u9664\u3059\u308B)  \n    /// \u6D88\
    \u3057\u305F\u6587\u5B57\u5217\u306E\u500B\u6570\u3092\u8FD4\u3059\n    pub fn\
    \ delete_prefix(&mut self, s: &[u8]) -> usize {\n        if s.is_empty() {\n \
    \           let ret = self.cnt;\n            *self = Trie::new();\n          \
    \  return ret;\n        }\n        if let Some(child) = &mut self.chidlren[(s[0]\
    \ - b'a') as usize] {\n            let cnt = child.delete_prefix(&s[1..]);\n \
    \           self.cnt -= cnt;\n            cnt\n        } else {\n            0\n\
    \        }\n    }\n\n    /// s\u3092\u63A5\u982D\u8F9E\u306B\u6301\u3064\u6587\
    \u5B57\u5217\u304C\u5B58\u5728\u3059\u308B\u304B\n    pub fn find_prefix(&self,\
    \ s: &[u8]) -> bool {\n        let mut node = self;\n        for &c in s {\n \
    \           let idx = (c - b'a') as usize;\n            if let Some(child) = &node.chidlren[idx]\
    \ {\n                node = child;\n                if node.is_end {\n       \
    \             return true;\n                }\n            } else {\n        \
    \        break;\n            }\n        }\n        false\n    }\n\n    /// s\u304C\
    \u5B58\u5728\u3059\u308B\u304B\n    pub fn find(&self, s: &[u8]) -> bool {\n \
    \       let mut node = self;\n        for &c in s {\n            let idx = (c\
    \ - b'a') as usize;\n            if let Some(child) = &node.chidlren[idx] {\n\
    \                node = child;\n            } else {\n                return false;\n\
    \            }\n        }\n        node.is_end\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-28 01:05:50+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/string/trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/trie/src/lib.rs
- /library/crates/string/trie/src/lib.rs.html
title: crates/string/trie/src/lib.rs
---
