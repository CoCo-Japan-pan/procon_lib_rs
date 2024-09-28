//! 内部でu64の配列を持ってビット演算をまとめて行い、64倍高速化を図る  
//! Vec<bool>を用いたいDPなどで、ビット演算を使いたい場合等を想定  
//! シフト演算やテストのコードについては
//! [bitset-fixed](https://github.com/hatoo/bitset-fixed) Under [MIT License](https://opensource.org/license/mit)
//! を使用させていただいています

use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitSet {
    buf: Vec<u64>,
    size: usize,
}

macro_rules! out_of_bounds {
    ($size:expr, $i:expr) => {
        assert!(
            $i < $size,
            "index out of bounds: the len is {} but the index is {}",
            $size,
            $i
        );
    };
}

impl From<Vec<bool>> for BitSet {
    fn from(v: Vec<bool>) -> Self {
        let size = v.len();
        let mut buf = vec![0; (size + 63) / 64];
        for i in 0..size {
            if v[i] {
                buf[i >> 6] |= 1 << (i & 63);
            }
        }
        Self { buf, size }
    }
}

impl<const N: usize> From<[bool; N]> for BitSet {
    fn from(v: [bool; N]) -> Self {
        let size = N;
        let mut buf = vec![0; (size + 63) / 64];
        for i in 0..size {
            if v[i] {
                buf[i >> 6] |= 1 << (i & 63);
            }
        }
        Self { buf, size }
    }
}

impl BitSet {
    /// size個のbitを持つBitSetを生成する(どれもunset)
    pub fn new(size: usize) -> Self {
        let len = (size + 63) / 64;
        Self {
            buf: vec![0; len],
            size,
        }
    }

    #[inline]
    pub fn buffer(&self) -> &[u64] {
        &self.buf
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut [u64] {
        &mut self.buf
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    /// indexでアクセスしてもよい
    pub fn get(&self, i: usize) -> bool {
        out_of_bounds!(self.size, i);
        let x = self.buf[i >> 6];
        let mask = 1 << (i & 63);
        (x & mask) != 0
    }

    /// i番目のbitをbに設定する
    #[inline]
    pub fn set(&mut self, i: usize, b: bool) {
        out_of_bounds!(self.size, i);
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }

    /// i番目のbitを反転させる
    #[inline]
    pub fn flip(&mut self, i: usize) {
        out_of_bounds!(self.size, i);
        self.buf[i >> 6] ^= 1 << (i & 63);
    }

    /// 1の数を返す
    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.buf.iter().map(|&x| x.count_ones()).sum()
    }

    /// 0の数を返す
    #[inline]
    pub fn count_zeros(&self) -> u32 {
        self.size as u32 - self.count_ones()
    }

    /// 全て0かどうかを返す
    #[inline]
    pub fn none(&self) -> bool {
        self.count_ones() == 0
    }

    /// 全て1かどうかを返す
    #[inline]
    pub fn all(&self) -> bool {
        self.count_ones() == self.size as u32
    }

    /// どれか1つでも1があるかどうかを返す
    #[inline]
    pub fn any(&self) -> bool {
        self.count_ones() > 0
    }

    /// 範囲外だが余分に持っているbitを0にする
    #[inline]
    pub fn chomp(&mut self) {
        let r = self.size & 63;
        if r > 0 {
            if let Some(last) = self.buf.last_mut() {
                let d = 64 - r;
                *last = (*last << d) >> d;
            }
        }
    }

    /// Faster left shift and or
    ///
    /// `bitset | (bitset << x)`
    #[inline]
    pub fn shl_or(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;
        let len = self.buf.len();

        if q >= len {
            return;
        }

        if r == 0 {
            for i in (q..len).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } |=
                    *unsafe { self.buf.get_unchecked(i - q) };
            }
        } else {
            for i in (q + 1..len).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } |=
                    (unsafe { self.buf.get_unchecked(i - q) } << r)
                        | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));
            }
            *unsafe { self.buf.get_unchecked_mut(q) } |= unsafe { self.buf.get_unchecked(0) } << r;
        }

        self.chomp();
    }

    /// Faster right shift and or
    ///
    /// `bitset | (bitset >> x)`
    #[inline]
    pub fn shr_or(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;
        let len = self.buf.len();

        if q >= len {
            return;
        }

        if r == 0 {
            for i in 0..len - q {
                *unsafe { self.buf.get_unchecked_mut(i) } |=
                    *unsafe { self.buf.get_unchecked(i + q) };
            }
        } else {
            for i in 0..len - q - 1 {
                *unsafe { self.buf.get_unchecked_mut(i) } |=
                    (unsafe { self.buf.get_unchecked(i + q) } >> r)
                        | (unsafe { self.buf.get_unchecked(i + q + 1) } << (64 - r));
            }
            *unsafe { self.buf.get_unchecked_mut(len - q - 1) } |=
                unsafe { self.buf.get_unchecked(len - 1) } >> r;
        }

        self.chomp();
    }
}

