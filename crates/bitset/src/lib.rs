//! 内部でu64の配列を持ってビット演算をまとめて行い、64倍高速化を図る  
//! `Vec<bool>`を用いたいDPなどで、ビット演算を使いたい場合等を想定  
//! シフト演算のコードについては
//! [bitset-fixed](https://crates.io/crates/bitset-fixed) Under [MIT License](https://choosealicense.com/licenses/mit/)
//! を基にしています

use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign,
};

/// bitとしてはLSB -> MSBの順にu64の配列として保持する  
/// 他のBitSetとの演算を行う場合はサイズが同じでないとパニックすることにしている(bitandの仕様が非自明かもしれないので)
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
    /// buffer_mut等で直接いじった場合に辻褄合わせ用に使う 普段は使わなくてよい
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
            // ちょうど-qずれ
            for i in (q..len).rev() {
                unsafe {
                    *self.buf.get_unchecked_mut(i) |= *self.buf.get_unchecked(i - q);
                }
            }
        } else {
            for i in (q + 1..len).rev() {
                // -qずれの下位64-rbit と -(q+1)ずれの上位rbit の連結
                unsafe {
                    *self.buf.get_unchecked_mut(i) |= (*self.buf.get_unchecked(i - q) << r)
                        | (*self.buf.get_unchecked(i - q - 1) >> (64 - r));
                }
            }
            // 余りの下位64-rbit
            unsafe {
                *self.buf.get_unchecked_mut(q) |= *self.buf.get_unchecked(0) << r;
            }
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
                // ちょうど+qずれ
                unsafe {
                    *self.buf.get_unchecked_mut(i) |= *self.buf.get_unchecked(i + q);
                }
            }
        } else {
            for i in 0..len - q - 1 {
                unsafe {
                    // +qずれの上位64-rbit と +(q+1)ずれの下位rbit の連結
                    *self.buf.get_unchecked_mut(i) |= (*self.buf.get_unchecked(i + q) >> r)
                        | (*self.buf.get_unchecked(i + q + 1) << (64 - r));
                }
            }
            // 余りの上位64-rbit
            unsafe {
                *self.buf.get_unchecked_mut(len - q - 1) |= *self.buf.get_unchecked(len - 1) >> r;
            }
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
        assert_eq!(self.size, rhs.size);
        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {
            *a ^= *b;
        }
        self.chomp();
    }
}

impl<'a> BitAndAssign<&'a BitSet> for BitSet {
    #[inline]
    fn bitand_assign(&mut self, rhs: &'a BitSet) {
        assert_eq!(self.size, rhs.size);
        for (a, b) in self.buf.iter_mut().zip(&rhs.buf) {
            *a &= *b;
        }
    }
}

impl<'a> BitOrAssign<&'a BitSet> for BitSet {
    #[inline]
    fn bitor_assign(&mut self, rhs: &'a BitSet) {
        assert_eq!(self.size, rhs.size);
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
        let len = self.buf.len();

        if q >= len {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in (q..len).rev() {
                unsafe {
                    *self.buf.get_unchecked_mut(i) = *self.buf.get_unchecked(i - q);
                }
            }
        } else {
            for i in (q + 1..len).rev() {
                unsafe {
                    *self.buf.get_unchecked_mut(i) = (*self.buf.get_unchecked(i - q) << r)
                        | (*self.buf.get_unchecked(i - q - 1) >> (64 - r));
                }
            }
            unsafe {
                *self.buf.get_unchecked_mut(q) = *self.buf.get_unchecked(0) << r;
            }
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
        let len = self.buf.len();

        if q >= len {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in 0..len - q {
                unsafe {
                    *self.buf.get_unchecked_mut(i) = *self.buf.get_unchecked(i + q);
                }
            }
        } else {
            for i in 0..len - q - 1 {
                unsafe {
                    *self.buf.get_unchecked_mut(i) = (*self.buf.get_unchecked(i + q) >> r)
                        | (*self.buf.get_unchecked(i + q + 1) << (64 - r));
                }
            }
            unsafe {
                *self.buf.get_unchecked_mut(len - q - 1) = *self.buf.get_unchecked(len - 1) >> r;
            }
        }

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
    use rand::prelude::*;

    #[test]
    fn test_shl_or() {
        let do_test = |size, shift| {
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                if rng.gen() {
                    set.set(i, true);
                    v[i] = true;
                }
            }
            // shl_or
            let mut shifted = vec![false; size];
            for i in 0..size {
                if i >= shift {
                    shifted[i] = v[i - shift];
                }
            }
            for i in 0..size {
                shifted[i] |= v[i];
            }
            set.shl_or(shift);
            for i in 0..size {
                assert_eq!(set[i], shifted[i]);
            }
        };

        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            let shift = rng.gen_range(0..=10000);
            do_test(size, shift);
        }
    }

