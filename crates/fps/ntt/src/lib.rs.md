---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt_arbitrary_mod/src/lib.rs
    title: crates/fps/ntt_arbitrary_mod/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_ntt/src/main.rs
    title: verify/yosupo/convolution_ntt/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
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
    \  \n//! \u7D50\u5C40ac-library-rs\u3068\u307B\u3068\u3093\u3069\u540C\u3058\u5B9F\
    \u88C5\u3067\u3059(\u30B8\u30A7\u30CD\u30EA\u30C3\u30AF\u306B\u7D10\u3065\u304F\
    static\u306E\u5B9F\u73FE\u304C\u3080\u305A\u304F\u3066\u30AD\u30E3\u30C3\u30B7\
    \u30E5\u306F\u6BCE\u56DE\u3068\u308B\u3053\u3068\u306B...)\n\nuse static_modint::{ModInt998244353,\
    \ StaticModInt};\n\nfn prepare<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n\
    ) -> ([StaticModInt<NTT_MOD>; 30], [StaticModInt<NTT_MOD>; 30]) {\n    let g =\
    \ StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT);\n    let mut es = [StaticModInt::<NTT_MOD>::raw(0);\
    \ 30];\n    let mut ies = [StaticModInt::<NTT_MOD>::raw(0); 30];\n    let cnt2\
    \ = (NTT_MOD - 1).trailing_zeros() as usize;\n    let mut e = g.pow(((NTT_MOD\
    \ - 1) >> cnt2).into());\n    let mut ie = e.inv();\n    for i in (2..=cnt2).rev()\
    \ {\n        es[i - 2] = e;\n        ies[i - 2] = ie;\n        e *= e;\n     \
    \   ie *= ie;\n    }\n    for i in 1..30 {\n        es[i] *= es[i - 1];\n    \
    \    ies[i] *= ies[i - 1];\n    }\n    (es, ies)\n}\n\nfn ntt<const NTT_MOD: u32,\
    \ const PRIMITIVE_ROOT: u32>(\n    data: &mut [StaticModInt<NTT_MOD>],\n    sum_e:\
    \ &[StaticModInt<NTT_MOD>; 30],\n) {\n    let size = data.len();\n    assert!(size.is_power_of_two());\n\
    \    let height = size.next_power_of_two().trailing_zeros();\n    for ph in 1..=height\
    \ {\n        let w = 1 << (ph - 1);\n        let p = 1 << (height - ph);\n   \
    \     let mut now = StaticModInt::<NTT_MOD>::raw(1);\n        for s in 0..w {\n\
    \            let offset = s << (height - ph + 1);\n            for i in 0..p {\n\
    \                let l = data[i + offset];\n                let r = data[i + offset\
    \ + p] * now;\n                data[i + offset] = l + r;\n                data[i\
    \ + offset + p] = l - r;\n            }\n            now *= sum_e[(!s).trailing_zeros()\
    \ as usize];\n        }\n    }\n}\n\nfn intt<const NTT_MOD: u32, const PRIMITIVE_ROOT:\
    \ u32>(\n    data: &mut [StaticModInt<NTT_MOD>],\n    sum_ie: &[StaticModInt<NTT_MOD>;\
    \ 30],\n) {\n    let size = data.len();\n    assert!(size.is_power_of_two());\n\
    \    let height = size.next_power_of_two().trailing_zeros();\n    for ph in (1..=height).rev()\
    \ {\n        let w = 1 << (ph - 1);\n        let p = 1 << (height - ph);\n   \
    \     let mut inow = StaticModInt::<NTT_MOD>::raw(1);\n        for s in 0..w {\n\
    \            let offset = s << (height - ph + 1);\n            for i in 0..p {\n\
    \                let l = data[i + offset];\n                let r = data[i + offset\
    \ + p];\n                data[i + offset] = l + r;\n                data[i + offset\
    \ + p] = (l - r) * inow;\n            }\n            inow *= sum_ie[(!s).trailing_zeros()\
    \ as usize];\n        }\n    }\n}\n\n/// NTT\u306B\u3088\u308B\u7573\u307F\u8FBC\
    \u307F\npub fn convolution<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(\n \
    \   a: &[StaticModInt<NTT_MOD>],\n    b: &[StaticModInt<NTT_MOD>],\n) -> Vec<StaticModInt<NTT_MOD>>\
    \ {\n    if a.is_empty() || b.is_empty() {\n        return vec![];\n    }\n  \
    \  let n = a.len() + b.len() - 1;\n    let size = n.next_power_of_two();\n   \
    \ let mut a = a.to_owned();\n    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));\n\
    \    let (sum_e, sum_ie) = prepare::<NTT_MOD, PRIMITIVE_ROOT>();\n    ntt::<NTT_MOD,\
    \ PRIMITIVE_ROOT>(&mut a, &sum_e);\n    let mut b = b.to_owned();\n    b.resize(size,\
    \ StaticModInt::<NTT_MOD>::raw(0));\n    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b,\
    \ &sum_e);\n    for (a, b) in a.iter_mut().zip(b) {\n        *a *= b;\n    }\n\
    \    intt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, &sum_ie);\n    a.resize(n, StaticModInt::<NTT_MOD>::raw(0));\n\
    \    let inv_size = StaticModInt::<NTT_MOD>::raw(size as u32).inv();\n    for\
    \ a in a.iter_mut() {\n        *a *= inv_size;\n    }\n    a\n}\n\n/// 998244353\
    \ = 119 * 2^23 + 1 \u3067\u539F\u59CB\u68393\u3092\u6301\u3064\npub fn convolution_998244353(a:\
    \ &[ModInt998244353], b: &[ModInt998244353]) -> Vec<ModInt998244353> {\n    convolution::<998244353,\
    \ 3>(a, b)\n}\n"
  dependsOn:
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: false
  path: crates/fps/ntt/src/lib.rs
  requiredBy:
  - crates/fps/ntt_arbitrary_mod/src/lib.rs
  timestamp: '2024-04-06 17:23:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/convolution_ntt/src/main.rs
documentation_of: crates/fps/ntt/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/ntt/src/lib.rs
- /library/crates/fps/ntt/src/lib.rs.html
title: crates/fps/ntt/src/lib.rs
---
