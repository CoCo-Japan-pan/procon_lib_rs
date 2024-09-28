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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5B8C\u5099\u8F9E\u66F8  \n//! 1.5N bit \u7528\u3044\u3066\u3044\u308B\
    \u306E\u3067\u3001succint\u3067\u306F\u306A\u3044 compact\u3067\u306F\u3042\u308B\
    \  \n//! u64\u306E\u30D6\u30ED\u30C3\u30AF\u306E\u307F\u3092\u4F7F\u3044\u3001\
    \u5C0F\u30D6\u30ED\u30C3\u30AF\u306F\u4F7F\u308F\u306A\u3044  \n//! select\u306E\
    \u9AD8\u901F\u5316\u306E\u305F\u3081\u306B\u3001x86_64\u306E\u547D\u4EE4\u3092\
    \u4F7F\u3063\u3066\u3044\u308B  \n//! \u3057\u304B\u3057select\u7528\u306E\u7D22\
    \u5F15\u3092\u6301\u305F\u305B\u3066\u306A\u3044\u306E\u3067\u3001select\u306F\
    O(logN)  \n//! \u96D1\u306A\u30D9\u30F3\u30C1\u30DE\u30FC\u30AF\u306B\u3088\u308B\
    \u3068\u3001select1\u306Frank1\u306E4~5\u500D\u7A0B\u5EA6\u306E\u6642\u9593\u304C\
    \u304B\u304B\u308A\u305D\u3046\n\n#![cfg(target_arch = \"x86_64\")]\nuse std::arch::x86_64::_pdep_u64;\n\
    \n/// \u30AD\u30E3\u30C3\u30B7\u30E5\u52B9\u7387\u306E\u305F\u3081\u3001\u30D6\
    \u30ED\u30C3\u30AF\u3068\u305D\u306E\u524D\u306E\u30D6\u30ED\u30C3\u30AF\u307E\
    \u3067\u306E1\u306E\u6570\u3092\u307E\u3068\u3081\u3066\u6301\u3064\n#[derive(Debug,\
    \ Clone, Copy)]\nstruct Block {\n    block: u64,\n    cum_sum_popcnt: u32,\n}\n\
    \n#[derive(Debug, Clone)]\npub struct BitVec {\n    len: usize,\n    blocks: Vec<Block>,\n\
    \    all_popcnt: u32,\n}\n\nimpl BitVec {\n    pub fn new(bitvec: &[bool]) ->\
    \ Self {\n        let len = bitvec.len();\n        let b_len = (len + 63) / 64;\n\
    \        let mut blocks = vec![\n            Block {\n                block: 0,\n\
    \                cum_sum_popcnt: 0\n            };\n            b_len\n      \
    \  ];\n        for i in 0..len {\n            if bitvec[i] {\n               \
    \ blocks[i >> 6].block |= 1 << (i & 63);\n            }\n        }\n        let\
    \ mut popcnt = 0;\n        for b in blocks.iter_mut() {\n            b.cum_sum_popcnt\
    \ = popcnt;\n            popcnt += b.block.count_ones();\n        }\n        Self\
    \ {\n            len,\n            blocks,\n            all_popcnt: popcnt,\n\
    \        }\n    }\n\n    /// [0..i)\u306E1\u306E\u6570 O(1)\n    pub fn rank1(&self,\
    \ i: usize) -> usize {\n        debug_assert!(i <= self.len);\n        let Block\
    \ {\n            block,\n            cum_sum_popcnt,\n        } = self.blocks[i\
    \ >> 6];\n        let mask = (1 << (i & 63)) - 1;\n        let popcnt = (block\
    \ & mask).count_ones();\n        (cum_sum_popcnt + popcnt) as usize\n    }\n\n\
    \    /// [0..i)\u306E0\u306E\u6570 O(1)\n    pub fn rank0(&self, i: usize) ->\
    \ usize {\n        i - self.rank1(i)\n    }\n\n    /// 0-based\u3067i\u756A\u76EE\
    \u306E1\u306E\u4F4D\u7F6E O(logN)\n    pub fn select1(&self, i: usize) -> Option<usize>\
    \ {\n        if i >= self.all_popcnt as usize {\n            return None;\n  \
    \      }\n        let i = i as u32;\n        let mut ok = 0;\n        let mut\
    \ ng = self.blocks.len();\n        while ng - ok > 1 {\n            let mid =\
    \ (ok + ng) >> 1;\n            if self.blocks[mid].cum_sum_popcnt <= i {\n   \
    \             ok = mid;\n            } else {\n                ng = mid;\n   \
    \         }\n        }\n        let rem = i - self.blocks[ok].cum_sum_popcnt;\n\
    \        // ok\u306E\u30D6\u30ED\u30C3\u30AF\u306E\u4E2D\u3067\u306Erem\u756A\u76EE\
    \u306E1\u306E\u4F4D\u7F6E\n        let offset = select1_u64(self.blocks[ok].block,\
    \ rem as usize);\n        Some((ok << 6) + offset as usize)\n    }\n\n    ///\
    \ 0-based\u3067i\u756A\u76EE\u306E0\u306E\u4F4D\u7F6E O(logN)\n    pub fn select0(&self,\
    \ i: usize) -> Option<usize> {\n        let all_0 = self.len - self.all_popcnt\
    \ as usize;\n        if i >= all_0 {\n            return None;\n        }\n  \
    \      let mut ok = 0;\n        let mut ng = self.blocks.len();\n        while\
    \ ng - ok > 1 {\n            let mid = (ok + ng) >> 1;\n            if ((mid <<\
    \ 6) - self.blocks[mid].cum_sum_popcnt as usize) <= i {\n                ok =\
    \ mid;\n            } else {\n                ng = mid;\n            }\n     \
    \   }\n        let rem = i - ((ok << 6) - self.blocks[ok].cum_sum_popcnt as usize);\n\
    \        // ok\u306E\u30D6\u30ED\u30C3\u30AF\u306E\u4E2D\u3067\u306Erem\u756A\u76EE\
    \u306E0\u306E\u4F4D\u7F6E\n        let offset = select1_u64(!self.blocks[ok].block,\
    \ rem);\n        Some((ok << 6) + offset as usize)\n    }\n}\n\nfn select1_u64(x:\
    \ u64, index: usize) -> u32 {\n    let z = 1 << index;\n    let y = unsafe { _pdep_u64(z,\
    \ x) };\n    y.trailing_zeros()\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n\
    \    use rand::prelude::*;\n\n    #[test]\n    fn test_rank() {\n        let mut\
    \ rng = thread_rng();\n        const SIZE: usize = 250000;\n        let bool_vec\
    \ = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();\n        let bit_vec\
    \ = BitVec::new(&bool_vec);\n        let mut ans1 = vec![0; SIZE + 1];\n     \
    \   let mut ans0 = vec![0; SIZE + 1];\n        for i in 0..SIZE {\n          \
    \  ans1[i + 1] = ans1[i] + bool_vec[i] as usize;\n            ans0[i + 1] = ans0[i]\
    \ + !bool_vec[i] as usize;\n        }\n        for i in 0..SIZE {\n          \
    \  assert_eq!(bit_vec.rank1(i), ans1[i]);\n            assert_eq!(bit_vec.rank0(i),\
    \ ans0[i]);\n        }\n    }\n\n    #[test]\n    fn test_select() {\n       \
    \ let mut rng = thread_rng();\n        const SIZE: usize = 250000;\n        let\
    \ bool_vec = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();\n     \
    \   let bit_vec = BitVec::new(&bool_vec);\n        let mut one_indices = Vec::with_capacity(bit_vec.all_popcnt\
    \ as usize);\n        let mut zero_indices = Vec::with_capacity(SIZE - bit_vec.all_popcnt\
    \ as usize);\n        for i in 0..SIZE {\n            if bool_vec[i] {\n     \
    \           one_indices.push(i);\n            } else {\n                zero_indices.push(i);\n\
    \            }\n        }\n        for i in 0..SIZE {\n            assert_eq!(bit_vec.select1(i),\
    \ one_indices.get(i).copied());\n            assert_eq!(bit_vec.select0(i), zero_indices.get(i).copied());\n\
    \        }\n    }\n\n    #[test]\n    fn bench() {\n        fn stop_watch() ->\
    \ f64 {\n            use std::time::{SystemTime, UNIX_EPOCH};\n            static\
    \ mut START: f64 = 0.0;\n            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n\
    \            let current = time.as_secs() as f64 + time.subsec_nanos() as f64\
    \ * 1e-9;\n            unsafe {\n                let ret = current - START;\n\
    \                START = current;\n                ret\n            }\n      \
    \  }\n\n        let mut rng = thread_rng();\n        const SIZE: usize = 250000;\n\
    \        let bool_vec = (0..SIZE).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();\n\
    \        let bit_vec = BitVec::new(&bool_vec);\n        let rand_nums = {\n  \
    \          let mut rand_nums = (0..SIZE).collect::<Vec<_>>();\n            rand_nums.shuffle(&mut\
    \ rng);\n            rand_nums\n        };\n        stop_watch();\n        for\
    \ &i in &rand_nums {\n            assert!(bit_vec.rank1(i) <= i);\n        }\n\
    \        println!(\"rank1: {:.6}\", stop_watch());\n        for &i in &rand_nums\
    \ {\n            if let Some(pos) = bit_vec.select1(i) {\n                assert_eq!(bit_vec.rank1(pos),\
    \ i);\n            }\n        }\n        println!(\"select1, rank1: {:.6}\", stop_watch());\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/succint/bitvec/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-29 00:31:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/succint/bitvec/src/lib.rs
layout: document
redirect_from:
- /library/crates/succint/bitvec/src/lib.rs
- /library/crates/succint/bitvec/src/lib.rs.html
title: crates/succint/bitvec/src/lib.rs
---
