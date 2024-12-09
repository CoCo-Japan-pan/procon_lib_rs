---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/sum_of_floor_of_linear/src/main.rs
    title: verify/yosupo/sum_of_floor_of_linear/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// Calculates the sum of `floor((a * i + b) / m)` for `0 <= i < n`.\n///\
    \ # Example\n///\n/// ```\n/// use floor_ceil_sum::floor_sum;\n///\n/// assert_eq!(floor_sum(6,\
    \ 5, 4, 3), 13);\n/// ```\npub fn floor_sum(mut n: i64, mut m: i64, mut a: i64,\
    \ mut b: i64) -> i64 {\n    assert!(0 <= n);\n    assert!(1 <= m);\n    let mut\
    \ ans = 0_i64;\n    if a < 0 {\n        let a_div = a.div_euclid(m);\n       \
    \ ans += a_div * n * (n - 1) / 2;\n        a -= a_div * m;\n    }\n    if b <\
    \ 0 {\n        let b_div = b.div_euclid(m);\n        ans += b_div * n;\n     \
    \   b -= b_div * m;\n    }\n    loop {\n        if a >= m {\n            ans +=\
    \ n * (n - 1) / 2 * (a / m);\n            a %= m;\n        }\n        if b >=\
    \ m {\n            ans += n * (b / m);\n            b %= m;\n        }\n     \
    \   let y_max = a * n + b;\n        if y_max < m {\n            break;\n     \
    \   }\n        n = y_max / m;\n        b = y_max % m;\n        std::mem::swap(&mut\
    \ m, &mut a);\n    }\n    ans\n}\n\n/// Calculates the sum of `ceil((a * i + b)\
    \ / m)` for `0 <= i < n`.\npub fn ceil_sum(n: i64, m: i64, a: i64, b: i64) ->\
    \ i64 {\n    // ceil(x) = -floor(-x)\n    -floor_sum(n, m, -a, -b)\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n    use rand::prelude::*;\n    #[test]\n    fn\
    \ test_floor_sum() {\n        let mut rng = thread_rng();\n        for _ in 0..100\
    \ {\n            let n = rng.gen_range(0..10000);\n            let m = rng.gen_range(1..=1000_000_000);\n\
    \            let a = rng.gen_range(-1000_000_000..=1000_000_000);\n          \
    \  let b = rng.gen_range(-1000_000_000..=1000_000_000);\n            let mut ans\
    \ = 0;\n            for i in 0..n {\n                ans += (a * i as i64 + b).div_euclid(m);\n\
    \            }\n            assert_eq!(floor_sum(n, m, a, b), ans);\n        }\n\
    \    }\n    #[test]\n    fn test_ceil_sum() {\n        let mut rng = thread_rng();\n\
    \        for _ in 0..100 {\n            let n = rng.gen_range(0..10000);\n   \
    \         let m = rng.gen_range(1..=1000_000_000);\n            let a = rng.gen_range(-1000_000_000..=1000_000_000);\n\
    \            let b = rng.gen_range(-1000_000_000..=1000_000_000);\n          \
    \  let mut ans = 0;\n            for i in 0..n {\n                ans += (a *\
    \ i as i64 + b + m - 1).div_euclid(m);\n            }\n            assert_eq!(ceil_sum(n,\
    \ m, a, b), ans);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/floor_ceil_sum/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-24 16:54:37+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/sum_of_floor_of_linear/src/main.rs
documentation_of: crates/math/floor_ceil_sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/floor_ceil_sum/src/lib.rs
- /library/crates/math/floor_ceil_sum/src/lib.rs.html
title: crates/math/floor_ceil_sum/src/lib.rs
---