    #[test]
    fn test_shr_or() {
        use rand::prelude::*;
        let do_test = |size, shift| {
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                if rng.gen() {
                    set.set(i, true);
                    v[i] = true;
                }
            }
            // shr_or
            let mut shifted = vec![false; size];
            for i in 0..size {
                if i + shift < size {
                    shifted[i] = v[i + shift];
                }
            }
            for i in 0..size {
                shifted[i] |= v[i];
            }
            set.shr_or(shift);
            for i in 0..size {
                assert_eq!(set[i], shifted[i]);
            }
        };
        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            let shift = rng.gen_range(0..=10000);
            do_test(size, shift);
        }
    }

    #[test]
    fn test_bitset_set_read() {
        use rand::prelude::*;
        let do_test = |size| {
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                let b = rng.gen();
                set.set(i, b);
                v[i] = b;
            }
            for i in 0..size {
                assert_eq!(set.get(i), v[i]);
            }
        };
        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            do_test(size);
        }
    }

    #[test]
    fn test_bitset_shl() {
        use rand::prelude::*;
        let do_test = |size, shift| {
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                let b = rng.gen();
                set.set(i, b);
                v[i] = b;
            }
            // shl
            let mut shifted = vec![false; size];
            for i in 0..size {
                if i >= shift {
                    shifted[i] = v[i - shift];
                }
            }
            set <<= shift;
            for i in 0..size {
                assert_eq!(set[i], shifted[i]);
            }
        };

        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            let shift = rng.gen_range(0..=10000);
            do_test(size, shift);
        }

        let mut many_shift = BitSet::new(1000);
        for _ in 0..1000 {
            let shift = rng.gen_range(0..=1000);
            many_shift <<= shift;
        }
    }

    #[test]
    fn test_bitset_shr() {
        use rand::prelude::*;
        let do_test = |size, shift| {
            let mut set = BitSet::new(size);
            let mut v = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                let b = rng.gen();
                set.set(i, b);
                v[i] = b;
            }
            // shr
            let mut shifted = vec![false; size];
            for i in 0..size {
                if i + shift < size {
                    shifted[i] = v[i + shift];
                }
            }
            set >>= shift;
            for i in 0..size {
                assert_eq!(set[i], shifted[i]);
            }
        };

        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            let shift = rng.gen_range(0..=10000);
            do_test(size, shift);
        }

        let mut many_shift = BitSet::new(1000);
        for _ in 0..1000 {
            let shift = rng.gen_range(0..=1000);
            many_shift >>= shift;
        }
    }

    #[test]
    fn test_bit_or_and_xor() {
        use rand::prelude::*;
        let do_test = |size| {
            let mut set1 = BitSet::new(size);
            let mut set2 = BitSet::new(size);
            let mut v1 = vec![false; size];
            let mut v2 = vec![false; size];
            let mut rng = thread_rng();
            for i in 0..size {
                let b = rng.gen();
                set1.set(i, b);
                v1[i] = b;
                set2.set(i, b);
                v2[i] = b;
            }
            // or
            let mut or = vec![false; size];
            for i in 0..size {
                or[i] = v1[i] | v2[i];
            }
            let res = &set1 | &set2;
            for i in 0..size {
                assert_eq!(res[i], or[i]);
            }

            // and
            let mut and = vec![false; size];
            for i in 0..size {
                and[i] = v1[i] & v2[i];
            }
            let res = &set1 & &set2;
            for i in 0..size {
                assert_eq!(res[i], and[i]);
            }

            // xor
            let mut xor = vec![false; size];
            for i in 0..size {
                xor[i] = v1[i] ^ v2[i];
            }
            let res = &set1 ^ &set2;
            for i in 0..size {
                assert_eq!(res[i], xor[i]);
            }
        };
        let mut rng = thread_rng();
        for _ in 0..100 {
            let size = rng.gen_range(0..=10000);
            do_test(size);
        }
    }
}
