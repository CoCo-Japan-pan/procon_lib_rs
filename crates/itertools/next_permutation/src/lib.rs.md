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
    - https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/next_permutation/src/lib.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// From <https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/next_permutation/src/lib.rs>\
    \  \n/// Returns the next permutation of `a` in lexicographic order.  \n/// \u3053\
    \u308C\u306F\u91CD\u8907\u3092\u9664\u53BB\u3059\u308B\u306E\u3067\u3001itertools\u306E\
    permutations\u3068\u306F\u7570\u306A\u308B\uFF01\npub fn next_permutation<T: Ord>(a:\
    \ &mut [T]) -> bool {\n    let Some(i) = a.windows(2).rposition(|w| w[0] < w[1])\
    \ else {\n        return false;\n    };\n    let j = a.iter().rposition(|x| x\
    \ > &a[i]).unwrap();\n    a.swap(i, j);\n    a[i + 1..].reverse();\n    true\n\
    }\n\n/// \u6700\u521D\u306B\u30BD\u30FC\u30C8\u3057\u3001\u5168\u3066\u306E\u9806\
    \u5217\u3092\u5217\u6319\u3059\u308B\u30A4\u30C6\u30EC\u30FC\u30BF\u3092\u8FD4\
    \u3059\npub fn permutations<T: Ord + Clone>(mut start_vec: Vec<T>) -> Permutations<T>\
    \ {\n    start_vec.sort_unstable();\n    Permutations::new(start_vec)\n}\n\npub\
    \ struct Permutations<T: Ord + Clone> {\n    data: Vec<T>,\n    first: bool,\n\
    }\n\nimpl<T: Ord + Clone> Permutations<T> {\n    fn new(data: Vec<T>) -> Self\
    \ {\n        Permutations { data, first: true }\n    }\n}\n\nimpl<T: Ord + Clone>\
    \ Iterator for Permutations<T> {\n    type Item = Vec<T>;\n\n    fn next(&mut\
    \ self) -> Option<Self::Item> {\n        if self.first {\n            self.first\
    \ = false;\n            return Some(self.data.clone());\n        }\n        if\
    \ next_permutation(&mut self.data) {\n            Some(self.data.clone())\n  \
    \      } else {\n            None\n        }\n    }\n}\n\n#[cfg(test)]\nmod tests\
    \ {\n    use super::*;\n\n    #[test]\n    fn test_permutations() {\n        let\
    \ mut perms = permutations((0..3).collect());\n        assert_eq!(perms.next().unwrap(),\
    \ vec![0, 1, 2]);\n        assert_eq!(perms.next().unwrap(), vec![0, 2, 1]);\n\
    \        assert_eq!(perms.next().unwrap(), vec![1, 0, 2]);\n        assert_eq!(perms.next().unwrap(),\
    \ vec![1, 2, 0]);\n        assert_eq!(perms.next().unwrap(), vec![2, 0, 1]);\n\
    \        assert_eq!(perms.next().unwrap(), vec![2, 1, 0]);\n        assert!(perms.next().is_none());\n\
    \    }\n\n    #[test]\n    fn test_daburi_strings() {\n        let mut perms =\
    \ permutations(\"aba\".chars().collect());\n        assert_eq!(perms.next().unwrap(),\
    \ vec!['a', 'a', 'b']);\n        assert_eq!(perms.next().unwrap(), vec!['a', 'b',\
    \ 'a']);\n        assert_eq!(perms.next().unwrap(), vec!['b', 'a', 'a']);\n  \
    \      assert!(perms.next().is_none());\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/itertools/next_permutation/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-06 16:15:33+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/itertools/next_permutation/src/lib.rs
layout: document
redirect_from:
- /library/crates/itertools/next_permutation/src/lib.rs
- /library/crates/itertools/next_permutation/src/lib.rs.html
title: crates/itertools/next_permutation/src/lib.rs
---
