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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn add(left: usize, right: usize) -> usize {\n    left + right\n}\n\n\
    #[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn it_works()\
    \ {\n        let result = add(2, 2);\n        assert_eq!(result, 4);\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/sa_lcp_array/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-07 12:00:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/string/sa_lcp_array/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/sa_lcp_array/src/lib.rs
- /library/crates/string/sa_lcp_array/src/lib.rs.html
title: crates/string/sa_lcp_array/src/lib.rs
---
