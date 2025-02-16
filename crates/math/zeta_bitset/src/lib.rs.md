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
    - https://ikatakos.com/pot/programming_algorithm/dynamic_programming/subset_convolution
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! bit\u306E\u4E0A\u4F4D\u30FB\u4E0B\u4F4D\u96C6\u5408\u306B\u95A2\u3059\
    \u308B\u9AD8\u901F\u30BC\u30FC\u30BF\u30FB\u30E1\u30D3\u30A6\u30B9\u5909\u63DB\
    \  \n//! <https://ikatakos.com/pot/programming_algorithm/dynamic_programming/subset_convolution>\n\
    \nuse std::ops::Sub;\n\n/// bit\u306E\u4E0A\u4F4D\u96C6\u5408\u306B\u95A2\u3059\
    \u308B\u9AD8\u901F\u30BC\u30FC\u30BF\u5909\u63DB  \n/// list[i] = func({list[i\u306E\
    superset\u9054]}) \u306B\u5909\u63DB\u3059\u308B  \n/// \u53EF\u63DB\u306A\u4E8C\
    \u9805\u6F14\u7B97`func`\u3092\u6307\u5B9A\u3059\u308B\npub fn superset_zeta<T:\
    \ Copy>(mut list: Vec<T>, func: impl Fn(T, T) -> T) -> Vec<T> {\n    let len =\
    \ list.len();\n    assert!(len.is_power_of_two());\n    let bit = len.trailing_zeros();\n\
    \    for j in 0..bit {\n        for i in 0..len {\n            if i & (1 << j)\
    \ == 0 {\n                list[i] = func(list[i], list[i | (1 << j)]);\n     \
    \       }\n        }\n    }\n    list\n}\n\n/// bit\u306E\u4E0A\u4F4D\u96C6\u5408\
    \u306B\u95A2\u3059\u308B\u9AD8\u901F\u30E1\u30D3\u30A6\u30B9\u5909\u63DB(\u52A0\
    \u7B97\u306E\u9006\u6F14\u7B97)\npub fn superset_mobius<T: Sub<Output = T> + Copy>(mut\
    \ list: Vec<T>) -> Vec<T> {\n    let len = list.len();\n    assert!(len.is_power_of_two());\n\
    \    let bit = len.trailing_zeros();\n    for j in 0..bit {\n        for i in\
    \ 0..len {\n            if i & (1 << j) == 0 {\n                list[i] = list[i]\
    \ - list[i | (1 << j)];\n            }\n        }\n    }\n    list\n}\n\n/// bit\u306E\
    \u4E0B\u4F4D\u96C6\u5408\u306B\u95A2\u3059\u308B\u9AD8\u901F\u30BC\u30FC\u30BF\
    \u5909\u63DB\n/// list[i] = func({list[i\u306Esubset\u9054]}) \u306B\u5909\u63DB\
    \u3059\u308B\n/// \u53EF\u63DB\u306A\u4E8C\u9805\u6F14\u7B97`func`\u3092\u6307\
    \u5B9A\u3059\u308B\npub fn subset_zeta<T: Copy>(mut list: Vec<T>, func: impl Fn(T,\
    \ T) -> T) -> Vec<T> {\n    let len = list.len();\n    assert!(len.is_power_of_two());\n\
    \    let bit = len.trailing_zeros();\n    for j in 0..bit {\n        for i in\
    \ 0..len {\n            if i & (1 << j) != 0 {\n                list[i] = func(list[i],\
    \ list[i ^ (1 << j)]);\n            }\n        }\n    }\n    list\n}\n\n/// bit\u306E\
    \u4E0B\u4F4D\u96C6\u5408\u306B\u95A2\u3059\u308B\u9AD8\u901F\u30E1\u30D3\u30A6\
    \u30B9\u5909\u63DB(\u52A0\u7B97\u306E\u9006\u6F14\u7B97)\npub fn subset_mobius<T:\
    \ Sub<Output = T> + Copy>(mut list: Vec<T>) -> Vec<T> {\n    let len = list.len();\n\
    \    assert!(len.is_power_of_two());\n    let bit = len.trailing_zeros();\n  \
    \  for j in 0..bit {\n        for i in 0..len {\n            if i & (1 << j) !=\
    \ 0 {\n                list[i] = list[i] - list[i ^ (1 << j)];\n            }\n\
    \        }\n    }\n    list\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\
    \    use rand::prelude::*;\n\n    #[test]\n    fn test_superset_zeta() {\n   \
    \     const N: usize = 1 << 10;\n        let mut rng = thread_rng();\n       \
    \ let list = (0..N)\n            .map(|_| rng.gen_range(-100..=100))\n       \
    \     .collect::<Vec<i64>>();\n        let superset = superset_zeta(list.clone(),\
    \ |a, b| a + b);\n        let naive = (0..N)\n            .map(|i| {\n       \
    \         (0..N)\n                    .filter(|&j| (i & j) == i)\n           \
    \         .map(|j| list[j])\n                    .sum::<i64>()\n            })\n\
    \            .collect::<Vec<_>>();\n        assert_eq!(superset, naive);\n   \
    \ }\n\n    #[test]\n    fn test_superset_mobius() {\n        const N: usize =\
    \ 1 << 10;\n        let mut rng = thread_rng();\n        let list = (0..N)\n \
    \           .map(|_| rng.gen_range(-100..=100))\n            .collect::<Vec<i64>>();\n\
    \        let superset = superset_zeta(list.clone(), |a, b| a + b);\n        let\
    \ mobius = superset_mobius(superset);\n        assert_eq!(list, mobius);\n   \
    \ }\n\n    #[test]\n    fn test_subset_zeta() {\n        const N: usize = 1 <<\
    \ 10;\n        let mut rng = thread_rng();\n        let list = (0..N)\n      \
    \      .map(|_| rng.gen_range(-100..=100))\n            .collect::<Vec<i64>>();\n\
    \        let subset = subset_zeta(list.clone(), |a, b| a + b);\n        let naive\
    \ = (0..N)\n            .map(|i| {\n                (0..N)\n                 \
    \   .filter(|&j| (i | j) == i)\n                    .map(|j| list[j])\n      \
    \              .sum::<i64>()\n            })\n            .collect::<Vec<_>>();\n\
    \        assert_eq!(subset, naive);\n    }\n\n    #[test]\n    fn test_subset_mobius()\
    \ {\n        const N: usize = 1 << 10;\n        let mut rng = thread_rng();\n\
    \        let list = (0..N)\n            .map(|_| rng.gen_range(-100..=100))\n\
    \            .collect::<Vec<i64>>();\n        let subset = subset_zeta(list.clone(),\
    \ |a, b| a + b);\n        let mobius = subset_mobius(subset);\n        assert_eq!(list,\
    \ mobius);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/zeta_bitset/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-16 13:49:39+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/zeta_bitset/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/zeta_bitset/src/lib.rs
- /library/crates/math/zeta_bitset/src/lib.rs.html
title: crates/math/zeta_bitset/src/lib.rs
---
