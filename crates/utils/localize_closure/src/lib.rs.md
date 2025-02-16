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
    - https://github.com/to-omer/competitive-library/blob/master/crates/competitive/src/tools/mlambda.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/to-omer/competitive-library/blob/master/crates/competitive/src/tools/mlambda.rs>\n\
    \n/// Macro that define closure like macro. Unlike closure, this macro localizes\
    \ variable capture.\n///\n/// # Example\n/// ```\n/// # use localize_closure::mlambda;\n\
    /// let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![2], vec![]];\n/// let mut\
    \ deq = std::collections::VecDeque::new();\n/// let mut dist: Vec<usize> = vec![!0;\
    \ 3];\n/// mlambda!(\n///     fn push(v: usize, cost: usize) {\n///         if\
    \ dist[v] > cost {\n///             dist[v] = cost;\n///             deq.push_back(v);\n\
    ///         }\n///     }\n/// );\n/// push!(0, 0);\n/// while let Some(v) = deq.pop_front()\
    \ {\n///     for &to in &graph[v] {\n///         push!(to, dist[v] + 1);\n///\
    \     }\n/// }\n/// assert_eq!(vec![0, 1, 1], dist);\n/// ```\n#[macro_export]\n\
    macro_rules! mlambda {\n    (\n        @def ($dol:tt) [$([$x:ident])*][$([$y:ident,\
    \ $($z:tt)*])*]\n        fn $name:ident($($args:tt)*) -> $ret:ty $body:block\n\
    \    ) => {\n        macro_rules! $name {\n            ($($dol $x:expr),* $dol(,)?)\
    \ => {{\n                $(let $y $($z)* = $dol $y;)*\n                $body\n\
    \            }}\n        }\n    };\n    (@pre () [$($x:tt)*][$($y:tt)*] fn $name:ident($($args:tt)*)\
    \ -> $ret:ty $body:block) => {\n        $crate::mlambda!(@def ($) [$($x)*][$($y)*]\
    \ fn $name($($args)*) -> $ret $body)\n    };\n    (@pre () [$($x:tt)*][$($y:tt)*]\
    \ fn $name:ident($($args:tt)*) $body:block) => {\n        $crate::mlambda!(@pre\
    \ () [$($x)*][$($y)*] fn $name($($args)*) -> () $body)\n    };\n    (@pre ($arg:ident\
    \ $(:$ty:ty)?) [$($x:tt)*][$($y:tt)*] $($rest:tt)*) => {\n        $crate::mlambda!(@pre\
    \ () [$($x)* [$arg]][$($y)* [$arg, $(:$ty)?]] $($rest)*)\n    };\n    (@pre ($arg:ident\
    \ $(:$ty:ty)?, $($args:tt)*) [$($x:tt)*][$($y:tt)*] $($rest:tt)*) => {\n     \
    \   $crate::mlambda!(@pre ($($args)*) [$($x)* [$arg]][$($y)* [$arg, $(:$ty)?]]\
    \ $($rest)*)\n    };\n    (fn $name:ident($($args:tt)*) $($rest:tt)*) => {\n \
    \       $crate::mlambda!(@pre ($($args)*) [][] fn $name($($args)*) $($rest)*)\n\
    \    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/utils/localize_closure/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-09 18:05:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/utils/localize_closure/src/lib.rs
layout: document
redirect_from:
- /library/crates/utils/localize_closure/src/lib.rs
- /library/crates/utils/localize_closure/src/lib.rs.html
title: crates/utils/localize_closure/src/lib.rs
---
