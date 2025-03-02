---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/flow/mincost_bflow/src/lib.rs
    title: crates/flow/mincost_bflow/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use mincost_bflow::MinCostBFlow;\nuse proconio::{fastout, input};\n\n// capacity\u304C\
    \u8CA0\u306E\u5834\u5408\u306B\u307E\u3060\u5BFE\u5FDC\u3067\u304D\u3066\u306A\
    \u3044(TODO)\n// \u5BB9\u91CF\u30B9\u30B1\u30FC\u30EA\u30F3\u30B0\u3082\u307E\u3060\
    \u3067\u304D\u3066\u306A\u3044(TODO)\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        m: usize,\n        b: [i128; n],\n        s_t_l_u_c:\
    \ [(usize, usize, i128, i128, i128); m],\n    }\n    let mut mcf = MinCostBFlow::new(n);\n\
    \    for (i, b) in b.into_iter().enumerate() {\n        if b < 0 {\n         \
    \   mcf.add_demand(i, -b);\n        } else if b > 0 {\n            mcf.add_supply(i,\
    \ b);\n        }\n    }\n    for (s, t, l, u, c) in s_t_l_u_c {\n        mcf.add_edge(s,\
    \ t, l, u, c);\n    }\n    if let Some(res) = mcf.mincost_bflow() {\n        println!(\"\
    {}\", res.cost);\n        for &v in &res.potential {\n            println!(\"\
    {}\", v);\n        }\n        for &f in &res.flow {\n            println!(\"{}\"\
    , f);\n        }\n    } else {\n        println!(\"infeasible\")\n    }\n}\n"
  dependsOn:
  - crates/flow/mincost_bflow/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/min_cost_b_flow/src/main.rs
  requiredBy: []
  timestamp: '2025-03-02 17:56:59+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/min_cost_b_flow/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/min_cost_b_flow/src/main.rs
- /library/verify/yosupo/min_cost_b_flow/src/main.rs.html
title: verify/yosupo/min_cost_b_flow/src/main.rs
---