impl Index<usize> for BitSet {
    type Output = bool;
    #[inline]
    fn index(&self, i: usize) -> &bool {
        out_of_bounds!(self.size, i);
        let x = self.buf[i >> 6];
        let mask = 1 << (i & 63);
        if (x & mask) == 0 {
            &false
        } else {
            &true
        }
    }
}

impl<'a> BitXorAssign<&'a BitSet> for BitSet {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &'a BitSet) {
        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {
            *a ^= *b;
        }
        self.chomp();
    }
}

impl<'a> BitAndAssign<&'a BitSet> for BitSet {
    #[inline]
    fn bitand_assign(&mut self, rhs: &'a BitSet) {
        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {
            *a &= *b;
        }
    }
}

impl<'a> BitOrAssign<&'a BitSet> for BitSet {
    #[inline]
    fn bitor_assign(&mut self, rhs: &'a BitSet) {
        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {
            *a |= *b;
        }
        self.chomp();
    }
}

macro_rules! impl_bit_op {
    ($op:ident, $op_assign:ident, $f:ident) => {
        impl<'a> $op<&'a BitSet> for BitSet {
            type Output = BitSet;
            #[inline]
            fn $f(mut self, rhs: &'a BitSet) -> Self::Output {
                self.$op_assign(rhs);
                self
            }
        }
        impl<'a, 'b> $op<&'b BitSet> for &'a BitSet {
            type Output = BitSet;
            #[inline]
            fn $f(self, rhs: &'b BitSet) -> Self::Output {
                let mut res = self.clone();
                res.$op_assign(rhs);
                res
            }
        }
    };
}

impl_bit_op!(BitXor, bitxor_assign, bitxor);
impl_bit_op!(BitAnd, bitand_assign, bitand);
impl_bit_op!(BitOr, bitor_assign, bitor);

impl Not for BitSet {
    type Output = Self;
    #[inline]
    fn not(mut self) -> Self {
        for x in &mut self.buf {
            *x = !*x;
        }
        self.chomp();
        self
    }
}
impl<'a> Not for &'a BitSet {
    type Output = BitSet;
    #[inline]
    fn not(self) -> Self::Output {
        !self.clone()
    }
}

impl ShlAssign<usize> for BitSet {
    #[inline]
    fn shl_assign(&mut self, rhs: usize) {
        let q = rhs >> 6;
        let r = rhs & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in (q..self.buf.len()).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    *unsafe { self.buf.get_unchecked(i - q) };
            }
        } else {
            for i in (q + 1..self.buf.len()).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    (unsafe { self.buf.get_unchecked(i - q) } << r)
                        | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));
            }
            *unsafe { self.buf.get_unchecked_mut(q) } = unsafe { self.buf.get_unchecked(0) } << r;
        }

        for x in &mut self.buf[..q] {
            *x = 0;
        }

        self.chomp();
    }
}

impl ShrAssign<usize> for BitSet {
    #[inline]
    fn shr_assign(&mut self, rhs: usize) {
        let q = rhs >> 6;
        let r = rhs & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in 0..self.buf.len() - q {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    *unsafe { self.buf.get_unchecked(i + q) };
            }
        } else {
            for i in 0..self.buf.len() - q - 1 {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    (unsafe { self.buf.get_unchecked(i + q) } >> r)
                        | (unsafe { self.buf.get_unchecked(i + q + 1) } << (64 - r));
            }
            let len = self.buf.len();
            *unsafe { self.buf.get_unchecked_mut(len - q - 1) } =
                unsafe { self.buf.get_unchecked(len - 1) } >> r;
        }

        let len = self.buf.len();
        for x in &mut self.buf[len - q..] {
            *x = 0;
        }
    }
}

macro_rules! impl_shift_op {
    ($op:ident, $op_assign:ident, $f:ident) => {
        impl $op<usize> for BitSet {
            type Output = BitSet;
            #[inline]
            fn $f(mut self, rhs: usize) -> Self::Output {
                self.$op_assign(rhs);
                self
            }
        }
        impl<'a> $op<usize> for &'a BitSet {
            type Output = BitSet;
            #[inline]
            fn $f(self, rhs: usize) -> Self::Output {
                let mut res = self.clone();
                res.$op_assign(rhs);
                res
            }
        }
    };
}

