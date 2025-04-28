---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/potentialized_unionfind/src/lib.rs
    title: crates/data_structure/potentialized_unionfind/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
    links:
    - https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group\n\
    \nuse algebra::{Group, Monoid};\nuse potentialized_unionfind::PotentializedUnionFind;\n\
    use proconio::{fastout, input};\nuse static_modint::ModInt998244353 as MInt;\n\
    \n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\nstruct M2([MInt; 4]);\n\nimpl\
    \ Monoid for M2 {\n    type Target = Self;\n    fn id_element() -> Self::Target\
    \ {\n        M2([MInt::new(1), MInt::new(0), MInt::new(0), MInt::new(1)])\n  \
    \  }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        let mut res = [MInt::new(0); 4];\n        for i in 0..2 {\n     \
    \       for j in 0..2 {\n                for k in 0..2 {\n                   \
    \ res[i * 2 + j] += a.0[i * 2 + k] * b.0[k * 2 + j];\n                }\n    \
    \        }\n        }\n        M2(res)\n    }\n}\n\nimpl Group for M2 {\n    fn\
    \ inverse(a: &Self::Target) -> Self::Target {\n        M2([a.0[3], -a.0[1], -a.0[2],\
    \ a.0[0]])\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n    }\n    let mut puf = PotentializedUnionFind::<M2>::new(n);\n\
    \    for _ in 0..q {\n        input! {\n            t: u8,\n        }\n      \
    \  if t == 0 {\n            input! {\n                u: usize,\n            \
    \    v: usize,\n                x: [MInt; 4],\n            }\n            let\
    \ diff = {\n                let mut ret = [MInt::new(0); 4];\n               \
    \ for i in 0..4 {\n                    ret[i] = x[i];\n                }\n   \
    \             M2(ret)\n            };\n            if puf.relate(v, u, diff).is_err()\
    \ {\n                println!(\"0\");\n            } else {\n                println!(\"\
    1\");\n            }\n        } else {\n            input! {\n               \
    \ u: usize,\n                v: usize,\n            }\n            let ans = puf.diff(v,\
    \ u);\n            if let Some(ans) = ans {\n                println!(\"{} {}\
    \ {} {}\", ans.0[0], ans.0[1], ans.0[2], ans.0[3]);\n            } else {\n  \
    \              println!(\"-1\");\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/potentialized_unionfind/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/unionfind_with_potential_non_commutative_group/src/main.rs
  requiredBy: []
  timestamp: '2025-03-13 00:09:19+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/unionfind_with_potential_non_commutative_group/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/unionfind_with_potential_non_commutative_group/src/main.rs
- /verify/verify/yosupo/unionfind_with_potential_non_commutative_group/src/main.rs.html
title: verify/yosupo/unionfind_with_potential_non_commutative_group/src/main.rs
---
