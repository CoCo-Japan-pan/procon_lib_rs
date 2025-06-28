---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/suffixarray/src/main.rs
    title: verify/yosupo/suffixarray/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/string.rs
    - https://mametter.hatenablog.com/entry/20180130/p1
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/string.rs>\n\
    \n#![allow(clippy::many_single_char_names)]\n\nfn sa_naive<T: Ord>(s: &[T]) ->\
    \ Vec<usize> {\n    let n = s.len();\n    let mut sa: Vec<usize> = (0..n).collect();\n\
    \    sa.sort_by(|&(mut l), &(mut r)| {\n        if l == r {\n            return\
    \ std::cmp::Ordering::Equal;\n        }\n        while l < n && r < n {\n    \
    \        if s[l] != s[r] {\n                return s[l].cmp(&s[r]);\n        \
    \    }\n            l += 1;\n            r += 1;\n        }\n        if l == n\
    \ {\n            std::cmp::Ordering::Less\n        } else {\n            std::cmp::Ordering::Greater\n\
    \        }\n    });\n    sa\n}\n\nfn sa_doubling(s: &[i32]) -> Vec<usize> {\n\
    \    let n = s.len();\n    let mut sa: Vec<usize> = (0..n).collect();\n    let\
    \ mut rnk: Vec<i32> = s.to_vec();\n    let mut tmp = vec![0; n];\n    let mut\
    \ k = 1;\n    while k < n {\n        let cmp = |&x: &usize, &y: &usize| {\n  \
    \          if rnk[x] != rnk[y] {\n                return rnk[x].cmp(&rnk[y]);\n\
    \            }\n            let rx = if x + k < n { rnk[x + k] } else { -1 };\n\
    \            let ry = if y + k < n { rnk[y + k] } else { -1 };\n            rx.cmp(&ry)\n\
    \        };\n        sa.sort_by(cmp);\n        tmp[sa[0]] = 0;\n        for i\
    \ in 1..n {\n            tmp[sa[i]] =\n                tmp[sa[i - 1]] + i32::from(cmp(&sa[i\
    \ - 1], &sa[i]) == std::cmp::Ordering::Less);\n        }\n        std::mem::swap(&mut\
    \ tmp, &mut rnk);\n        k *= 2;\n    }\n    sa\n}\n\ntrait Threshold {\n  \
    \  fn threshold_naive() -> usize;\n    fn threshold_doubling() -> usize;\n}\n\n\
    enum DefaultThreshold {}\nimpl Threshold for DefaultThreshold {\n    fn threshold_naive()\
    \ -> usize {\n        10\n    }\n    fn threshold_doubling() -> usize {\n    \
    \    40\n    }\n}\n\n#[allow(clippy::cognitive_complexity)]\nfn sa_is<T: Threshold>(s:\
    \ &[usize], upper: usize) -> Vec<usize> {\n    let n = s.len();\n    match n {\n\
    \        0 => return vec![],\n        1 => return vec![0],\n        2 => return\
    \ if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] },\n        _ => (),\n    }\n\
    \    if n < T::threshold_naive() {\n        return sa_naive(s);\n    }\n    if\
    \ n < T::threshold_doubling() {\n        let s: Vec<i32> = s.iter().map(|&x| x\
    \ as i32).collect();\n        return sa_doubling(&s);\n    }\n    let mut sa =\
    \ vec![0; n];\n    let mut ls = vec![false; n];\n    for i in (0..n - 1).rev()\
    \ {\n        ls[i] = if s[i] == s[i + 1] {\n            ls[i + 1]\n        } else\
    \ {\n            s[i] < s[i + 1]\n        };\n    }\n    let mut sum_l = vec![0;\
    \ upper + 1];\n    let mut sum_s = vec![0; upper + 1];\n    for i in 0..n {\n\
    \        if !ls[i] {\n            sum_s[s[i]] += 1;\n        } else {\n      \
    \      sum_l[s[i] + 1] += 1;\n        }\n    }\n    for i in 0..=upper {\n   \
    \     sum_s[i] += sum_l[i];\n        if i < upper {\n            sum_l[i + 1]\
    \ += sum_s[i];\n        }\n    }\n\n    // sa's origin is 1.\n    let induce =\
    \ |sa: &mut [usize], lms: &[usize]| {\n        for elem in sa.iter_mut() {\n \
    \           *elem = 0;\n        }\n        let mut buf = sum_s.clone();\n    \
    \    for &d in lms {\n            if d == n {\n                continue;\n   \
    \         }\n            let old = buf[s[d]];\n            buf[s[d]] += 1;\n \
    \           sa[old] = d + 1;\n        }\n        buf.copy_from_slice(&sum_l);\n\
    \        let old = buf[s[n - 1]];\n        buf[s[n - 1]] += 1;\n        sa[old]\
    \ = n;\n        for i in 0..n {\n            let v = sa[i];\n            if v\
    \ >= 2 && !ls[v - 2] {\n                let old = buf[s[v - 2]];\n           \
    \     buf[s[v - 2]] += 1;\n                sa[old] = v - 1;\n            }\n \
    \       }\n        buf.copy_from_slice(&sum_l);\n        for i in (0..n).rev()\
    \ {\n            let v = sa[i];\n            if v >= 2 && ls[v - 2] {\n      \
    \          buf[s[v - 2] + 1] -= 1;\n                sa[buf[s[v - 2] + 1]] = v\
    \ - 1;\n            }\n        }\n    };\n    // origin: 1\n    let mut lms_map\
    \ = vec![0; n + 1];\n    let mut m = 0;\n    for i in 1..n {\n        if !ls[i\
    \ - 1] && ls[i] {\n            lms_map[i] = m + 1;\n            m += 1;\n    \
    \    }\n    }\n    let mut lms = Vec::with_capacity(m);\n    for i in 1..n {\n\
    \        if !ls[i - 1] && ls[i] {\n            lms.push(i);\n        }\n    }\n\
    \    assert_eq!(lms.len(), m);\n    induce(&mut sa, &lms);\n\n    if m > 0 {\n\
    \        let mut sorted_lms = Vec::with_capacity(m);\n        for &v in &sa {\n\
    \            if lms_map[v - 1] != 0 {\n                sorted_lms.push(v - 1);\n\
    \            }\n        }\n        let mut rec_s = vec![0; m];\n        let mut\
    \ rec_upper = 0;\n        rec_s[lms_map[sorted_lms[0]] - 1] = 0;\n        for\
    \ i in 1..m {\n            let mut l = sorted_lms[i - 1];\n            let mut\
    \ r = sorted_lms[i];\n            let end_l = if lms_map[l] < m { lms[lms_map[l]]\
    \ } else { n };\n            let end_r = if lms_map[r] < m { lms[lms_map[r]] }\
    \ else { n };\n            let same = if end_l - l != end_r - r {\n          \
    \      false\n            } else {\n                while l < end_l {\n      \
    \              if s[l] != s[r] {\n                        break;\n           \
    \         }\n                    l += 1;\n                    r += 1;\n      \
    \          }\n                l != n && s[l] == s[r]\n            };\n       \
    \     if !same {\n                rec_upper += 1;\n            }\n           \
    \ rec_s[lms_map[sorted_lms[i]] - 1] = rec_upper;\n        }\n\n        let rec_sa\
    \ = sa_is::<T>(&rec_s, rec_upper);\n        for i in 0..m {\n            sorted_lms[i]\
    \ = lms[rec_sa[i]];\n        }\n        induce(&mut sa, &mut sorted_lms);\n  \
    \  }\n    for elem in sa.iter_mut() {\n        *elem -= 1;\n    }\n    sa\n}\n\
    \nfn sa_is_i32<T: Threshold>(s: &[i32], upper: i32) -> Vec<usize> {\n    let s:\
    \ Vec<usize> = s.iter().map(|&x| x as usize).collect();\n    sa_is::<T>(&s, upper\
    \ as usize)\n}\n\npub fn suffix_array_manual(s: &[i32], upper: i32) -> Vec<usize>\
    \ {\n    assert!(upper >= 0);\n    for &elem in s {\n        assert!(0 <= elem\
    \ && elem <= upper);\n    }\n    sa_is_i32::<DefaultThreshold>(s, upper)\n}\n\n\
    pub fn suffix_array_arbitrary<T: Ord>(s: &[T]) -> Vec<usize> {\n    let n = s.len();\n\
    \    let mut idx: Vec<usize> = (0..n).collect();\n    idx.sort_by_key(|&i| &s[i]);\n\
    \    let mut s2 = vec![0; n];\n    let mut now = 0;\n    for i in 0..n {\n   \
    \     if i > 0 && s[idx[i - 1]] != s[idx[i]] {\n            now += 1;\n      \
    \  }\n        s2[idx[i]] = now;\n    }\n    sa_is_i32::<DefaultThreshold>(&s2,\
    \ now)\n}\n\npub fn suffix_array(s: &str) -> Vec<usize> {\n    let s2: Vec<usize>\
    \ = s.bytes().map(|x| x as usize).collect();\n    sa_is::<DefaultThreshold>(&s2,\
    \ 255)\n}\n\n// Reference:\n// T. Kasai, G. Lee, H. Arimura, S. Arikawa, and K.\
    \ Park,\n// Linear-Time Longest-Common-Prefix Computation in Suffix Arrays and\
    \ Its\n// Applications\npub fn lcp_array_arbitrary<T: Ord>(s: &[T], sa: &[usize])\
    \ -> Vec<usize> {\n    let n = s.len();\n    assert!(n >= 1);\n    let mut rnk\
    \ = vec![0; n];\n    for i in 0..n {\n        rnk[sa[i]] = i;\n    }\n    let\
    \ mut lcp = vec![0; n - 1];\n    let mut h: usize = 0;\n    for i in 0..n - 1\
    \ {\n        h = h.saturating_sub(1);\n        if rnk[i] == 0 {\n            continue;\n\
    \        }\n        let j = sa[rnk[i] - 1];\n        while j + h < n && i + h\
    \ < n {\n            if s[j + h] != s[i + h] {\n                break;\n     \
    \       }\n            h += 1;\n        }\n        lcp[rnk[i] - 1] = h;\n    }\n\
    \    lcp\n}\n\npub fn lcp_array(s: &str, sa: &[usize]) -> Vec<usize> {\n    let\
    \ s: &[u8] = s.as_bytes();\n    lcp_array_arbitrary(s, sa)\n}\n\n// Reference:\n\
    // D. Gusfield,\n// Algorithms on Strings, Trees, and Sequences: Computer Science\
    \ and\n// Computational Biology\npub fn z_algorithm_arbitrary<T: Ord>(s: &[T])\
    \ -> Vec<usize> {\n    let n = s.len();\n    if n == 0 {\n        return vec![];\n\
    \    }\n    let mut z = vec![0; n];\n    z[0] = 0;\n    let mut j = 0;\n    for\
    \ i in 1..n {\n        let mut k = if j + z[j] <= i {\n            0\n       \
    \ } else {\n            std::cmp::min(j + z[j] - i, z[i - j])\n        };\n  \
    \      while i + k < n && s[k] == s[i + k] {\n            k += 1;\n        }\n\
    \        z[i] = k;\n        if j + z[j] < i + z[i] {\n            j = i;\n   \
    \     }\n    }\n    z[0] = n;\n    z\n}\n\npub fn z_algorithm(s: &str) -> Vec<usize>\
    \ {\n    let s: &[u8] = s.as_bytes();\n    z_algorithm_arbitrary(s)\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use super::*;\n\n    enum ZeroThreshold {}\n    impl Threshold\
    \ for ZeroThreshold {\n        fn threshold_naive() -> usize {\n            0\n\
    \        }\n        fn threshold_doubling() -> usize {\n            0\n      \
    \  }\n    }\n\n    fn verify_all(str: &str, expected_array: &[usize]) {\n    \
    \    let array: Vec<i32> = str.bytes().map(|x| x as i32).collect();\n        let\
    \ sa = sa_doubling(&array);\n        assert_eq!(sa, expected_array);\n       \
    \ let sa_naive = sa_naive(&array);\n        assert_eq!(sa_naive, expected_array);\n\
    \        let sa_is = sa_is_i32::<ZeroThreshold>(&array, 255);\n        assert_eq!(sa_is,\
    \ expected_array);\n\n        let sa_str = suffix_array(str);\n        assert_eq!(sa_str,\
    \ expected_array);\n    }\n\n    #[test]\n    fn test_sa_0() {\n        let array\
    \ = vec![0, 1, 2, 3, 4];\n        let sa = sa_doubling(&array);\n        assert_eq!(sa,\
    \ vec![0, 1, 2, 3, 4]);\n    }\n\n    #[test]\n    fn test_sa_1() {\n        let\
    \ str = \"abracadabra\";\n        verify_all(str, &[10, 7, 0, 3, 5, 8, 1, 4, 6,\
    \ 9, 2]);\n    }\n\n    #[test]\n    fn test_sa_2() {\n        let str = \"mmiissiissiippii\"\
    ; // an example taken from https://mametter.hatenablog.com/entry/20180130/p1\n\
    \        verify_all(str, &[15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8,\
    \ 4]);\n    }\n\n    #[test]\n    fn test_lcp_0() {\n        let str = \"abracadabra\"\
    ;\n        let sa = suffix_array(str);\n        let lcp = lcp_array(str, &sa);\n\
    \        assert_eq!(lcp, &[1, 4, 1, 1, 0, 3, 0, 0, 0, 2]);\n    }\n\n    #[test]\n\
    \    fn test_lcp_1() {\n        let str = \"mmiissiissiippii\"; // an example\
    \ taken from https://mametter.hatenablog.com/entry/20180130/p1\n        let sa\
    \ = suffix_array(str);\n        let lcp = lcp_array(str, &sa);\n        assert_eq!(lcp,\
    \ &[1, 2, 2, 6, 1, 1, 5, 0, 1, 0, 1, 0, 3, 1, 4]);\n    }\n\n    #[test]\n   \
    \ fn test_z_0() {\n        let str = \"abracadabra\";\n        let lcp = z_algorithm(str);\n\
    \        assert_eq!(lcp, &[11, 0, 0, 1, 0, 1, 0, 4, 0, 0, 1]);\n    }\n\n    #[test]\n\
    \    fn test_z_1() {\n        let str = \"ababababa\";\n        let lcp = z_algorithm(str);\n\
    \        assert_eq!(lcp, &[9, 0, 7, 0, 5, 0, 3, 0, 1]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/atcoder_string/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-01 11:52:10+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/suffixarray/src/main.rs
documentation_of: crates/string/atcoder_string/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/atcoder_string/src/lib.rs
- /library/crates/string/atcoder_string/src/lib.rs.html
title: crates/string/atcoder_string/src/lib.rs
---
