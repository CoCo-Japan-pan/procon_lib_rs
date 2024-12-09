---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/wavelet/bitdict/src/lib.rs
    title: crates/wavelet/bitdict/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use bitdict::BitDict;\nuse criterion::{black_box, criterion_group, criterion_main,\
    \ Criterion};\nuse rand::prelude::*;\nuse rand_pcg::Pcg32;\n\npub fn rank1(c:\
    \ &mut Criterion) {\n    let mut rng = Pcg32::seed_from_u64(0);\n    const SIZE:\
    \ usize = 100000;\n    let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();\n\
    \    let bit_vec = BitDict::from(&bool_vec[..]);\n    c.bench_function(\"rank1_all\"\
    , |b| {\n        b.iter(|| {\n            for i in 0..SIZE {\n               \
    \ black_box(bit_vec.rank_1(i));\n            }\n        });\n    });\n}\n\npub\
    \ fn rank0(c: &mut Criterion) {\n    let mut rng = Pcg32::seed_from_u64(0);\n\
    \    const SIZE: usize = 100000;\n    let bool_vec: Vec<bool> = (0..SIZE).map(|_|\
    \ rng.gen()).collect();\n    let bit_vec = BitDict::from(&bool_vec[..]);\n   \
    \ c.bench_function(\"rank0_all\", |b| {\n        b.iter(|| {\n            for\
    \ i in 0..SIZE {\n                black_box(bit_vec.rank_0(i));\n            }\n\
    \        });\n    });\n}\n\npub fn select1(c: &mut Criterion) {\n    let mut rng\
    \ = Pcg32::seed_from_u64(0);\n    const SIZE: usize = 100000;\n    let bool_vec:\
    \ Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();\n    let bit_vec = BitDict::from(&bool_vec[..]);\n\
    \    c.bench_function(\"select1_all\", |b| {\n        b.iter(|| {\n          \
    \  for i in 0..SIZE {\n                black_box(bit_vec.select_1(i));\n     \
    \       }\n        });\n    });\n}\n\npub fn select0(c: &mut Criterion) {\n  \
    \  let mut rng = Pcg32::seed_from_u64(0);\n    const SIZE: usize = 100000;\n \
    \   let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();\n    let\
    \ bit_vec = BitDict::from(&bool_vec[..]);\n    c.bench_function(\"select0_all\"\
    , |b| {\n        b.iter(|| {\n            for i in 0..SIZE {\n               \
    \ black_box(bit_vec.select_0(i));\n            }\n        });\n    });\n}\n\n\
    criterion_group! {\n    name = benches;\n    config = Criterion::default();\n\
    \    targets = rank1, rank0, select1, select0\n}\ncriterion_main!(benches);\n"
  dependsOn:
  - crates/wavelet/bitdict/src/lib.rs
  isVerificationFile: false
  path: crates/wavelet/bitdict/benches/my_benchmark.rs
  requiredBy: []
  timestamp: '2024-10-09 22:07:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/wavelet/bitdict/benches/my_benchmark.rs
layout: document
redirect_from:
- /library/crates/wavelet/bitdict/benches/my_benchmark.rs
- /library/crates/wavelet/bitdict/benches/my_benchmark.rs.html
title: crates/wavelet/bitdict/benches/my_benchmark.rs
---
