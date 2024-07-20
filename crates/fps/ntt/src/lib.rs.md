---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
    title: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_ntt/src/main.rs
    title: verify/yosupo/convolution_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
    title: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/convolution.rs)
    - https://tayu0110.hatenablog.com/entry/2023/05/06/023244)
    - https://www.creativ.xyz/fast-fourier-transform/)
    - https://www.mathenachia.blog/ntt-mod-list-01/)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [FFT](https://www.creativ.xyz/fast-fourier-transform/)  \n//! [\u539F\
    \u59CB\u6839, NTT friendly MOD](https://www.mathenachia.blog/ntt-mod-list-01/)\
    \  \n//! [\u9AD8\u901F\u5316](https://tayu0110.hatenablog.com/entry/2023/05/06/023244)\
    \  \n//! [Reference](https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/convolution.rs)\
    \  \n//! \u30B8\u30A7\u30CD\u30EA\u30C3\u30AF\u306B\u7D10\u3065\u304Fstatic\u306E\
    \u5B9F\u73FE\u304C\u3080\u305A\u304F\u3066\u30AD\u30E3\u30C3\u30B7\u30E5\u306F\
    \u6BCE\u56DE\u3068\u308B\u3053\u3068\u306B\u3057\u305F...  \n\nuse dynamic_modint::{DynamicModInt,\
    \ ModContainer};\nuse internal_modint::{inv_gcd, safe_mod, ModInt};\nuse internal_type_traits::Zero;\n\
    use static_modint::StaticModInt;\nuse std::ops::{AddAssign, Mul};\n\nfn prepare<const\
    \ NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n) -> ([StaticModInt<NTT_MOD>; 30],\
    \ [StaticModInt<NTT_MOD>; 30]) {\n    let g = StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT);\n\
    \    let mut es = [StaticModInt::<NTT_MOD>::raw(0); 30];\n    let mut ies = [StaticModInt::<NTT_MOD>::raw(0);\
    \ 30];\n    let cnt2 = (NTT_MOD - 1).trailing_zeros() as usize;\n    let mut e\
    \ = g.pow(((NTT_MOD - 1) >> cnt2).into());\n    let mut ie = e.inv();\n    for\
    \ i in (2..=cnt2).rev() {\n        es[i - 2] = e;\n        ies[i - 2] = ie;\n\
    \        e *= e;\n        ie *= ie;\n    }\n    for i in 1..30 {\n        es[i]\
    \ *= es[i - 1];\n        ies[i] *= ies[i - 1];\n    }\n    (es, ies)\n}\n\nfn\
    \ ntt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n    data: &mut [StaticModInt<NTT_MOD>],\n\
    \    sum_e: &[StaticModInt<NTT_MOD>; 30],\n) {\n    let size = data.len();\n \
    \   assert!(size.is_power_of_two());\n    let height = size.next_power_of_two().trailing_zeros();\n\
    \    for ph in 1..=height {\n        let w = 1 << (ph - 1);\n        let p = 1\
    \ << (height - ph);\n        let mut now = StaticModInt::<NTT_MOD>::raw(1);\n\
    \        for s in 0..w {\n            let offset = s << (height - ph + 1);\n \
    \           for i in 0..p {\n                let l = data[i + offset];\n     \
    \           let r = data[i + offset + p] * now;\n                data[i + offset]\
    \ = l + r;\n                data[i + offset + p] = l - r;\n            }\n   \
    \         now *= sum_e[(!s).trailing_zeros() as usize];\n        }\n    }\n}\n\
    \nfn intt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n    data: &mut [StaticModInt<NTT_MOD>],\n\
    \    sum_ie: &[StaticModInt<NTT_MOD>; 30],\n) {\n    let size = data.len();\n\
    \    assert!(size.is_power_of_two());\n    let height = size.next_power_of_two().trailing_zeros();\n\
    \    for ph in (1..=height).rev() {\n        let w = 1 << (ph - 1);\n        let\
    \ p = 1 << (height - ph);\n        let mut inow = StaticModInt::<NTT_MOD>::raw(1);\n\
    \        for s in 0..w {\n            let offset = s << (height - ph + 1);\n \
    \           for i in 0..p {\n                let l = data[i + offset];\n     \
    \           let r = data[i + offset + p];\n                data[i + offset] =\
    \ l + r;\n                data[i + offset + p] = (l - r) * inow;\n           \
    \ }\n            inow *= sum_ie[(!s).trailing_zeros() as usize];\n        }\n\
    \    }\n}\n\nfn convolution_naive<M: Zero + AddAssign + Mul<Output = M> + Copy>(a:\
    \ &[M], b: &[M]) -> Vec<M> {\n    let (n, m) = (a.len(), b.len());\n    let mut\
    \ ret = vec![M::zero(); n + m - 1];\n    for (i, j) in (0..n).flat_map(|i| (0..m).map(move\
    \ |j| (i, j))) {\n        ret[i + j] += a[i] * b[j];\n    }\n    ret\n}\n\n///\
    \ NTT\u306B\u3088\u308B\u7573\u307F\u8FBC\u307F\nfn convolution_ntt_friendly<const\
    \ NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n    a: &[StaticModInt<NTT_MOD>],\n\
    \    b: &[StaticModInt<NTT_MOD>],\n) -> Vec<StaticModInt<NTT_MOD>> {\n    if a.is_empty()\
    \ || b.is_empty() {\n        return vec![];\n    }\n    if a.len().min(b.len())\
    \ <= 60 {\n        return convolution_naive(a, b);\n    }\n    let n = a.len()\
    \ + b.len() - 1;\n    let size = n.next_power_of_two();\n    // NTT_MOD\u306F\
    1\u306Esize\u4E57\u6839\u3092\u6301\u3064\u306F\u305A\n    assert!((NTT_MOD -\
    \ 1) % size as u32 == 0);\n    let mut a = a.to_owned();\n    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));\n\
    \    let (sum_e, sum_ie) = prepare::<NTT_MOD, PRIMITIVE_ROOT>();\n    ntt::<NTT_MOD,\
    \ PRIMITIVE_ROOT>(&mut a, &sum_e);\n    let mut b = b.to_owned();\n    b.resize(size,\
    \ StaticModInt::<NTT_MOD>::raw(0));\n    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b,\
    \ &sum_e);\n    for (a, b) in a.iter_mut().zip(b) {\n        *a *= b;\n    }\n\
    \    intt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, &sum_ie);\n    a.resize(n, StaticModInt::<NTT_MOD>::raw(0));\n\
    \    let inv_size = StaticModInt::<NTT_MOD>::raw(size as u32).inv();\n    for\
    \ a in a.iter_mut() {\n        *a *= inv_size;\n    }\n    a\n}\n\n/// \u53D6\u308A\
    \u3046\u308B\u6700\u5927\u5024\u3092\u8D85\u3048\u308Bmod\u3092\u8868\u73FE\u3067\
    \u304D\u308B\u3088\u3046\u306Amod\u306E\u7D44\u3092\u9078\u3093\u3067\u7573\u307F\
    \u8FBC\u307F\u3001Garner\u3067\u5FA9\u5143\nfn convolution_aribtrary_u32_mod<M:\
    \ ModInt>(a: &[M], b: &[M]) -> Vec<M> {\n    const G_MOD1: u32 = 167_772_161;\n\
    \    const G_MOD2: u32 = 469_762_049;\n    const G_MOD3: u32 = 1_224_736_769;\n\
    \    let x = convolution_ntt_friendly::<G_MOD1, 3>(\n        a.iter()\n      \
    \      .map(|x| StaticModInt::<G_MOD1>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n        b.iter()\n            .map(|x| StaticModInt::<G_MOD1>::new(x.value()))\n\
    \            .collect::<Vec<_>>()\n            .as_slice(),\n    );\n    let y\
    \ = convolution_ntt_friendly::<G_MOD2, 3>(\n        a.iter()\n            .map(|x|\
    \ StaticModInt::<G_MOD2>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n        b.iter()\n            .map(|x| StaticModInt::<G_MOD2>::new(x.value()))\n\
    \            .collect::<Vec<_>>()\n            .as_slice(),\n    );\n    let z\
    \ = convolution_ntt_friendly::<G_MOD3, 3>(\n        a.iter()\n            .map(|x|\
    \ StaticModInt::<G_MOD3>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n        b.iter()\n            .map(|x| StaticModInt::<G_MOD3>::new(x.value()))\n\
    \            .collect::<Vec<_>>()\n            .as_slice(),\n    );\n\n    let\
    \ m1_inv_m2 = StaticModInt::<G_MOD2>::new(G_MOD1).inv();\n    let m12_inv_m3 =\
    \ StaticModInt::<G_MOD3>::new(G_MOD1 as u64 * G_MOD2 as u64).inv();\n    let m12_mod\
    \ = M::new(G_MOD1 as u64 * G_MOD2 as u64);\n    let mut ret = vec![M::raw(0);\
    \ x.len()];\n    for (i, r) in ret.iter_mut().enumerate() {\n        let v1 =\
    \ ((StaticModInt::<G_MOD2>::new(y[i].value())\n            - StaticModInt::<G_MOD2>::new(x[i].value()))\n\
    \            * m1_inv_m2)\n            .value();\n        let v2 = ((StaticModInt::<G_MOD3>::new(z[i].value())\n\
    \            - StaticModInt::<G_MOD3>::new(x[i].value())\n            - StaticModInt::<G_MOD3>::new(G_MOD1)\
    \ * StaticModInt::<G_MOD3>::new(v1))\n            * m12_inv_m3)\n            .value();\n\
    \        let constants = M::new(x[i].value()) + M::new(G_MOD1) * M::new(v1) +\
    \ m12_mod * M::new(v2);\n        *r = constants;\n    }\n    ret\n}\n\n/// ModInt\u306B\
    \u7573\u307F\u8FBC\u307F\u3082\u8FFD\u52A0\u3057\u305F\u30C8\u30EC\u30A4\u30C8\
    \npub trait ConvHelper: ModInt {\n    fn convolution(a: &[Self], b: &[Self]) ->\
    \ Vec<Self>;\n}\n\nimpl<const MOD: u32> ConvHelper for StaticModInt<MOD> {\n \
    \   fn convolution(a: &[Self], b: &[Self]) -> Vec<Self> {\n        match MOD {\n\
    \            998_244_353 | 167_772_161 | 469_762_049 | 1_224_736_769 | 4_194_304_001\
    \ => {\n                convolution_ntt_friendly::<MOD, 3>(a, b)\n           \
    \ }\n            754_974_721 => convolution_ntt_friendly::<MOD, 11>(a, b),\n \
    \           _ => convolution_aribtrary_u32_mod(a, b),\n        }\n    }\n}\n\n\
    impl<MOD: ModContainer> ConvHelper for DynamicModInt<MOD> {\n    fn convolution(a:\
    \ &[Self], b: &[Self]) -> Vec<Self> {\n        convolution_aribtrary_u32_mod(a,\
    \ b)\n    }\n}\n\n/// NTT-freindly\u306A\u5834\u5408\u3082\u305D\u3046\u3067\u306A\
    \u3044\u5834\u5408\u3082\u5305\u62EC\u3059\u308B\npub fn convolution<M: ConvHelper>(a:\
    \ &[M], b: &[M]) -> Vec<M> {\n    M::convolution(a, b)\n}\n\nfn convolution_raw<M>(a:\
    \ &[i64], b: &[i64]) -> Vec<i64>\nwhere\n    M: ConvHelper,\n{\n    let a = a.iter().map(|&x|\
    \ M::new(x)).collect::<Vec<_>>();\n    let b = b.iter().map(|&x| M::new(x)).collect::<Vec<_>>();\n\
    \    convolution::<M>(&a, &b)\n        .into_iter()\n        .map(|x| x.value()\
    \ as i64)\n        .collect()\n}\n\n/// i64\u306B\u5024\u304C\u53CE\u307E\u308B\
    \u5834\u5408\u306E\u7573\u307F\u8FBC\u307F(\u8CA0\u306E\u5024\u3082\u6271\u3048\
    \u308B)  \npub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {\n    const\
    \ M1: u64 = 754_974_721; // 2^24\n    const M2: u64 = 167_772_161; // 2^25\n \
    \   const M3: u64 = 469_762_049; // 2^26\n    const M2M3: u64 = M2 * M3;\n   \
    \ const M1M3: u64 = M1 * M3;\n    const M1M2: u64 = M1 * M2;\n    const M1M2M3:\
    \ u64 = M1M2.wrapping_mul(M3);\n\n    if a.is_empty() || b.is_empty() {\n    \
    \    return vec![];\n    }\n\n    if a.len().min(b.len()) <= 60 {\n        return\
    \ convolution_naive(a, b);\n    }\n\n    const I1: i64 = inv_gcd(M2M3 as i64,\
    \ M1 as i64).1;\n    const I2: i64 = inv_gcd(M1M3 as i64, M2 as i64).1;\n    const\
    \ I3: i64 = inv_gcd(M1M2 as i64, M3 as i64).1;\n\n    let (c1, c2, c3) = {\n \
    \       const M1: u32 = 754_974_721;\n        const M2: u32 = 167_772_161;\n \
    \       const M3: u32 = 469_762_049;\n        (\n            convolution_raw::<StaticModInt<M1>>(a,\
    \ b),\n            convolution_raw::<StaticModInt<M2>>(a, b),\n            convolution_raw::<StaticModInt<M3>>(a,\
    \ b),\n        )\n    };\n\n    c1.into_iter()\n        .zip(c2)\n        .zip(c3)\n\
    \        .map(|((c1, c2), c3)| {\n            const OFFSET: &[u64] = &[0, 0, M1M2M3,\
    \ 2 * M1M2M3, 3 * M1M2M3];\n\n            let mut x = [(c1, I1, M1, M2M3), (c2,\
    \ I2, M2, M1M3), (c3, I3, M3, M1M2)]\n                .iter()\n              \
    \  .map(|&(c, i, m1, m2)| c.wrapping_mul(i).rem_euclid(m1 as _).wrapping_mul(m2\
    \ as _))\n                .fold(0, i64::wrapping_add);\n\n            let mut\
    \ diff = c1 - safe_mod(x, M1 as _);\n            if diff < 0 {\n             \
    \   diff += M1 as i64;\n            }\n            x = x.wrapping_sub(OFFSET[diff.rem_euclid(5)\
    \ as usize] as _);\n            x\n        })\n        .collect()\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n    use rand::prelude::*;\n\n    #[test]\n    fn\
    \ test_convolution_i64() {\n        fn do_test(size: u32) {\n            let mut\
    \ rng = thread_rng();\n            let a = (0..size)\n                .map(|_|\
    \ rng.gen_range(-10_000_000..=10_000_000))\n                .collect::<Vec<_>>();\n\
    \            let b = (0..size)\n                .map(|_| rng.gen_range(-10_000_000..=10_000_000))\n\
    \                .collect::<Vec<_>>();\n            let naive = convolution_naive(&a,\
    \ &b);\n            let fast = convolution_i64(&a, &b);\n            assert_eq!(naive,\
    \ fast);\n        }\n        do_test(1);\n        do_test(10);\n        do_test(100);\n\
    \        do_test(1_000);\n        do_test(10_000);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: false
  path: crates/fps/ntt/src/lib.rs
  requiredBy:
  - crates/fps/fps_utils/src/lib.rs
  timestamp: '2024-07-20 16:29:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/convolution_ntt/src/main.rs
  - verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - verify/yosupo/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/fps/ntt/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/ntt/src/lib.rs
- /library/crates/fps/ntt/src/lib.rs.html
title: crates/fps/ntt/src/lib.rs
---
