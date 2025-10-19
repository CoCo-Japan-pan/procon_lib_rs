---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u30C0\u30D6\u30EA\u30F3\u30B0  \n\n#[derive(Debug, Clone)]\npub struct\
    \ Doubling {\n    n: usize,\n    max_pow: u64,\n    table: Vec<Vec<u32>>,\n}\n\
    \nimpl Doubling {\n    /// [0, n) -> [0, n) \u306E\u5199\u50CFfunc\u3092\u53D7\
    \u3051\u53D6\u308B  \n    /// n\u306F32bit\u4EE5\u4E0B\u3067\u53CE\u307E\u308B\
    \u3068\u4EEE\u5B9A\u3057\u3066\u3044\u308B  \n    /// \u5199\u50CF\u306Fmax_pow\u4E57\
    \u307E\u3067\u306E\u307F\u8003\u3048\u308B\n    pub fn new(func: &[usize], max_pow:\
    \ u64) -> Self {\n        let n = func.len();\n        let mut table = vec![func.iter().map(|&x|\
    \ x as u32).collect::<Vec<_>>()];\n        let mut cur_pow = max_pow >> 1;\n \
    \       while cur_pow > 0 {\n            let mut next = vec![0; n];\n        \
    \    let last_table = table.last().unwrap();\n            for i in 0..n {\n  \
    \              next[i] = last_table[last_table[i] as usize];\n            }\n\
    \            table.push(next);\n            cur_pow >>= 1;\n        }\n      \
    \  Self { n, max_pow, table }\n    }\n\n    /// func^x(index)\u3092\u8FD4\u3059\
    \n    pub fn query(&self, mut index: usize, mut x: u64) -> usize {\n        assert!(index\
    \ < self.n);\n        assert!(x <= self.max_pow);\n        let mut table_index\
    \ = 0;\n        while x > 0 {\n            if x & 1 == 1 {\n                index\
    \ = self.table[table_index][index] as usize;\n            }\n            table_index\
    \ += 1;\n            x >>= 1;\n        }\n        index\n    }\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n    use rand::prelude::*;\n\n    #[test]\n    fn\
    \ test() {\n        let mut rng = thread_rng();\n        const SIZE: usize = 1000;\n\
    \        let func = (0..SIZE)\n            .map(|_| rng.gen_range(0..SIZE))\n\
    \            .collect::<Vec<_>>();\n        let max_pow = 1000;\n        let doubling\
    \ = Doubling::new(&func, max_pow);\n        for _ in 0..100 {\n            let\
    \ index = rng.gen_range(0..SIZE);\n            let x = rng.gen_range(0..max_pow);\n\
    \            let expected = (0..x).fold(index, |i, _| func[i]);\n            assert_eq!(doubling.query(index,\
    \ x as u64), expected);\n        }\n    }\n\n    #[test]\n    fn test_two_beki()\
    \ {\n        let mut rng = thread_rng();\n        const SIZE: usize = 1024;\n\
    \        let func = (0..SIZE)\n            .map(|_| rng.gen_range(0..SIZE))\n\
    \            .collect::<Vec<_>>();\n        let max_pow = 1024;\n        let doubling\
    \ = Doubling::new(&func, max_pow);\n        for bit in 0..=10 {\n            let\
    \ index = rng.gen_range(0..SIZE);\n            let x = 1 << bit;\n           \
    \ let expected = (0..x).fold(index, |i, _| func[i]);\n            assert_eq!(doubling.query(index,\
    \ x as u64), expected);\n            let x = x - 1;\n            let expected\
    \ = (0..x).fold(index, |i, _| func[i]);\n            assert_eq!(doubling.query(index,\
    \ x as u64), expected);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/doubling/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 20:59:15+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/doubling/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/doubling/src/lib.rs
- /library/crates/misc/doubling/src/lib.rs.html
title: crates/misc/doubling/src/lib.rs
---
