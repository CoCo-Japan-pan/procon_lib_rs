---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/modint_traits/src/lib.rs
    title: crates/internals/modint_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
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
  code: "use modint_traits::ModInt;\n\n/// [1, n)\u306Emod\u9006\u5143\u5217\u6319\
    \u3092`O(n)`\u3067\u884C\u3046 (index 0\u306B\u306F\u4FBF\u5B9C\u7684\u306B0\u3092\
    \u5165\u308C\u3066\u304A\u304F)\npub fn enumerate_invs<M: ModInt>(n: usize) ->\
    \ Vec<M> {\n    assert!(n <= M::modulus() as usize);\n    let mut invs = vec![M::raw(0);\
    \ n];\n    if n <= 1 {\n        return invs;\n    }\n    invs[1] = M::raw(1);\n\
    \    for i in 2..n {\n        invs[i] = -invs[M::modulus() as usize % i] * M::raw(M::modulus()\
    \ / i as u32);\n    }\n    invs\n}\n"
  dependsOn:
  - crates/internals/modint_traits/src/lib.rs
  isVerificationFile: false
  path: crates/math/enumerate_inv_mods/src/lib.rs
  requiredBy:
  - crates/fps/fps_utils/src/lib.rs
  timestamp: '2024-05-30 18:25:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/enumerate_inv_mods/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/enumerate_inv_mods/src/lib.rs
- /library/crates/math/enumerate_inv_mods/src/lib.rs.html
title: crates/math/enumerate_inv_mods/src/lib.rs
---
