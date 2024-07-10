---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/math/bit_matrix/src/lib.rs
    title: crates/math/bit_matrix/src/lib.rs
  - icon: ':warning:'
    path: verify/AtCoder/typical_059/src/main.rs
    title: verify/AtCoder/typical_059/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://github.com/hatoo/bitset-fixed)
    - https://opensource.org/license/mit)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5185\u90E8\u3067u64\u306E\u914D\u5217\u3092\u6301\u3063\u3066\u30D3\
    \u30C3\u30C8\u6F14\u7B97\u3092\u307E\u3068\u3081\u3066\u884C\u3044\u300164\u500D\
    \u9AD8\u901F\u5316\u3092\u56F3\u308B  \n//! \u30B7\u30D5\u30C8\u6F14\u7B97\u3084\
    \u30C6\u30B9\u30C8\u306E\u30B3\u30FC\u30C9\u306B\u3064\u3044\u3066\u306F\n//!\
    \ [bitset-fixed](https://github.com/hatoo/bitset-fixed) Under [MIT License](https://opensource.org/license/mit)\n\
    //! \u3092\u4F7F\u7528\u3055\u305B\u3066\u3044\u305F\u3060\u3044\u3066\u3044\u307E\
    \u3059\n\nuse std::ops::{\n    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,\
    \ BitXorAssign, Index, Not, Shl, ShlAssign,\n    Shr, ShrAssign,\n};\n\n#[derive(Debug,\
    \ Clone, PartialEq, Eq, Hash)]\npub struct BitSet {\n    buf: Vec<u64>,\n    size:\
    \ usize,\n}\n\nmacro_rules! out_of_bounds {\n    ($size:expr, $i:expr) => {\n\
    \        assert!(\n            $i < $size,\n            \"index out of bounds:\
    \ the len is {} but the index is {}\",\n            $size,\n            $i\n \
    \       );\n    };\n}\n\nimpl From<Vec<bool>> for BitSet {\n    fn from(v: Vec<bool>)\
    \ -> Self {\n        let size = v.len();\n        let mut buf = vec![0; (size\
    \ + 63) / 64];\n        for i in 0..size {\n            if v[i] {\n          \
    \      buf[i >> 6] |= 1 << (i & 63);\n            }\n        }\n        Self {\
    \ buf, size }\n    }\n}\n\nimpl<const N: usize> From<[bool; N]> for BitSet {\n\
    \    fn from(v: [bool; N]) -> Self {\n        let size = N;\n        let mut buf\
    \ = vec![0; (size + 63) / 64];\n        for i in 0..size {\n            if v[i]\
    \ {\n                buf[i >> 6] |= 1 << (i & 63);\n            }\n        }\n\
    \        Self { buf, size }\n    }\n}\n\nimpl BitSet {\n    /// size\u500B\u306E\
    bit\u3092\u6301\u3064BitSet\u3092\u751F\u6210\u3059\u308B(\u3069\u308C\u3082unset)\n\
    \    pub fn new(size: usize) -> Self {\n        let len = (size + 63) / 64;\n\
    \        Self {\n            buf: vec![0; len],\n            size,\n        }\n\
    \    }\n\n    #[inline]\n    pub fn buffer(&self) -> &[u64] {\n        &self.buf\n\
    \    }\n\n    #[inline]\n    pub fn buffer_mut(&mut self) -> &mut [u64] {\n  \
    \      &mut self.buf\n    }\n\n    #[inline]\n    pub fn size(&self) -> usize\
    \ {\n        self.size\n    }\n\n    #[inline]\n    /// index\u3067\u30A2\u30AF\
    \u30BB\u30B9\u3057\u3066\u3082\u3088\u3044\n    pub fn get(&self, i: usize) ->\
    \ bool {\n        out_of_bounds!(self.size, i);\n        let x = self.buf[i >>\
    \ 6];\n        let mask = 1 << (i & 63);\n        (x & mask) != 0\n    }\n\n \
    \   /// i\u756A\u76EE\u306Ebit\u3092b\u306B\u8A2D\u5B9A\u3059\u308B\n    #[inline]\n\
    \    pub fn set(&mut self, i: usize, b: bool) {\n        out_of_bounds!(self.size,\
    \ i);\n        if b {\n            self.buf[i >> 6] |= 1 << (i & 63);\n      \
    \  } else {\n            self.buf[i >> 6] &= !(1 << (i & 63));\n        }\n  \
    \  }\n\n    /// i\u756A\u76EE\u306Ebit\u3092\u53CD\u8EE2\u3055\u305B\u308B\n \
    \   #[inline]\n    pub fn flip(&mut self, i: usize) {\n        out_of_bounds!(self.size,\
    \ i);\n        self.buf[i >> 6] ^= 1 << (i & 63);\n    }\n\n    /// 1\u306E\u6570\
    \u3092\u8FD4\u3059\n    #[inline]\n    pub fn count_ones(&self) -> u32 {\n   \
    \     self.buf.iter().map(|&x| x.count_ones()).sum()\n    }\n\n    /// 0\u306E\
    \u6570\u3092\u8FD4\u3059\n    #[inline]\n    pub fn count_zeros(&self) -> u32\
    \ {\n        self.size as u32 - self.count_ones()\n    }\n\n    /// \u5168\u3066\
    0\u304B\u3069\u3046\u304B\u3092\u8FD4\u3059\n    #[inline]\n    pub fn none(&self)\
    \ -> bool {\n        self.count_ones() == 0\n    }\n\n    /// \u5168\u30661\u304B\
    \u3069\u3046\u304B\u3092\u8FD4\u3059\n    #[inline]\n    pub fn all(&self) ->\
    \ bool {\n        self.count_ones() == self.size as u32\n    }\n\n    /// \u3069\
    \u308C\u304B1\u3064\u3067\u30821\u304C\u3042\u308B\u304B\u3069\u3046\u304B\u3092\
    \u8FD4\u3059\n    #[inline]\n    pub fn any(&self) -> bool {\n        self.count_ones()\
    \ > 0\n    }\n\n    /// \u7BC4\u56F2\u5916\u3060\u304C\u4F59\u5206\u306B\u6301\
    \u3063\u3066\u3044\u308Bbit\u30920\u306B\u3059\u308B\n    #[inline]\n    pub fn\
    \ chomp(&mut self) {\n        let r = self.size & 63;\n        if r > 0 {\n  \
    \          if let Some(last) = self.buf.last_mut() {\n                let d =\
    \ 64 - r;\n                *last = (*last << d) >> d;\n            }\n       \
    \ }\n    }\n}\n\nimpl Index<usize> for BitSet {\n    type Output = bool;\n   \
    \ #[inline]\n    fn index(&self, i: usize) -> &bool {\n        out_of_bounds!(self.size,\
    \ i);\n        let x = self.buf[i >> 6];\n        let mask = 1 << (i & 63);\n\
    \        if (x & mask) == 0 {\n            &false\n        } else {\n        \
    \    &true\n        }\n    }\n}\n\nimpl<'a> BitXorAssign<&'a BitSet> for BitSet\
    \ {\n    #[inline]\n    fn bitxor_assign(&mut self, rhs: &'a BitSet) {\n     \
    \   for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {\n            *a ^= *b;\n\
    \        }\n        self.chomp();\n    }\n}\n\nimpl<'a> BitAndAssign<&'a BitSet>\
    \ for BitSet {\n    #[inline]\n    fn bitand_assign(&mut self, rhs: &'a BitSet)\
    \ {\n        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {\n            *a\
    \ &= *b;\n        }\n    }\n}\n\nimpl<'a> BitOrAssign<&'a BitSet> for BitSet {\n\
    \    #[inline]\n    fn bitor_assign(&mut self, rhs: &'a BitSet) {\n        for\
    \ (a, b) in self.buf.iter_mut().zip(&rhs.buf) {\n            *a |= *b;\n     \
    \   }\n        self.chomp();\n    }\n}\n\nmacro_rules! impl_bit_op {\n    ($op:ident,\
    \ $op_assign:ident, $f:ident) => {\n        impl<'a> $op<&'a BitSet> for BitSet\
    \ {\n            type Output = BitSet;\n            #[inline]\n            fn\
    \ $f(mut self, rhs: &'a BitSet) -> Self::Output {\n                self.$op_assign(rhs);\n\
    \                self\n            }\n        }\n        impl<'a, 'b> $op<&'b\
    \ BitSet> for &'a BitSet {\n            type Output = BitSet;\n            #[inline]\n\
    \            fn $f(self, rhs: &'b BitSet) -> Self::Output {\n                let\
    \ mut res = self.clone();\n                res.$op_assign(rhs);\n            \
    \    res\n            }\n        }\n    };\n}\n\nimpl_bit_op!(BitXor, bitxor_assign,\
    \ bitxor);\nimpl_bit_op!(BitAnd, bitand_assign, bitand);\nimpl_bit_op!(BitOr,\
    \ bitor_assign, bitor);\n\nimpl Not for BitSet {\n    type Output = Self;\n  \
    \  #[inline]\n    fn not(mut self) -> Self {\n        for x in &mut self.buf {\n\
    \            *x = !*x;\n        }\n        self.chomp();\n        self\n    }\n\
    }\nimpl<'a> Not for &'a BitSet {\n    type Output = BitSet;\n    #[inline]\n \
    \   fn not(self) -> Self::Output {\n        !self.clone()\n    }\n}\n\nimpl ShlAssign<usize>\
    \ for BitSet {\n    #[inline]\n    fn shl_assign(&mut self, rhs: usize) {\n  \
    \      let q = rhs >> 6;\n        let r = rhs & 63;\n\n        if q >= self.buf.len()\
    \ {\n            for x in &mut self.buf {\n                *x = 0;\n         \
    \   }\n            return;\n        }\n\n        if r == 0 {\n            for\
    \ i in (q..self.buf.len()).rev() {\n                *unsafe { self.buf.get_unchecked_mut(i)\
    \ } =\n                    *unsafe { self.buf.get_unchecked(i - q) };\n      \
    \      }\n        } else {\n            for i in (q + 1..self.buf.len()).rev()\
    \ {\n                *unsafe { self.buf.get_unchecked_mut(i) } =\n           \
    \         (unsafe { self.buf.get_unchecked(i - q) } << r)\n                  \
    \      | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));\n       \
    \     }\n            *unsafe { self.buf.get_unchecked_mut(q) } = unsafe { self.buf.get_unchecked(0)\
    \ } << r;\n        }\n\n        for x in &mut self.buf[..q] {\n            *x\
    \ = 0;\n        }\n\n        self.chomp();\n    }\n}\n\nimpl ShrAssign<usize>\
    \ for BitSet {\n    #[inline]\n    fn shr_assign(&mut self, rhs: usize) {\n  \
    \      let q = rhs >> 6;\n        let r = rhs & 63;\n\n        if q >= self.buf.len()\
    \ {\n            for x in &mut self.buf {\n                *x = 0;\n         \
    \   }\n            return;\n        }\n\n        if r == 0 {\n            for\
    \ i in 0..self.buf.len() - q {\n                *unsafe { self.buf.get_unchecked_mut(i)\
    \ } =\n                    *unsafe { self.buf.get_unchecked(i + q) };\n      \
    \      }\n        } else {\n            for i in 0..self.buf.len() - q - 1 {\n\
    \                *unsafe { self.buf.get_unchecked_mut(i) } =\n               \
    \     (unsafe { self.buf.get_unchecked(i + q) } >> r)\n                      \
    \  | (unsafe { self.buf.get_unchecked(i + q + 1) } << (64 - r));\n           \
    \ }\n            let len = self.buf.len();\n            *unsafe { self.buf.get_unchecked_mut(len\
    \ - q - 1) } =\n                unsafe { self.buf.get_unchecked(len - 1) } >>\
    \ r;\n        }\n\n        let len = self.buf.len();\n        for x in &mut self.buf[len\
    \ - q..] {\n            *x = 0;\n        }\n    }\n}\n\nmacro_rules! impl_shift_op\
    \ {\n    ($op:ident, $op_assign:ident, $f:ident) => {\n        impl $op<usize>\
    \ for BitSet {\n            type Output = BitSet;\n            #[inline]\n   \
    \         fn $f(mut self, rhs: usize) -> Self::Output {\n                self.$op_assign(rhs);\n\
    \                self\n            }\n        }\n        impl<'a> $op<usize> for\
    \ &'a BitSet {\n            type Output = BitSet;\n            #[inline]\n   \
    \         fn $f(self, rhs: usize) -> Self::Output {\n                let mut res\
    \ = self.clone();\n                res.$op_assign(rhs);\n                res\n\
    \            }\n        }\n    };\n}\n\nimpl_shift_op!(Shl, shl_assign, shl);\n\
    impl_shift_op!(Shr, shr_assign, shr);\n\n#[cfg(test)]\nmod test {\n    use super::*;\n\
    \    #[test]\n    fn test_bitset_set_read() {\n        use rand::prelude::*;\n\
    \        let size = 6400;\n        let mut set = BitSet::new(size);\n        let\
    \ mut v = vec![false; size];\n        let mut rng = StdRng::seed_from_u64(114514);\n\
    \n        for i in 0..size {\n            let b = rng.next_u32() % 2 == 0;\n \
    \           set.set(i, b);\n            v[i] = b;\n        }\n\n        for i\
    \ in 0..size {\n            assert_eq!(set[i], v[i]);\n        }\n    }\n\n  \
    \  #[test]\n    fn test_bitset_shl() {\n        let do_test = |size, shift| {\n\
    \            use rand::prelude::*;\n            let mut set = BitSet::new(size);\n\
    \            let mut v = vec![false; size];\n            let mut rng = StdRng::seed_from_u64(114514);\n\
    \n            for i in 0..size {\n                let b = rng.next_u32() % 2 ==\
    \ 0;\n                set.set(i, b);\n                v[i] = b;\n            }\n\
    \            for i in (shift..v.len()).rev() {\n                v[i] = v[i - shift];\n\
    \            }\n            for i in 0..std::cmp::min(size, shift) {\n       \
    \         v[i] = false;\n            }\n\n            set <<= shift;\n       \
    \     for i in 0..size {\n                assert_eq!(set[i], v[i]);\n        \
    \    }\n        };\n\n        do_test(6400, 640);\n        do_test(6400, 114);\n\
    \        do_test(6400, 514);\n        do_test(6400, 6400);\n        do_test(6400,\
    \ 16400);\n\n        let mut t = BitSet::new(310);\n\n        for i in 0..31000\
    \ {\n            t <<= i;\n        }\n    }\n\n    #[test]\n    fn test_bitset_shr()\
    \ {\n        let do_test = |size, shift| {\n            use rand::prelude::*;\n\
    \            let mut set = BitSet::new(size);\n            let mut v = vec![false;\
    \ size];\n            let mut rng = StdRng::seed_from_u64(114514);\n\n       \
    \     for i in 0..size {\n                let b = rng.next_u32() % 2 == 0;\n \
    \               set.set(i, b);\n                v[i] = b;\n            }\n\n \
    \           let s = if size >= shift { size - shift } else { 0 };\n\n        \
    \    for i in 0..s {\n                v[i] = v[i + shift];\n            }\n\n\
    \            for i in s..size {\n                v[i] = false;\n            }\n\
    \n            set >>= shift;\n            for i in 0..size {\n               \
    \ assert_eq!(set[i], v[i]);\n            }\n        };\n\n        do_test(6400,\
    \ 640);\n        do_test(6400, 114);\n        do_test(6400, 514);\n        do_test(63,\
    \ 65);\n        do_test(6400, 6400);\n        do_test(6400, 16400);\n\n      \
    \  let mut t = BitSet::new(310);\n\n        for i in 0..31000 {\n            t\
    \ >>= i;\n        }\n    }\n\n    #[test]\n    fn test_bitset_chomp() {\n    \
    \    let mut set1 = BitSet::new(4);\n        let mut set2 = BitSet::new(8);\n\n\
    \        for i in 0..4 {\n            set1.set(i, true);\n            set2.set(i,\
    \ true);\n        }\n\n        for i in 4..8 {\n            set2.set(i, true);\n\
    \        }\n\n        set1 <<= 2;\n        assert_eq!(set1.count_ones(), 2);\n\
    \        assert_eq!(set1.count_zeros(), 2);\n        assert_eq!((&set1 | &set2).count_ones(),\
    \ 4);\n        assert_eq!((&set1 & &set2).count_ones(), 2);\n        assert_eq!((&set1\
    \ ^ &set2).count_ones(), 2);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/bitset/src/lib.rs
  requiredBy:
  - verify/AtCoder/typical_059/src/main.rs
  - crates/math/bit_matrix/src/lib.rs
  timestamp: '2024-07-10 22:19:14+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/bitset/src/lib.rs
layout: document
redirect_from:
- /library/crates/bitset/src/lib.rs
- /library/crates/bitset/src/lib.rs.html
title: crates/bitset/src/lib.rs
---
