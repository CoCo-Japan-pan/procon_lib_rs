---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/modint_traits/src/lib.rs
    title: crates/internals/modint_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
    title: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://math314.hateblo.jp/entry/2015/05/07/014908)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [\u4EFB\u610Fmod](https://math314.hateblo.jp/entry/2015/05/07/014908)\
    \  \n\nuse modint_traits::ModInt;\nuse ntt::convolution;\nuse static_modint::StaticModInt;\n\
    \n/// \u53D6\u308A\u3046\u308B\u6700\u5927\u5024\u3092\u8D85\u3048\u308Bmod\u3092\
    \u8868\u73FE\u3067\u304D\u308B\u3088\u3046\u306Amod\u306E\u7D44\u3092\u9078\u3093\
    \u3067\u7573\u307F\u8FBC\u307F\u3001Garner\u3067\u5FA9\u5143\npub fn convolution_aribtrary_u32_mod<M:\
    \ ModInt>(a: &[M], b: &[M]) -> Vec<M> {\n    // \u3069\u308C\u3082\u539F\u5B50\
    \u68393\u3067\u30012^24\u4E57\u6839\u304C\u3042\u308B\n    const MOD1: u32 = 167772161;\n\
    \    const MOD2: u32 = 469762049;\n    const MOD3: u32 = 1224736769;\n    let\
    \ x = convolution::<MOD1, 3>(\n        a.iter()\n            .map(|x| StaticModInt::<MOD1>::new(x.value()))\n\
    \            .collect::<Vec<_>>()\n            .as_slice(),\n        b.iter()\n\
    \            .map(|x| StaticModInt::<MOD1>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n    );\n    let y = convolution::<MOD2, 3>(\n     \
    \   a.iter()\n            .map(|x| StaticModInt::<MOD2>::new(x.value()))\n   \
    \         .collect::<Vec<_>>()\n            .as_slice(),\n        b.iter()\n \
    \           .map(|x| StaticModInt::<MOD2>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n    );\n    let z = convolution::<MOD3, 3>(\n     \
    \   a.iter()\n            .map(|x| StaticModInt::<MOD3>::new(x.value()))\n   \
    \         .collect::<Vec<_>>()\n            .as_slice(),\n        b.iter()\n \
    \           .map(|x| StaticModInt::<MOD3>::new(x.value()))\n            .collect::<Vec<_>>()\n\
    \            .as_slice(),\n    );\n\n    let m1_inv_m2 = StaticModInt::<MOD2>::new(MOD1).inv();\n\
    \    let m12_inv_m3 = StaticModInt::<MOD3>::new(MOD1 as u64 * MOD2 as u64).inv();\n\
    \    let m12_mod = M::new(MOD1 as u64 * MOD2 as u64);\n    let mut ret = vec![M::raw(0);\
    \ x.len()];\n    for (i, r) in ret.iter_mut().enumerate() {\n        let v1 =\
    \ ((StaticModInt::<MOD2>::new(y[i].value())\n            - StaticModInt::<MOD2>::new(x[i].value()))\n\
    \            * m1_inv_m2)\n            .value();\n        let v2 = ((StaticModInt::<MOD3>::new(z[i].value())\n\
    \            - StaticModInt::<MOD3>::new(x[i].value())\n            - StaticModInt::<MOD3>::new(MOD1)\
    \ * StaticModInt::<MOD3>::new(v1))\n            * m12_inv_m3)\n            .value();\n\
    \        let constants = M::new(x[i].value()) + M::new(MOD1) * M::new(v1) + m12_mod\
    \ * M::new(v2);\n        *r = constants;\n    }\n    ret\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/internals/modint_traits/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: false
  path: crates/fps/ntt_arbitrary_mod/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 12:40:51+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
documentation_of: crates/fps/ntt_arbitrary_mod/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/ntt_arbitrary_mod/src/lib.rs
- /library/crates/fps/ntt_arbitrary_mod/src/lib.rs.html
title: crates/fps/ntt_arbitrary_mod/src/lib.rs
---
