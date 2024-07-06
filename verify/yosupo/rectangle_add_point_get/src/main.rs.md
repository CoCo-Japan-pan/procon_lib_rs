---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/data_structure/rect_add_point_get/src/lib.rs
    title: crates/data_structure/rect_add_point_get/src/lib.rs
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
  code: "use algebra::{Action, Commutative};\nuse proconio::{fastout, input};\nuse\
    \ rect_add_point_get::RectActPointGet;\n\n#[derive(Clone, Copy, Debug)]\nenum\
    \ Query {\n    Add((u32, u32, u32, u32, u64)),\n    Get(u32, u32),\n}\n\n#[derive(Clone,\
    \ Copy, Debug)]\nstruct AddMap(u64);\nimpl Action for AddMap {\n    type Target\
    \ = u64;\n    fn id_map() -> Self {\n        Self(0)\n    }\n    fn composition(&mut\
    \ self, rhs: &Self) {\n        self.0 += rhs.0;\n    }\n    fn mapping(&self,\
    \ target: &mut Self::Target) {\n        *target += self.0;\n    }\n}\nimpl Commutative\
    \ for AddMap {}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        l_d_r_u_w: [(u32, u32, u32, u32, u64); n],\n    }\n\
    \    let querys = {\n        let mut querys = Vec::with_capacity(q);\n       \
    \ for _ in 0..q {\n            input! {\n                t: u32,\n           \
    \ }\n            match t {\n                0 => {\n                    input!\
    \ {\n                        l_d_r_u_w: (u32, u32, u32, u32, u64),\n         \
    \           }\n                    querys.push(Query::Add(l_d_r_u_w));\n     \
    \           }\n                1 => {\n                    input! {\n        \
    \                x: u32,\n                        y: u32,\n                  \
    \  }\n                    querys.push(Query::Get(x, y));\n                }\n\
    \                _ => unreachable!(),\n            }\n        }\n        querys\n\
    \    };\n    let get_points: Vec<(u32, u32)> = querys\n        .iter()\n     \
    \   .filter_map(|q| match q {\n            Query::Get(x, y) => Some((*x, *y)),\n\
    \            _ => None,\n        })\n        .collect();\n    let mut kdtree =\
    \ RectActPointGet::<AddMap, _>::new(get_points);\n    for (l, d, r, u, w) in l_d_r_u_w\
    \ {\n        kdtree.add_range(&(l..r), &(d..u), &AddMap(w));\n    }\n    // eprintln!(\"\
    {:?}\", kdtree);\n    for q in querys {\n        match q {\n            Query::Add((l,\
    \ d, r, u, w)) => {\n                kdtree.add_range(&(l..r), &(d..u), &AddMap(w));\n\
    \            }\n            Query::Get(x, y) => {\n                let ans = kdtree.get_composition(x,\
    \ y);\n                println!(\"{}\", ans.0);\n            }\n        }\n  \
    \  }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/rect_add_point_get/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/rectangle_add_point_get/src/main.rs
  requiredBy: []
  timestamp: '2024-07-06 23:41:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/rectangle_add_point_get/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/rectangle_add_point_get/src/main.rs
- /library/verify/yosupo/rectangle_add_point_get/src/main.rs.html
title: verify/yosupo/rectangle_add_point_get/src/main.rs
---
