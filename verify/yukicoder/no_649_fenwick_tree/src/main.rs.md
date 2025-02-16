---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/649
    links:
    - https://yukicoder.me/problems/no/649
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/649\n\n\
    use fenwick_tree::FenwickTree;\nuse proconio::{fastout, input};\n\n#[derive(Debug,\
    \ Clone, Copy)]\nenum Query {\n    Add(u64),\n    Ask,\n}\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        q: usize,\n        k: i32,\n    }\n    let mut queries\
    \ = vec![];\n    for _ in 0..q {\n        input! {\n            t: usize,\n  \
    \      }\n        match t {\n            1 => {\n                input! {\n  \
    \                  a: u64,\n                }\n                queries.push(Query::Add(a));\n\
    \            }\n            2 => {\n                queries.push(Query::Ask);\n\
    \            }\n            _ => unreachable!(),\n        }\n    }\n    let queries\
    \ = queries;\n    let mut num_list = queries\n        .iter()\n        .filter_map(|q|\
    \ {\n            if let Query::Add(a) = q {\n                Some(*a)\n      \
    \      } else {\n                None\n            }\n        })\n        .collect::<Vec<_>>();\n\
    \    num_list.sort();\n    num_list.dedup();\n    let num_list = num_list;\n \
    \   let mut fenwick = FenwickTree::new(num_list.len(), 0_i32);\n    let mut sum\
    \ = 0;\n    for query in queries {\n        match query {\n            Query::Add(a)\
    \ => {\n                let idx = num_list.binary_search(&a).unwrap();\n     \
    \           fenwick.add(idx, 1);\n                sum += 1;\n            }\n \
    \           Query::Ask => {\n                if sum < k {\n                  \
    \  println!(\"-1\");\n                } else {\n                    let id = fenwick.lower_bound(k);\n\
    \                    println!(\"{}\", num_list[id]);\n                    fenwick.add(id,\
    \ -1);\n                    sum -= 1;\n                }\n            }\n    \
    \    }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_649_fenwick_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-01-19 12:17:00+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_649_fenwick_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_649_fenwick_tree/src/main.rs
- /verify/verify/yukicoder/no_649_fenwick_tree/src/main.rs.html
title: verify/yukicoder/no_649_fenwick_tree/src/main.rs
---
