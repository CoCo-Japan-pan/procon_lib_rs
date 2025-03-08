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
    - https://choosealicense.com/licenses/mit/)
    - https://crates.io/crates/bitset-fixed)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5185\u90E8\u3067u64\u306E\u914D\u5217\u3092\u6301\u3063\u3066\u30D3\
    \u30C3\u30C8\u6F14\u7B97\u3092\u307E\u3068\u3081\u3066\u884C\u3044\u300164\u500D\
    \u9AD8\u901F\u5316\u3092\u56F3\u308B  \n//! `Vec<bool>`\u3092\u7528\u3044\u305F\
    \u3044DP\u306A\u3069\u3067\u3001\u30D3\u30C3\u30C8\u6F14\u7B97\u3092\u4F7F\u3044\
    \u305F\u3044\u5834\u5408\u7B49\u3092\u60F3\u5B9A  \n//! \u30B7\u30D5\u30C8\u6F14\
    \u7B97\u306E\u30B3\u30FC\u30C9\u306B\u3064\u3044\u3066\u306F\n//! [bitset-fixed](https://crates.io/crates/bitset-fixed)\
    \ Under [MIT License](https://choosealicense.com/licenses/mit/)\n//! \u3092\u57FA\
    \u306B\u3057\u3066\u3044\u307E\u3059\n\nuse std::ops::{\n    BitAnd, BitAndAssign,\
    \ BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,\n    Shr,\
    \ ShrAssign,\n};\n\n/// bit\u3068\u3057\u3066\u306FLSB -> MSB\u306E\u9806\u306B\
    u64\u306E\u914D\u5217\u3068\u3057\u3066\u4FDD\u6301\u3059\u308B  \n/// \u4ED6\u306E\
    BitSet\u3068\u306E\u6F14\u7B97\u3092\u884C\u3046\u5834\u5408\u306F\u30B5\u30A4\
    \u30BA\u304C\u540C\u3058\u3067\u306A\u3044\u3068\u30D1\u30CB\u30C3\u30AF\u3059\
    \u308B\u3053\u3068\u306B\u3057\u3066\u3044\u308B(bitand\u306E\u4ED5\u69D8\u304C\
    \u975E\u81EA\u660E\u304B\u3082\u3057\u308C\u306A\u3044\u306E\u3067)\n#[derive(Debug,\
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
    \u3063\u3066\u3044\u308Bbit\u30920\u306B\u3059\u308B  \n    /// buffer_mut\u7B49\
    \u3067\u76F4\u63A5\u3044\u3058\u3063\u305F\u5834\u5408\u306B\u8FBB\u8904\u5408\
    \u308F\u305B\u7528\u306B\u4F7F\u3046 \u666E\u6BB5\u306F\u4F7F\u308F\u306A\u304F\
    \u3066\u3088\u3044\n    #[inline]\n    pub fn chomp(&mut self) {\n        let\
    \ r = self.size & 63;\n        if r > 0 {\n            if let Some(last) = self.buf.last_mut()\
    \ {\n                let d = 64 - r;\n                *last = (*last << d) >>\
    \ d;\n            }\n        }\n    }\n\n    /// Faster left shift and or\n  \
    \  ///\n    /// `bitset | (bitset << x)`\n    #[inline]\n    pub fn shl_or(&mut\
    \ self, x: usize) {\n        let q = x >> 6;\n        let r = x & 63;\n      \
    \  let len = self.buf.len();\n\n        if q >= len {\n            return;\n \
    \       }\n\n        if r == 0 {\n            // \u3061\u3087\u3046\u3069-q\u305A\
    \u308C\n            for i in (q..len).rev() {\n                unsafe {\n    \
    \                *self.buf.get_unchecked_mut(i) |= *self.buf.get_unchecked(i -\
    \ q);\n                }\n            }\n        } else {\n            for i in\
    \ (q + 1..len).rev() {\n                // -q\u305A\u308C\u306E\u4E0B\u4F4D64-rbit\
    \ \u3068 -(q+1)\u305A\u308C\u306E\u4E0A\u4F4Drbit \u306E\u9023\u7D50\n       \
    \         unsafe {\n                    *self.buf.get_unchecked_mut(i) |= (*self.buf.get_unchecked(i\
    \ - q) << r)\n                        | (*self.buf.get_unchecked(i - q - 1) >>\
    \ (64 - r));\n                }\n            }\n            // \u4F59\u308A\u306E\
    \u4E0B\u4F4D64-rbit\n            unsafe {\n                *self.buf.get_unchecked_mut(q)\
    \ |= *self.buf.get_unchecked(0) << r;\n            }\n        }\n\n        self.chomp();\n\
    \    }\n\n    /// Faster right shift and or\n    ///\n    /// `bitset | (bitset\
    \ >> x)`\n    #[inline]\n    pub fn shr_or(&mut self, x: usize) {\n        let\
    \ q = x >> 6;\n        let r = x & 63;\n        let len = self.buf.len();\n\n\
    \        if q >= len {\n            return;\n        }\n\n        if r == 0 {\n\
    \            for i in 0..len - q {\n                // \u3061\u3087\u3046\u3069\
    +q\u305A\u308C\n                unsafe {\n                    *self.buf.get_unchecked_mut(i)\
    \ |= *self.buf.get_unchecked(i + q);\n                }\n            }\n     \
    \   } else {\n            for i in 0..len - q - 1 {\n                unsafe {\n\
    \                    // +q\u305A\u308C\u306E\u4E0A\u4F4D64-rbit \u3068 +(q+1)\u305A\
    \u308C\u306E\u4E0B\u4F4Drbit \u306E\u9023\u7D50\n                    *self.buf.get_unchecked_mut(i)\
    \ |= (*self.buf.get_unchecked(i + q) >> r)\n                        | (*self.buf.get_unchecked(i\
    \ + q + 1) << (64 - r));\n                }\n            }\n            // \u4F59\
    \u308A\u306E\u4E0A\u4F4D64-rbit\n            unsafe {\n                *self.buf.get_unchecked_mut(len\
    \ - q - 1) |= *self.buf.get_unchecked(len - 1) >> r;\n            }\n        }\n\
    \n        self.chomp();\n    }\n}\n\nimpl Index<usize> for BitSet {\n    type\
    \ Output = bool;\n    #[inline]\n    fn index(&self, i: usize) -> &bool {\n  \
    \      out_of_bounds!(self.size, i);\n        let x = self.buf[i >> 6];\n    \
    \    let mask = 1 << (i & 63);\n        if (x & mask) == 0 {\n            &false\n\
    \        } else {\n            &true\n        }\n    }\n}\n\nimpl<'a> BitXorAssign<&'a\
    \ BitSet> for BitSet {\n    #[inline]\n    fn bitxor_assign(&mut self, rhs: &'a\
    \ BitSet) {\n        assert_eq!(self.size, rhs.size);\n        for (a, b) in self.buf.iter_mut().zip(&rhs.buf)\
    \ {\n            *a ^= *b;\n        }\n        self.chomp();\n    }\n}\n\nimpl<'a>\
    \ BitAndAssign<&'a BitSet> for BitSet {\n    #[inline]\n    fn bitand_assign(&mut\
    \ self, rhs: &'a BitSet) {\n        assert_eq!(self.size, rhs.size);\n       \
    \ for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {\n            *a &= *b;\n \
    \       }\n    }\n}\n\nimpl<'a> BitOrAssign<&'a BitSet> for BitSet {\n    #[inline]\n\
    \    fn bitor_assign(&mut self, rhs: &'a BitSet) {\n        assert_eq!(self.size,\
    \ rhs.size);\n        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {\n    \
    \        *a |= *b;\n        }\n        self.chomp();\n    }\n}\n\nmacro_rules!\
    \ impl_bit_op {\n    ($op:ident, $op_assign:ident, $f:ident) => {\n        impl<'a>\
    \ $op<&'a BitSet> for BitSet {\n            type Output = BitSet;\n          \
    \  #[inline]\n            fn $f(mut self, rhs: &'a BitSet) -> Self::Output {\n\
    \                self.$op_assign(rhs);\n                self\n            }\n\
    \        }\n        impl<'a, 'b> $op<&'b BitSet> for &'a BitSet {\n          \
    \  type Output = BitSet;\n            #[inline]\n            fn $f(self, rhs:\
    \ &'b BitSet) -> Self::Output {\n                let mut res = self.clone();\n\
    \                res.$op_assign(rhs);\n                res\n            }\n  \
    \      }\n    };\n}\n\nimpl_bit_op!(BitXor, bitxor_assign, bitxor);\nimpl_bit_op!(BitAnd,\
    \ bitand_assign, bitand);\nimpl_bit_op!(BitOr, bitor_assign, bitor);\n\nimpl Not\
    \ for BitSet {\n    type Output = Self;\n    #[inline]\n    fn not(mut self) ->\
    \ Self {\n        for x in &mut self.buf {\n            *x = !*x;\n        }\n\
    \        self.chomp();\n        self\n    }\n}\nimpl<'a> Not for &'a BitSet {\n\
    \    type Output = BitSet;\n    #[inline]\n    fn not(self) -> Self::Output {\n\
    \        !self.clone()\n    }\n}\n\nimpl ShlAssign<usize> for BitSet {\n    #[inline]\n\
    \    fn shl_assign(&mut self, rhs: usize) {\n        let q = rhs >> 6;\n     \
    \   let r = rhs & 63;\n        let len = self.buf.len();\n\n        if q >= len\
    \ {\n            for x in &mut self.buf {\n                *x = 0;\n         \
    \   }\n            return;\n        }\n\n        if r == 0 {\n            for\
    \ i in (q..len).rev() {\n                unsafe {\n                    *self.buf.get_unchecked_mut(i)\
    \ = *self.buf.get_unchecked(i - q);\n                }\n            }\n      \
    \  } else {\n            for i in (q + 1..len).rev() {\n                unsafe\
    \ {\n                    *self.buf.get_unchecked_mut(i) = (*self.buf.get_unchecked(i\
    \ - q) << r)\n                        | (*self.buf.get_unchecked(i - q - 1) >>\
    \ (64 - r));\n                }\n            }\n            unsafe {\n       \
    \         *self.buf.get_unchecked_mut(q) = *self.buf.get_unchecked(0) << r;\n\
    \            }\n        }\n\n        for x in &mut self.buf[..q] {\n         \
    \   *x = 0;\n        }\n\n        self.chomp();\n    }\n}\n\nimpl ShrAssign<usize>\
    \ for BitSet {\n    #[inline]\n    fn shr_assign(&mut self, rhs: usize) {\n  \
    \      let q = rhs >> 6;\n        let r = rhs & 63;\n        let len = self.buf.len();\n\
    \n        if q >= len {\n            for x in &mut self.buf {\n              \
    \  *x = 0;\n            }\n            return;\n        }\n\n        if r == 0\
    \ {\n            for i in 0..len - q {\n                unsafe {\n           \
    \         *self.buf.get_unchecked_mut(i) = *self.buf.get_unchecked(i + q);\n \
    \               }\n            }\n        } else {\n            for i in 0..len\
    \ - q - 1 {\n                unsafe {\n                    *self.buf.get_unchecked_mut(i)\
    \ = (*self.buf.get_unchecked(i + q) >> r)\n                        | (*self.buf.get_unchecked(i\
    \ + q + 1) << (64 - r));\n                }\n            }\n            unsafe\
    \ {\n                *self.buf.get_unchecked_mut(len - q - 1) = *self.buf.get_unchecked(len\
    \ - 1) >> r;\n            }\n        }\n\n        for x in &mut self.buf[len -\
    \ q..] {\n            *x = 0;\n        }\n    }\n}\n\nmacro_rules! impl_shift_op\
    \ {\n    ($op:ident, $op_assign:ident, $f:ident) => {\n        impl $op<usize>\
    \ for BitSet {\n            type Output = BitSet;\n            #[inline]\n   \
    \         fn $f(mut self, rhs: usize) -> Self::Output {\n                self.$op_assign(rhs);\n\
    \                self\n            }\n        }\n        impl<'a> $op<usize> for\
    \ &'a BitSet {\n            type Output = BitSet;\n            #[inline]\n   \
    \         fn $f(self, rhs: usize) -> Self::Output {\n                let mut res\
    \ = self.clone();\n                res.$op_assign(rhs);\n                res\n\
    \            }\n        }\n    };\n}\n\nimpl_shift_op!(Shl, shl_assign, shl);\n\
    impl_shift_op!(Shr, shr_assign, shr);\n\n#[cfg(test)]\nmod test {\n    use super::*;\n\
    \    use rand::prelude::*;\n\n    #[test]\n    fn test_shl_or() {\n        let\
    \ do_test = |size, shift| {\n            let mut set = BitSet::new(size);\n  \
    \          let mut v = vec![false; size];\n            let mut rng = thread_rng();\n\
    \            for i in 0..size {\n                if rng.gen() {\n            \
    \        set.set(i, true);\n                    v[i] = true;\n               \
    \ }\n            }\n            // shl_or\n            let mut shifted = vec![false;\
    \ size];\n            for i in 0..size {\n                if i >= shift {\n  \
    \                  shifted[i] = v[i - shift];\n                }\n           \
    \ }\n            for i in 0..size {\n                shifted[i] |= v[i];\n   \
    \         }\n            set.shl_or(shift);\n            for i in 0..size {\n\
    \                assert_eq!(set[i], shifted[i]);\n            }\n        };\n\n\
    \        let mut rng = thread_rng();\n        for _ in 0..100 {\n            let\
    \ size = rng.gen_range(0..=10000);\n            let shift = rng.gen_range(0..=10000);\n\
    \            do_test(size, shift);\n        }\n    }\n\n    #[test]\n    fn test_shr_or()\
    \ {\n        use rand::prelude::*;\n        let do_test = |size, shift| {\n  \
    \          let mut set = BitSet::new(size);\n            let mut v = vec![false;\
    \ size];\n            let mut rng = thread_rng();\n            for i in 0..size\
    \ {\n                if rng.gen() {\n                    set.set(i, true);\n \
    \                   v[i] = true;\n                }\n            }\n         \
    \   // shr_or\n            let mut shifted = vec![false; size];\n            for\
    \ i in 0..size {\n                if i + shift < size {\n                    shifted[i]\
    \ = v[i + shift];\n                }\n            }\n            for i in 0..size\
    \ {\n                shifted[i] |= v[i];\n            }\n            set.shr_or(shift);\n\
    \            for i in 0..size {\n                assert_eq!(set[i], shifted[i]);\n\
    \            }\n        };\n        let mut rng = thread_rng();\n        for _\
    \ in 0..100 {\n            let size = rng.gen_range(0..=10000);\n            let\
    \ shift = rng.gen_range(0..=10000);\n            do_test(size, shift);\n     \
    \   }\n    }\n\n    #[test]\n    fn test_bitset_set_read() {\n        use rand::prelude::*;\n\
    \        let do_test = |size| {\n            let mut set = BitSet::new(size);\n\
    \            let mut v = vec![false; size];\n            let mut rng = thread_rng();\n\
    \            for i in 0..size {\n                let b = rng.gen();\n        \
    \        set.set(i, b);\n                v[i] = b;\n            }\n          \
    \  for i in 0..size {\n                assert_eq!(set.get(i), v[i]);\n       \
    \     }\n        };\n        let mut rng = thread_rng();\n        for _ in 0..100\
    \ {\n            let size = rng.gen_range(0..=10000);\n            do_test(size);\n\
    \        }\n    }\n\n    #[test]\n    fn test_bitset_shl() {\n        use rand::prelude::*;\n\
    \        let do_test = |size, shift| {\n            let mut set = BitSet::new(size);\n\
    \            let mut v = vec![false; size];\n            let mut rng = thread_rng();\n\
    \            for i in 0..size {\n                let b = rng.gen();\n        \
    \        set.set(i, b);\n                v[i] = b;\n            }\n          \
    \  // shl\n            let mut shifted = vec![false; size];\n            for i\
    \ in 0..size {\n                if i >= shift {\n                    shifted[i]\
    \ = v[i - shift];\n                }\n            }\n            set <<= shift;\n\
    \            for i in 0..size {\n                assert_eq!(set[i], shifted[i]);\n\
    \            }\n        };\n\n        let mut rng = thread_rng();\n        for\
    \ _ in 0..100 {\n            let size = rng.gen_range(0..=10000);\n          \
    \  let shift = rng.gen_range(0..=10000);\n            do_test(size, shift);\n\
    \        }\n\n        let mut many_shift = BitSet::new(1000);\n        for _ in\
    \ 0..1000 {\n            let shift = rng.gen_range(0..=1000);\n            many_shift\
    \ <<= shift;\n        }\n    }\n\n    #[test]\n    fn test_bitset_shr() {\n  \
    \      use rand::prelude::*;\n        let do_test = |size, shift| {\n        \
    \    let mut set = BitSet::new(size);\n            let mut v = vec![false; size];\n\
    \            let mut rng = thread_rng();\n            for i in 0..size {\n   \
    \             let b = rng.gen();\n                set.set(i, b);\n           \
    \     v[i] = b;\n            }\n            // shr\n            let mut shifted\
    \ = vec![false; size];\n            for i in 0..size {\n                if i +\
    \ shift < size {\n                    shifted[i] = v[i + shift];\n           \
    \     }\n            }\n            set >>= shift;\n            for i in 0..size\
    \ {\n                assert_eq!(set[i], shifted[i]);\n            }\n        };\n\
    \n        let mut rng = thread_rng();\n        for _ in 0..100 {\n           \
    \ let size = rng.gen_range(0..=10000);\n            let shift = rng.gen_range(0..=10000);\n\
    \            do_test(size, shift);\n        }\n\n        let mut many_shift =\
    \ BitSet::new(1000);\n        for _ in 0..1000 {\n            let shift = rng.gen_range(0..=1000);\n\
    \            many_shift >>= shift;\n        }\n    }\n\n    #[test]\n    fn test_bit_or_and_xor()\
    \ {\n        use rand::prelude::*;\n        let do_test = |size| {\n         \
    \   let mut set1 = BitSet::new(size);\n            let mut set2 = BitSet::new(size);\n\
    \            let mut v1 = vec![false; size];\n            let mut v2 = vec![false;\
    \ size];\n            let mut rng = thread_rng();\n            for i in 0..size\
    \ {\n                let b = rng.gen();\n                set1.set(i, b);\n   \
    \             v1[i] = b;\n                set2.set(i, b);\n                v2[i]\
    \ = b;\n            }\n            // or\n            let mut or = vec![false;\
    \ size];\n            for i in 0..size {\n                or[i] = v1[i] | v2[i];\n\
    \            }\n            let res = &set1 | &set2;\n            for i in 0..size\
    \ {\n                assert_eq!(res[i], or[i]);\n            }\n\n           \
    \ // and\n            let mut and = vec![false; size];\n            for i in 0..size\
    \ {\n                and[i] = v1[i] & v2[i];\n            }\n            let res\
    \ = &set1 & &set2;\n            for i in 0..size {\n                assert_eq!(res[i],\
    \ and[i]);\n            }\n\n            // xor\n            let mut xor = vec![false;\
    \ size];\n            for i in 0..size {\n                xor[i] = v1[i] ^ v2[i];\n\
    \            }\n            let res = &set1 ^ &set2;\n            for i in 0..size\
    \ {\n                assert_eq!(res[i], xor[i]);\n            }\n        };\n\
    \        let mut rng = thread_rng();\n        for _ in 0..100 {\n            let\
    \ size = rng.gen_range(0..=10000);\n            do_test(size);\n        }\n  \
    \  }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/bitset/src/lib.rs
  requiredBy:
  - verify/AtCoder/typical_059/src/main.rs
  - crates/math/bit_matrix/src/lib.rs
  timestamp: '2024-10-20 18:08:55+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/bitset/src/lib.rs
layout: document
redirect_from:
- /library/crates/bitset/src/lib.rs
- /library/crates/bitset/src/lib.rs.html
title: crates/bitset/src/lib.rs
---
