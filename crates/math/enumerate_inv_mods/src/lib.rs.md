---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_modint::ModInt;\n\n/// n\u4EE5\u4E0B\u306Emod\u9006\u5143\u5217\
    \u6319\u3092`O(n)`\u3067\u884C\u3046  \n/// index 0\u306B\u306F\u4FBF\u5B9C\u7684\
    \u306B0\u3092\u5165\u308C\u305Fn+1\u306E\u9577\u3055\u306E\u914D\u5217\u3092\u8FD4\
    \u3059\npub fn enumerate_invs<M: ModInt>(n: usize) -> Vec<M> {\n    let n = n\
    \ + 1;\n    assert!(n <= M::modulus() as usize);\n    let mut invs = vec![M::raw(0);\
    \ n];\n    if n <= 1 {\n        return invs;\n    }\n    invs[1] = M::raw(1);\n\
    \    for i in 2..n {\n        invs[i] = -invs[M::modulus() as usize % i] * M::raw(M::modulus()\
    \ / i as u32);\n    }\n    invs\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n\
    \    #[test]\n    fn test() {\n        use static_modint::ModInt998244353 as MInt;\n\
    \        const SIZE: usize = 1_000_000;\n        let invs = enumerate_invs::<MInt>(SIZE);\n\
    \        for i in 1..=SIZE {\n            assert_eq!(invs[i] * MInt::new(i), MInt::new(1));\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  isVerificationFile: false
  path: crates/math/enumerate_inv_mods/src/lib.rs
  requiredBy:
  - crates/fps/fps_utils/src/lib.rs
  timestamp: '2024-10-18 21:07:52+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/enumerate_inv_mods/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/enumerate_inv_mods/src/lib.rs
- /library/crates/math/enumerate_inv_mods/src/lib.rs.html
title: crates/math/enumerate_inv_mods/src/lib.rs
---