impl_shift_op!(Shl, shl_assign, shl);
impl_shift_op!(Shr, shr_assign, shr);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shl_or() {
        let do_test = |size, shift| {
            use rand::prelude::*;
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();

            for i in 0..size {
                let b = rng.next_u32() % 2 == 0;
                set.set(i, b);
                v[i] = b;
            }
            for i in (shift..v.len()).rev() {
                v[i] = v[i - shift];
            }
            for i in 0..std::cmp::min(size, shift) {
                v[i] = false;
            }
            for i in 0..size {
                v[i] |= set[i];
            }

            set.shl_or(shift);
            for i in 0..size {
                assert_eq!(set[i], v[i]);
            }
        };

        do_test(6400, 640);
        do_test(6400, 114);
        do_test(6400, 514);
        do_test(6400, 6400);
        do_test(6400, 16400);
    }

    #[test]
    fn test_shr_or() {
        let do_test = |size, shift| {
            use rand::prelude::*;
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();

            for i in 0..size {
                let b = rng.next_u32() % 2 == 0;
                set.set(i, b);
                v[i] = b;
            }

            let s = if size >= shift { size - shift } else { 0 };

            for i in 0..s {
                v[i] |= v[i + shift];
            }

            for i in s..size {
                v[i] = false;
            }

            for i in 0..size {
                v[i] |= set[i];
            }

            set.shr_or(shift);
            for i in 0..size {
                assert_eq!(set[i], v[i]);
            }
        };

        do_test(6400, 640);
        do_test(6400, 114);
        do_test(6400, 514);
        do_test(63, 65);
        do_test(6400, 6400);
        do_test(6400, 16400);
    }

    #[test]
    fn test_bitset_set_read() {
        use rand::prelude::*;
        let size = 6400;
        let mut set = BitSet::new(size);
        let mut v = vec![false; size];
        let mut rng = thread_rng();

        for i in 0..size {
            let b = rng.next_u32() % 2 == 0;
            set.set(i, b);
            v[i] = b;
        }

        for i in 0..size {
            assert_eq!(set[i], v[i]);
        }
    }

    #[test]
    fn test_bitset_shl() {
        let do_test = |size, shift| {
            use rand::prelude::*;
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();

            for i in 0..size {
                let b = rng.next_u32() % 2 == 0;
                set.set(i, b);
                v[i] = b;
            }
            for i in (shift..v.len()).rev() {
                v[i] = v[i - shift];
            }
            for i in 0..std::cmp::min(size, shift) {
                v[i] = false;
            }

            set <<= shift;
            for i in 0..size {
                assert_eq!(set[i], v[i]);
            }
        };

        do_test(6400, 640);
        do_test(6400, 114);
        do_test(6400, 514);
        do_test(6400, 6400);
        do_test(6400, 16400);

        let mut t = BitSet::new(310);

        for i in 0..31000 {
            t <<= i;
        }
    }

    #[test]
    fn test_bitset_shr() {
        let do_test = |size, shift| {
            use rand::prelude::*;
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();

            for i in 0..size {
                let b = rng.next_u32() % 2 == 0;
                set.set(i, b);
                v[i] = b;
            }

            let s = if size >= shift { size - shift } else { 0 };

            for i in 0..s {
                v[i] = v[i + shift];
            }

            for i in s..size {
                v[i] = false;
            }

            set >>= shift;
            for i in 0..size {
                assert_eq!(set[i], v[i]);
            }
        };

        do_test(6400, 640);
        do_test(6400, 114);
        do_test(6400, 514);
        do_test(63, 65);
        do_test(6400, 6400);
        do_test(6400, 16400);

        let mut t = BitSet::new(310);

        for i in 0..31000 {
            t >>= i;
        }
    }

    #[test]
    fn test_bitset_chomp() {
        let mut set1 = BitSet::new(4);
        let mut set2 = BitSet::new(8);

        for i in 0..4 {
            set1.set(i, true);
            set2.set(i, true);
        }

        for i in 4..8 {
            set2.set(i, true);
        }

        set1 <<= 2;
        assert_eq!(set1.count_ones(), 2);
        assert_eq!(set1.count_zeros(), 2);
        assert_eq!((&set1 | &set2).count_ones(), 4);
        assert_eq!((&set1 & &set2).count_ones(), 2);
        assert_eq!((&set1 ^ &set2).count_ones(), 2);
    }
}
