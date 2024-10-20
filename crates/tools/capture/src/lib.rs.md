---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc359g_centroid/src/main.rs
    title: verify/AtCoder/abc359g_centroid/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
    title: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/to-omer/competitive-library/blob/master/crates/competitive/src/tools/capture.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/to-omer/competitive-library/blob/master/crates/competitive/src/tools/capture.rs>\n\
    \n/// Macro that returns a recursive function that (semi-)automatically captures.\n\
    ///\n/// # Example\n/// default version\n/// ```\n/// # use capture::crecurse;\n\
    /// let mut res = 0usize;\n/// let coeff = 3usize;\n/// crecurse!(\n///     //\
    \ (1) semi-automatically capture mutable reference (res: &mut usize)\n///    \
    \ [res: usize],\n///     fn mul(x: usize, y: usize) {\n///         if y > 0 {\n\
    ///             if y % 2 == 1 {\n///                 // (2) automatically capture\
    \ reference (coeff: &usize)\n///                 *res += coeff * x;\n///     \
    \        }\n///             // (3) call macro to recurse\n///             mul!(x\
    \ + x, y / 2);\n///         }\n///     }\n/// )(10, 19); // (4) macro returns\
    \ captured version of the recursive function\n/// assert_eq!(res, coeff * 10 *\
    \ 19);\n/// ```\n///\n/// unsafe version (automatically capture everything)\n\
    /// ```\n/// # use capture::crecurse;\n/// let mut res = 0usize;\n/// let coeff\
    \ = 3usize;\n/// crecurse!(\n///     unsafe fn mul(x: usize, y: usize) {\n///\
    \         if y > 0 {\n///             if y % 2 == 1 {\n///                 res\
    \ += coeff * x;\n///             }\n///             mul!(x + x, y / 2);\n/// \
    \        }\n///     }\n/// )(10, 19);\n/// assert_eq!(res, coeff * 10 * 19);\n\
    /// ```\n///\n/// no overhead version (semi-automatically capture everything)\n\
    /// ```\n/// # use capture::crecurse;\n/// let mut res = 0usize;\n/// let coeff\
    \ = 3usize;\n/// crecurse!(\n///     [res: &mut usize, coeff: &usize],\n///  \
    \   static fn mul(x: usize, y: usize) {\n///         if y > 0 {\n///         \
    \    if y % 2 == 1 {\n///                 *res += coeff * x;\n///            \
    \ }\n///             mul!(x + x, y / 2);\n///         }\n///     }\n/// )(10,\
    \ 19);\n/// assert_eq!(res, coeff * 10 * 19);\n/// ```\n///\n/// # Syntax\n///\
    \ ```txt\n/// crecurse!(\n///     ([($ident: $type),*,?],)?\n///     (unsafe|static)?\
    \ fn $ident\\(($ident: $type),*,?\\) (-> $type)? $block\n/// )\n/// ```\n#[macro_export]\n\
    macro_rules! crecurse {\n    (@macro_def ($dol:tt) $name:ident $($cargs:ident)*)\
    \ => {\n        #[allow(unused_macros)]\n        macro_rules! $name { ($dol($dol\
    \ args:expr),*) => { $name($dol($dol args,)* $($cargs,)* ) } }\n    };\n\n   \
    \ (\n        @static [$(($cargs:ident, $cargsexpr:expr, $cargsty:ty))*] [$(,)?],\n\
    \        fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) -> $ret:ty $body:block\n\
    \    ) => {{\n        fn $func($($args: $argsty,)* $($cargs: $cargsty,)*) -> $ret\
    \ {\n            $crate::crecurse!(@macro_def ($) $func $($cargs)*);\n       \
    \     $body\n        }\n        |$($args: $argsty,)*| -> $ret { $func($($args,)*\
    \ $($cargsexpr,)*) }\n    }};\n    (@static [$($pcaps:tt)*] [$(,)?], fn $func:ident\
    \ ($($argstt:tt)*) $($rest:tt)*) => {\n        $crate::crecurse!(@static [$($pcaps)*]\
    \ [], fn $func ($($argstt)*) -> () $($rest)*)\n    };\n    (@static [$($pcaps:tt)*]\
    \ [$carg:ident: &mut $cargty:ty, $($caps:tt)*], $($rest:tt)*) => {\n        $crate::crecurse!(@static\
    \ [$($pcaps)* ($carg, &mut $carg, &mut $cargty)] [$($caps)*], $($rest)*)\n   \
    \ };\n    (@static [$($pcaps:tt)*] [$carg:ident: &$cargty:ty, $($caps:tt)*], $($rest:tt)*)\
    \ => {\n        $crate::crecurse!(@static [$($pcaps)* ($carg, &$carg, &$cargty)]\
    \ [$($caps)*], $($rest)*)\n    };\n    (@static [$($pcaps:tt)*] [$carg:ident:\
    \ $cargty:ty, $($caps:tt)*], $($rest:tt)*) => {\n        $crate::crecurse!(@static\
    \ [$($pcaps)* ($carg, $carg, $cargty)] [$($caps)*], $($rest)*)\n    };\n    ($([$($caps:tt)*],)?\
    \ static fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) $($rest:tt)*) =>\
    \ {\n        $crate::crecurse!(@static [] [$($($caps)*)?,], fn $func ($($args:\
    \ $argsty),*) $($rest)*)\n    };\n\n    (\n        @default [$($cargs:ident: $cargsty:ty),*\
    \ $(,)?],\n        fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) -> $ret:ty\
    \ $body:block\n    ) => {{\n        fn call<F>(f: &F, $($args: $argsty,)* $($cargs:\
    \ &mut $cargsty,)*) -> $ret\n        where\n            F: Fn(&dyn Fn($($argsty,)*\
    \ $(&mut $cargsty,)*) -> $ret, $($argsty,)* $(&mut $cargsty,)*) -> $ret,\n   \
    \     {\n            f(\n                &|$($args: $argsty,)* $($cargs: &mut\
    \ $cargsty,)*| -> $ret {\n                    call(f, $($args,)* $($cargs,)*)\n\
    \                },\n                $($args,)* $($cargs,)*\n            )\n \
    \       }\n        |$($args: $argsty,)*| -> $ret {\n            call(\n      \
    \          &|$func, $($args: $argsty,)* $($cargs: &mut $cargsty,)*| -> $ret {\n\
    \                    $crate::crecurse!(@macro_def ($) $func $($cargs)*);\n   \
    \                 $body\n                },\n                $($args,)* $(&mut\
    \ $cargs,)*\n            )\n        }\n    }};\n    (@default [$($caps:tt)*],\
    \ fn $func:ident ($($argstt:tt)*) $($rest:tt)*) => {\n        $crate::crecurse!(@default\
    \ [$($caps)*], fn $func ($($argstt)*) -> () $($rest)*)\n    };\n    ($([$($caps:tt)*],)?\
    \ fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) $($rest:tt)*) => {\n   \
    \     $crate::crecurse!(@default [$($($caps)*)?], fn $func ($($args: $argsty),*)\
    \ $($rest)*)\n    };\n\n    (\n        @unsafe [$($cargs:ident: $cargsty:ty),*\
    \ $(,)?],\n        fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) -> $ret:ty\
    \ $body:block\n    ) => {{\n        fn call<F>(f: &mut F, $($args: $argsty,)*\
    \ $($cargs: &mut $cargsty,)*) -> $ret\n        where\n            F: FnMut(&mut\
    \ dyn FnMut($($argsty,)* $(&mut $cargsty,)*) -> $ret, $($argsty,)* $(&mut $cargsty,)*)\
    \ -> $ret,\n        {\n            let fp = f as *mut F;\n            (unsafe\
    \ { &mut *fp })(\n                &mut |$($args: $argsty,)* $($cargs: &mut $cargsty,)*|\
    \ -> $ret {\n                    call(unsafe { &mut *fp }, $($args,)* $($cargs,)*)\n\
    \                },\n                $($args,)* $($cargs,)*\n            )\n \
    \       }\n        |$($args: $argsty,)*| -> $ret {\n            call(\n      \
    \          &mut |$func, $($args: $argsty,)* $($cargs: &mut $cargsty,)*| -> $ret\
    \ {\n                    $crate::crecurse!(@macro_def ($) $func $($cargs)*);\n\
    \                    $body\n                },\n                $($args,)* $(&mut\
    \ $cargs,)*\n            )\n        }\n    }};\n\n    (@unsafe [$($caps:tt)*],\
    \ fn $func:ident ($($argstt:tt)*) $($rest:tt)*) => {\n        $crate::crecurse!(@unsafe\
    \ [$($caps)*], fn $func ($($argstt)*) -> () $($rest)*)\n    };\n    ($([$($caps:tt)*],)?\
    \ unsafe fn $func:ident ($($args:ident: $argsty:ty),* $(,)?) $($rest:tt)*) =>\
    \ {\n        $crate::crecurse!(@unsafe [$($($caps)*)?], fn $func ($($args: $argsty),*)\
    \ $($rest)*)\n    };\n    ($($t:tt)*) => {\n        ::std::compile_error!(::std::concat!(\"\
    invalid input: \", ::std::stringify!($($t)*)))\n    };\n}\n\n/// Automatic memorization\
    \ for recursive functions.\n///\n/// This macro binds memorized version of the\
    \ recursive functions to a local variable.\n/// The specification of the function\
    \ declaration part is the same as [`crecurse`].\n///\n/// [`crecurse`]: crate::crecurse\n\
    ///\n/// # Example\n/// ```\n/// # use capture::memorize;\n/// memorize!(\n///\
    \     fn comb(n: usize, r: usize) -> usize {\n///         if r > n {\n///    \
    \         0\n///         } else if r == 0 || r == n {\n///             1\n///\
    \         } else {\n///             comb!(n - 1, r) + comb!(n - 1, r - 1)\n///\
    \         }\n///     }\n/// );\n/// assert_eq!(comb(30, 12), 86493225);\n/// ```\n\
    #[macro_export]\nmacro_rules! memorize {\n    (\n        @inner [$map:ident, $Map:ty,\
    \ $init:expr]\n        fn $name:ident ($($args:ident: $argsty:ty),* $(,)?) ->\
    \ $ret:ty $body:block\n    ) => {\n        let mut $map: $Map = $init;\n     \
    \   #[allow(unused_mut)]\n        let mut $name = $crate::crecurse!(\n       \
    \     [$map: $Map],\n            fn $name ($($args: $argsty),*) -> $ret {\n  \
    \              if let Some(value) = $map.get(&($($args,)*)).cloned() {\n     \
    \               value\n                } else {\n                    let value\
    \ = (|| $body)();\n                    $map.insert(($($args,)*), value.clone());\n\
    \                    value\n                }\n            }\n        );\n   \
    \ };\n    (fn $name:ident ($($args:ident: $argsty:ty),* $(,)?) -> $ret:ty $body:block)\
    \ => {\n        $crate::memorize!(\n            @inner [\n                __memorize_map,\n\
    \                ::std::collections::HashMap<($($argsty,)*), $ret>,\n        \
    \        ::std::default::Default::default()\n            ]\n            fn $name\
    \ ($($args: $argsty),*) -> $ret $body\n        );\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tools/capture/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-20 16:41:21+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  - verify/AtCoder/abc359g_centroid/src/main.rs
documentation_of: crates/tools/capture/src/lib.rs
layout: document
redirect_from:
- /library/crates/tools/capture/src/lib.rs
- /library/crates/tools/capture/src/lib.rs.html
title: crates/tools/capture/src/lib.rs
---
