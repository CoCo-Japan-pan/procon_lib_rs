---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://creativecommons.org/licenses/by-sa/4.0/deed.en
    - https://ja.wikipedia.org/wiki/Permuted_congruential_generator
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://ja.wikipedia.org/wiki/Permuted_congruential_generator>\
    \  \n//! Under CC BY-SA 4.0 <https://creativecommons.org/licenses/by-sa/4.0/deed.en>\n\
    \n#[derive(Debug, Clone, Copy)]\npub struct Pcg32 {\n    state: u64,\n}\n\nimpl\
    \ Pcg32 {\n    const MULTIPLIER: u64 = 6364136223846793005;\n    const INCREMENT:\
    \ u64 = 1442695040888963407;\n\n    pub fn init(seed: u64) -> Self {\n       \
    \ Self {\n            state: seed.wrapping_add(Self::INCREMENT),\n        }\n\
    \    }\n\n    pub fn init_rand() -> Self {\n        use std::time::SystemTime;\n\
    \        let now = SystemTime::now()\n            .duration_since(SystemTime::UNIX_EPOCH)\n\
    \            .unwrap();\n        Self::init(now.as_secs() + now.subsec_nanos()\
    \ as u64)\n    }\n\n    pub fn next_u32(&mut self) -> u32 {\n        let old_state\
    \ = self.state;\n        self.state = old_state\n            .wrapping_mul(Self::MULTIPLIER)\n\
    \            .wrapping_add(Self::INCREMENT);\n        let xorshifted = (((old_state\
    \ >> 18) ^ old_state) >> 27) as u32;\n        let rot = (old_state >> 59) as u32;\n\
    \        xorshifted.rotate_right(rot)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/rand/pcg32/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-20 16:41:21+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/rand/pcg32/src/lib.rs
layout: document
redirect_from:
- /library/crates/rand/pcg32/src/lib.rs
- /library/crates/rand/pcg32/src/lib.rs.html
title: crates/rand/pcg32/src/lib.rs
---
