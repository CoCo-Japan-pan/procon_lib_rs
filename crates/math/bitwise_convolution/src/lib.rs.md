---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/math/zeta_mobius_bitset/src/lib.rs
    title: crates/math/zeta_mobius_bitset/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! bitwise AND, OR, XOR convolution\n\nuse std::ops::{Add, MulAssign, ShrAssign,\
    \ Sub};\nuse zeta_mobius_bitset::*;\n\n/// `c[k] = \u03A3 a[i] * b[j] (i | j ==\
    \ k)`\npub fn bitwise_or_convolution<T: Copy + Add<Output = T> + Sub<Output =\
    \ T> + MulAssign>(\n    a: &[T],\n    b: &[T],\n) -> Vec<T> {\n    assert_eq!(a.len(),\
    \ b.len());\n    let mut a = subset_zeta(a.to_vec(), |x, y| x + y);\n    let b\
    \ = subset_zeta(b.to_vec(), |x, y| x + y);\n    let n = a.len();\n    for i in\
    \ 0..n {\n        a[i] *= b[i];\n    }\n    subset_mobius(a)\n}\n\n/// `c[k] =\
    \ \u03A3 a[i] * b[j] (i & j == k)`\npub fn bitwise_and_convolution<T: Copy + Add<Output\
    \ = T> + Sub<Output = T> + MulAssign>(\n    a: &[T],\n    b: &[T],\n) -> Vec<T>\
    \ {\n    assert_eq!(a.len(), b.len());\n    let mut a = superset_zeta(a.to_vec(),\
    \ |x, y| x + y);\n    let b = superset_zeta(b.to_vec(), |x, y| x + y);\n    let\
    \ n = a.len();\n    for i in 0..n {\n        a[i] *= b[i];\n    }\n    superset_mobius(a)\n\
    }\n\n/// `c[k] = \u03A3 a[i] * b[j] (i ^ j == k)`\npub fn bitwise_xor_convolution<\n\
    \    T: Copy + Add<Output = T> + Sub<Output = T> + MulAssign + ShrAssign<u32>,\n\
    >(\n    a: &[T],\n    b: &[T],\n) -> Vec<T> {\n    assert_eq!(a.len(), b.len());\n\
    \    let mut a = fast_hadamard(a.to_vec());\n    let b = fast_hadamard(b.to_vec());\n\
    \    let n = a.len();\n    for i in 0..n {\n        a[i] *= b[i];\n    }\n   \
    \ inv_fast_hadamard(a)\n}\n\n/// `H~n = 2^(n/2) Hn` \u306E\u884C\u5217\u3092\u304B\
    \u3051\u308B `Hn`\u306F\u30A2\u30C0\u30DE\u30FC\u30EB\u884C\u5217\npub fn fast_hadamard<T:\
    \ Copy + Add<Output = T> + Sub<Output = T>>(mut list: Vec<T>) -> Vec<T> {\n  \
    \  let n = list.len();\n    assert!(n.is_power_of_two());\n    let bit = n.trailing_zeros();\n\
    \    for i in 0..bit {\n        for j in 0..n {\n            if j & (1 << i) ==\
    \ 0 {\n                let x = list[j];\n                let y = list[j | (1 <<\
    \ i)];\n                list[j] = x + y;\n                list[j | (1 << i)] =\
    \ x - y;\n            }\n        }\n    }\n    list\n}\n\n/// `fast_hadamard`\u306E\
    \u9006\u884C\u5217\u3092\u304B\u3051\u308B  \n/// `(H~n)^(-1) = 2^(-n/2) Hn^(-1)\
    \ = 2^(-n/2) Hn = 2^(-n) H~n`  \n/// \u3064\u307E\u308A`fast_hadamard`\u3092\u3057\
    \u3066\u5168\u4F53\u3092\u9577\u3055\u3067\u5272\u308B\u3060\u3051\npub fn inv_fast_hadamard<T:\
    \ Copy + Add<Output = T> + Sub<Output = T> + ShrAssign<u32>>(\n    mut list: Vec<T>,\n\
    ) -> Vec<T> {\n    list = fast_hadamard(list);\n    let bit = list.len().trailing_zeros();\n\
    \    list.iter_mut().for_each(|x| *x >>= bit);\n    list\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use super::*;\n    use rand::prelude::*;\n\n    #[test]\n   \
    \ fn test_bitor_conv() {\n        let mut rng = thread_rng();\n        let n =\
    \ 1 << 10;\n        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let c = bitwise_or_convolution(&a, &b);\n        let mut c_naive = vec![0;\
    \ n];\n        for i in 0..n {\n            for j in 0..n {\n                c_naive[i\
    \ | j] += a[i] * b[j];\n            }\n        }\n        assert_eq!(c, c_naive);\n\
    \    }\n\n    #[test]\n    fn test_bitand_conv() {\n        let mut rng = thread_rng();\n\
    \        let n = 1 << 10;\n        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let c = bitwise_and_convolution(&a, &b);\n        let mut c_naive = vec![0;\
    \ n];\n        for i in 0..n {\n            for j in 0..n {\n                c_naive[i\
    \ & j] += a[i] * b[j];\n            }\n        }\n        assert_eq!(c, c_naive);\n\
    \    }\n\n    #[test]\n    fn test_xor_conv() {\n        let mut rng = thread_rng();\n\
    \        let n = 1 << 10;\n        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();\n\
    \        let c = bitwise_xor_convolution(&a, &b);\n        let mut c_naive = vec![0;\
    \ n];\n        for i in 0..n {\n            for j in 0..n {\n                c_naive[i\
    \ ^ j] += a[i] * b[j];\n            }\n        }\n        assert_eq!(c, c_naive);\n\
    \    }\n}\n"
  dependsOn:
  - crates/math/zeta_mobius_bitset/src/lib.rs
  isVerificationFile: false
  path: crates/math/bitwise_convolution/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-09 15:54:02+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/bitwise_convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/bitwise_convolution/src/lib.rs
- /library/crates/math/bitwise_convolution/src/lib.rs.html
title: crates/math/bitwise_convolution/src/lib.rs
---
