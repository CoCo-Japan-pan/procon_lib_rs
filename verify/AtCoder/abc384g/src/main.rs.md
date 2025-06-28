---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/mo/src/lib.rs
    title: crates/misc/mo/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc384/tasks/abc384_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc384/tasks/abc384_g\n\n#![allow(non_snake_case)]\n\
    use fenwick_tree::FenwickTree;\nuse mo::calc_mo_friendly_order;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        N: usize,\n      \
    \  A: [i64; N],\n        B: [i64; N],\n        K: usize,\n        X_Y: [(usize,\
    \ usize); K],\n    }\n    let order = calc_mo_friendly_order(&X_Y, N, N);\n  \
    \  let mut answer = vec![0; K];\n    let mut cur_answer = 0;\n    let mut a_data\
    \ = MyData::new(A);\n    let mut b_data = MyData::new(B);\n    let a_to_b = a_data\n\
    \        .list\n        .iter()\n        .map(|&x| b_data.sorted.partition_point(|&b|\
    \ b < x))\n        .collect::<Vec<_>>();\n    let b_to_a = b_data\n        .list\n\
    \        .iter()\n        .map(|&x| a_data.sorted.partition_point(|&a| a < x))\n\
    \        .collect::<Vec<_>>();\n    let (mut x, mut y) = (0, 0);\n    let diff\
    \ = |add_num: i64, idx: usize, data: &MyData| -> i64 {\n        let mut ret =\
    \ 0;\n        let (sum, cnt) = data.sum_cnt_lt(idx);\n        ret += add_num *\
    \ cnt - sum;\n        let (sum, cnt) = data.sum_cnt_ge(idx);\n        ret += sum\
    \ - add_num * cnt;\n        ret\n    };\n    for id in order {\n        let (nx,\
    \ ny) = X_Y[id];\n        while x < nx {\n            a_data.add(x);\n       \
    \     let num = a_data.list[x];\n            let idx = a_to_b[x];\n          \
    \  cur_answer += diff(num, idx, &b_data);\n            x += 1;\n        }\n  \
    \      while x > nx {\n            x -= 1;\n            a_data.remove(x);\n  \
    \          let num = a_data.list[x];\n            let idx = a_to_b[x];\n     \
    \       cur_answer -= diff(num, idx, &b_data);\n        }\n        while y < ny\
    \ {\n            b_data.add(y);\n            let num = b_data.list[y];\n     \
    \       let idx = b_to_a[y];\n            cur_answer += diff(num, idx, &a_data);\n\
    \            y += 1;\n        }\n        while y > ny {\n            y -= 1;\n\
    \            b_data.remove(y);\n            let num = b_data.list[y];\n      \
    \      let idx = b_to_a[y];\n            cur_answer -= diff(num, idx, &a_data);\n\
    \        }\n        answer[id] = cur_answer;\n    }\n    for a in answer {\n \
    \       println!(\"{}\", a);\n    }\n}\n\n/// \u7279\u5B9A\u306E\u5024\u4EE5\u4E0B\
    \u306E\u5024\u9054\u306E\u548C\u3068\u500B\u6570\u3068\u3001\u307E\u305F\u5168\
    \u4F53\u306E\u548C\u3068\u500B\u6570\u304C\u6C42\u307E\u308B\u69CB\u9020\u4F53\
    \nstruct MyData {\n    list: Vec<i64>,\n    sorted: Vec<i64>,\n    id_to_sorted_id:\
    \ Vec<usize>,\n    ft_cnt: FenwickTree<i64>,\n    ft_sum: FenwickTree<i64>,\n\
    }\n\nimpl MyData {\n    fn new(list: Vec<i64>) -> Self {\n        let mut sorted\
    \ = list.clone();\n        sorted.sort_unstable();\n        sorted.dedup();\n\
    \        let id_to_sorted_id = list\n            .iter()\n            .map(|&x|\
    \ sorted.binary_search(&x).unwrap())\n            .collect();\n        let n =\
    \ sorted.len();\n        let ft_cnt = FenwickTree::new(n, 0_i64);\n        let\
    \ ft_sum = FenwickTree::new(n, 0_i64);\n        Self {\n            list,\n  \
    \          sorted,\n            id_to_sorted_id,\n            ft_cnt,\n      \
    \      ft_sum,\n        }\n    }\n\n    /// list[idx]\u3092\u8FFD\u52A0\u3059\u308B\
    \n    fn add(&mut self, idx: usize) {\n        let num = self.list[idx];\n   \
    \     let idx = self.id_to_sorted_id[idx];\n        self.ft_cnt.add(idx, 1);\n\
    \        self.ft_sum.add(idx, num);\n    }\n\n    /// list[idx]\u3092\u524A\u9664\
    \u3059\u308B\n    fn remove(&mut self, idx: usize) {\n        let num = self.list[idx];\n\
    \        let idx = self.id_to_sorted_id[idx];\n        self.ft_cnt.add(idx, -1);\n\
    \        self.ft_sum.add(idx, -num);\n    }\n\n    /// ...idx\u306E\u5024\u306E\
    \u548C\u3068\u500B\u6570\u3092\u8FD4\u3059\n    fn sum_cnt_lt(&self, idx: usize)\
    \ -> (i64, i64) {\n        let sum = self.ft_sum.sum(..idx);\n        let cnt\
    \ = self.ft_cnt.sum(..idx);\n        (sum, cnt)\n    }\n\n    /// idx..\u306E\u5024\
    \u306E\u548C\u3068\u500B\u6570\u3092\u8FD4\u3059\n    fn sum_cnt_ge(&self, idx:\
    \ usize) -> (i64, i64) {\n        let (sum, cnt) = self.sum_cnt_lt(idx);\n   \
    \     let all_sum = self.ft_sum.sum(..);\n        let all_cnt = self.ft_cnt.sum(..);\n\
    \        (all_sum - sum, all_cnt - cnt)\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/misc/mo/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc384g/src/main.rs
  requiredBy: []
  timestamp: '2025-01-19 12:17:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc384g/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc384g/src/main.rs
- /library/verify/AtCoder/abc384g/src/main.rs.html
title: verify/AtCoder/abc384g/src/main.rs
---
