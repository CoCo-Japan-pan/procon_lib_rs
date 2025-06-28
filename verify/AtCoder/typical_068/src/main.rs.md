---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/potentialized_unionfind/src/lib.rs
    title: crates/data_structure/potentialized_unionfind/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/typical90/tasks/typical90_bp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://atcoder.jp/contests/typical90/tasks/typical90_bp\n\nuse algebra::{Group,\
    \ Monoid};\nuse potentialized_unionfind::PotentializedUnionFind;\nuse proconio::{fastout,\
    \ input, marker::Usize1};\n\n/// y = +-x + bias \u3092\u8868\u3059\u7FA4\n#[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\nstruct AffineGroup {\n    keisuu: bool,\n    bias:\
    \ i64,\n}\n\nimpl AffineGroup {\n    fn apply(&self, x: i64) -> i64 {\n      \
    \  if self.keisuu {\n            x + self.bias\n        } else {\n           \
    \ self.bias - x\n        }\n    }\n}\n\nimpl Monoid for AffineGroup {\n    type\
    \ Target = Self;\n    fn binary_operation(a: &Self::Target, b: &Self::Target)\
    \ -> Self::Target {\n        AffineGroup {\n            keisuu: !(a.keisuu ^ b.keisuu),\n\
    \            bias: if b.keisuu {\n                b.bias + a.bias\n          \
    \  } else {\n                b.bias - a.bias\n            },\n        }\n    }\n\
    \    fn id_element() -> Self::Target {\n        Self {\n            keisuu: true,\n\
    \            bias: 0,\n        }\n    }\n}\n\nimpl Group for AffineGroup {\n \
    \   fn inverse(a: &Self::Target) -> Self::Target {\n        Self {\n         \
    \   keisuu: a.keisuu,\n            bias: if a.keisuu { -a.bias } else { a.bias\
    \ },\n        }\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n:\
    \ usize,\n        q: usize,\n    }\n    let mut uf = PotentializedUnionFind::<AffineGroup>::new(n);\n\
    \    for _ in 0..q {\n        input! {\n            t: u8,\n            x: Usize1,\n\
    \            y: Usize1,\n            v: i64,\n        }\n        if t == 0 {\n\
    \            // y = -x + v\n            uf.relate(\n                x,\n     \
    \           y,\n                AffineGroup {\n                    keisuu: false,\n\
    \                    bias: v,\n                },\n            )\n           \
    \ .unwrap();\n        } else {\n            let diff = uf.diff(x, y);\n      \
    \      if let Some(diff) = diff {\n                println!(\"{}\", diff.apply(v));\n\
    \            } else {\n                println!(\"Ambiguous\");\n            }\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/potentialized_unionfind/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/typical_068/src/main.rs
  requiredBy: []
  timestamp: '2025-04-29 15:50:13+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/typical_068/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/typical_068/src/main.rs
- /library/verify/AtCoder/typical_068/src/main.rs.html
title: verify/AtCoder/typical_068/src/main.rs
---
