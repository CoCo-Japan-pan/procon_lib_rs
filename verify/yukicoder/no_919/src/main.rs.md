---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix/src/lib.rs
    title: crates/wavelet/wavelet_matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/919
    links:
    - https://maguro.dev/debug-macro/
    - https://yukicoder.me/problems/no/919
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/919\n\n\
    #![allow(non_snake_case)]\nuse proconio::{fastout, input};\nuse wavelet_matrix::WaveletMatrix;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        N: usize,\n        A: [i64; N],\n\
    \    }\n    let sorted_a = {\n        let mut a = A.clone();\n        a.sort_unstable();\n\
    \        a.dedup();\n        a\n    };\n    let compressed = A\n        .iter()\n\
    \        .map(|a| sorted_a.binary_search(a).unwrap())\n        .collect::<Vec<_>>();\n\
    \    let wm = WaveletMatrix::new(&compressed);\n    let mut ans = 0;\n    for\
    \ k in 1..=N {\n        let all_teams = N / k;\n        let median_id = (k + 1)\
    \ / 2 - 1;\n        let left_sum = {\n            let mut left_sum = vec![0; all_teams\
    \ + 1];\n            for i in 0..all_teams {\n                // [i * k, (i +\
    \ 1) * k)\n                let left = i * k;\n                let right = (i +\
    \ 1) * k;\n                let mid = wm.quantile(left..right, median_id);\n  \
    \              let power = sorted_a[mid] * k as i64;\n                left_sum[i\
    \ + 1] = left_sum[i] + power;\n            }\n            left_sum\n        };\n\
    \        let right_sum = {\n            let mut right_sum = vec![0; all_teams\
    \ + 1];\n            for i in 0..all_teams {\n                // [N - (i + 1)\
    \ * k, N - i * k)\n                let left = N - (i + 1) * k;\n             \
    \   let right = N - i * k;\n                let mid = wm.quantile(left..right,\
    \ median_id);\n                let power = sorted_a[mid] * k as i64;\n       \
    \         right_sum[i + 1] = right_sum[i] + power;\n            }\n          \
    \  right_sum\n        };\n        debug!(k, all_teams, left_sum, right_sum);\n\
    \        // \u7D2F\u7A4DMax\n        let right_cnt_max = {\n            let mut\
    \ right_cnt_max = right_sum;\n            for i in 1..=all_teams {\n         \
    \       right_cnt_max[i] = right_cnt_max[i].max(right_cnt_max[i - 1]);\n     \
    \       }\n            right_cnt_max\n        };\n        for left_cnt in 0..=all_teams\
    \ {\n            let right_cnt = all_teams - left_cnt;\n            // [0, right_cnt]\u306E\
    \u4E2D\u3067\u306E\u6700\u5927\u5024\n            let power = left_sum[left_cnt]\
    \ + right_cnt_max[right_cnt];\n            ans = ans.max(power);\n        }\n\
    \    }\n    println!(\"{}\", ans);\n}\n\n/// https://maguro.dev/debug-macro/\n\
    #[macro_export]\nmacro_rules! debug {\n    ($($a:expr),* $(,)*) => {\n       \
    \ #[cfg(debug_assertions)]\n        eprintln!(concat!($(\"| \", stringify!($a),\
    \ \"={:?} \"),*, \"|\"), $(&$a),*);\n    };\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_919/src/main.rs
  requiredBy: []
  timestamp: '2024-12-16 14:54:34+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_919/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_919/src/main.rs
- /verify/verify/yukicoder/no_919/src/main.rs.html
title: verify/yukicoder/no_919/src/main.rs
---
