//! [FFT](https://www.creativ.xyz/fast-fourier-transform/)  
//! [原始根, NTT friendly MOD](https://www.mathenachia.blog/ntt-mod-list-01/)  
//! [高速化](https://tayu0110.hatenablog.com/entry/2023/05/06/023244)  
//! [Reference](https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/convolution.rs)  
//! ジェネリックに紐づくstaticの実現がむずくてキャッシュは毎回とることにした...  

use dynamic_modint::{DynamicModInt, ModContainer};
use internal_modint::{inv_gcd, safe_mod, ModInt};
use internal_type_traits::Zero;
use static_modint::StaticModInt;
use std::ops::{AddAssign, Mul};

fn prepare<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
) -> ([StaticModInt<NTT_MOD>; 30], [StaticModInt<NTT_MOD>; 30]) {
    let g = StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT);
    let mut es = [StaticModInt::<NTT_MOD>::raw(0); 30];
    let mut ies = [StaticModInt::<NTT_MOD>::raw(0); 30];
    let cnt2 = (NTT_MOD - 1).trailing_zeros() as usize;
    let mut e = g.pow(((NTT_MOD - 1) >> cnt2).into());
    let mut ie = e.inv();
    for i in (2..=cnt2).rev() {
        es[i - 2] = e;
        ies[i - 2] = ie;
        e *= e;
        ie *= ie;
    }
    for i in 1..30 {
        es[i] *= es[i - 1];
        ies[i] *= ies[i - 1];
    }
    (es, ies)
}

fn ntt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    data: &mut [StaticModInt<NTT_MOD>],
    sum_e: &[StaticModInt<NTT_MOD>; 30],
) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let height = size.next_power_of_two().trailing_zeros();
    for ph in 1..=height {
        let w = 1 << (ph - 1);
        let p = 1 << (height - ph);
        let mut now = StaticModInt::<NTT_MOD>::raw(1);
        for s in 0..w {
            let offset = s << (height - ph + 1);
            for i in 0..p {
                let l = data[i + offset];
                let r = data[i + offset + p] * now;
                data[i + offset] = l + r;
                data[i + offset + p] = l - r;
            }
            now *= sum_e[(!s).trailing_zeros() as usize];
        }
    }
}

fn intt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    data: &mut [StaticModInt<NTT_MOD>],
    sum_ie: &[StaticModInt<NTT_MOD>; 30],
) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let height = size.next_power_of_two().trailing_zeros();
    for ph in (1..=height).rev() {
        let w = 1 << (ph - 1);
        let p = 1 << (height - ph);
        let mut inow = StaticModInt::<NTT_MOD>::raw(1);
        for s in 0..w {
            let offset = s << (height - ph + 1);
            for i in 0..p {
                let l = data[i + offset];
                let r = data[i + offset + p];
                data[i + offset] = l + r;
                data[i + offset + p] = (l - r) * inow;
            }
            inow *= sum_ie[(!s).trailing_zeros() as usize];
        }
    }
}

fn convolution_naive<M: Zero + AddAssign + Mul<Output = M> + Copy>(a: &[M], b: &[M]) -> Vec<M> {
    let (n, m) = (a.len(), b.len());
    let mut ret = vec![M::zero(); n + m - 1];
    for (i, j) in (0..n).flat_map(|i| (0..m).map(move |j| (i, j))) {
        ret[i + j] += a[i] * b[j];
    }
    ret
}

/// NTTによる畳み込み
fn convolution_ntt_friendly<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    a: &[StaticModInt<NTT_MOD>],
    b: &[StaticModInt<NTT_MOD>],
) -> Vec<StaticModInt<NTT_MOD>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    if a.len().min(b.len()) <= 60 {
        return convolution_naive(a, b);
    }
    let n = a.len() + b.len() - 1;
    let size = n.next_power_of_two();
    // NTT_MODは1のsize乗根を持つはず
    assert!((NTT_MOD - 1) % size as u32 == 0);
    let mut a = a.to_owned();
    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    let (sum_e, sum_ie) = prepare::<NTT_MOD, PRIMITIVE_ROOT>();
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, &sum_e);
    let mut b = b.to_owned();
    b.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b, &sum_e);
    for (a, b) in a.iter_mut().zip(b) {
        *a *= b;
    }
    intt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, &sum_ie);
    a.resize(n, StaticModInt::<NTT_MOD>::raw(0));
    let inv_size = StaticModInt::<NTT_MOD>::raw(size as u32).inv();
    for a in a.iter_mut() {
        *a *= inv_size;
    }
    a
}

/// 取りうる最大値を超えるmodを表現できるようなmodの組を選んで畳み込み、Garnerで復元
fn convolution_aribtrary_u32_mod<M: ModInt>(a: &[M], b: &[M]) -> Vec<M> {
    const G_MOD1: u32 = 167_772_161;
    const G_MOD2: u32 = 469_762_049;
    const G_MOD3: u32 = 1_224_736_769;
    let x = convolution_ntt_friendly::<G_MOD1, 3>(
        a.iter()
            .map(|x| StaticModInt::<G_MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<G_MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let y = convolution_ntt_friendly::<G_MOD2, 3>(
        a.iter()
            .map(|x| StaticModInt::<G_MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<G_MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let z = convolution_ntt_friendly::<G_MOD3, 3>(
        a.iter()
            .map(|x| StaticModInt::<G_MOD3>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<G_MOD3>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );

    let m1_inv_m2 = StaticModInt::<G_MOD2>::new(G_MOD1).inv();
    let m12_inv_m3 = StaticModInt::<G_MOD3>::new(G_MOD1 as u64 * G_MOD2 as u64).inv();
    let m12_mod = M::new(G_MOD1 as u64 * G_MOD2 as u64);
    let mut ret = vec![M::raw(0); x.len()];
    for (i, r) in ret.iter_mut().enumerate() {
        let v1 = ((StaticModInt::<G_MOD2>::new(y[i].value())
            - StaticModInt::<G_MOD2>::new(x[i].value()))
            * m1_inv_m2)
            .value();
        let v2 = ((StaticModInt::<G_MOD3>::new(z[i].value())
            - StaticModInt::<G_MOD3>::new(x[i].value())
            - StaticModInt::<G_MOD3>::new(G_MOD1) * StaticModInt::<G_MOD3>::new(v1))
            * m12_inv_m3)
            .value();
        let constants = M::new(x[i].value()) + M::new(G_MOD1) * M::new(v1) + m12_mod * M::new(v2);
        *r = constants;
    }
    ret
}

/// ModIntに畳み込みも追加したトレイト
pub trait ConvHelper: ModInt {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self>;
}

impl<const MOD: u32> ConvHelper for StaticModInt<MOD> {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self> {
        match MOD {
            998_244_353 | 167_772_161 | 469_762_049 | 1_224_736_769 | 4_194_304_001 => {
                convolution_ntt_friendly::<MOD, 3>(a, b)
            }
            754_974_721 => convolution_ntt_friendly::<MOD, 11>(a, b),
            _ => convolution_aribtrary_u32_mod(a, b),
        }
    }
}

impl<MOD: ModContainer> ConvHelper for DynamicModInt<MOD> {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self> {
        convolution_aribtrary_u32_mod(a, b)
    }
}

/// NTT-freindlyな場合もそうでない場合も包括する
pub fn convolution<M: ConvHelper>(a: &[M], b: &[M]) -> Vec<M> {
    M::convolution(a, b)
}

fn convolution_raw<M>(a: &[i64], b: &[i64]) -> Vec<i64>
where
    M: ConvHelper,
{
    let a = a.iter().map(|&x| M::new(x)).collect::<Vec<_>>();
    let b = b.iter().map(|&x| M::new(x)).collect::<Vec<_>>();
    convolution::<M>(&a, &b)
        .into_iter()
        .map(|x| x.value() as i64)
        .collect()
}

/// i64に値が収まる場合の畳み込み(負の値も扱える)  
pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    const M1: u64 = 754_974_721; // 2^24
    const M2: u64 = 167_772_161; // 2^25
    const M3: u64 = 469_762_049; // 2^26
    const M2M3: u64 = M2 * M3;
    const M1M3: u64 = M1 * M3;
    const M1M2: u64 = M1 * M2;
    const M1M2M3: u64 = M1M2.wrapping_mul(M3);

    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    if a.len().min(b.len()) <= 60 {
        return convolution_naive(a, b);
    }

    const I1: i64 = inv_gcd(M2M3 as i64, M1 as i64).1;
    const I2: i64 = inv_gcd(M1M3 as i64, M2 as i64).1;
    const I3: i64 = inv_gcd(M1M2 as i64, M3 as i64).1;

    let (c1, c2, c3) = {
        const M1: u32 = 754_974_721;
        const M2: u32 = 167_772_161;
        const M3: u32 = 469_762_049;
        (
            convolution_raw::<StaticModInt<M1>>(a, b),
            convolution_raw::<StaticModInt<M2>>(a, b),
            convolution_raw::<StaticModInt<M3>>(a, b),
        )
    };

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((c1, c2), c3)| {
            const OFFSET: &[u64] = &[0, 0, M1M2M3, 2 * M1M2M3, 3 * M1M2M3];

            let mut x = [(c1, I1, M1, M2M3), (c2, I2, M2, M1M3), (c3, I3, M3, M1M2)]
                .iter()
                .map(|&(c, i, m1, m2)| c.wrapping_mul(i).rem_euclid(m1 as _).wrapping_mul(m2 as _))
                .fold(0, i64::wrapping_add);

            let mut diff = c1 - safe_mod(x, M1 as _);
            if diff < 0 {
                diff += M1 as i64;
            }
            x = x.wrapping_sub(OFFSET[diff.rem_euclid(5) as usize] as _);
            x
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_convolution_i64() {
        fn do_test(size: u32) {
            let mut rng = thread_rng();
            let a = (0..size)
                .map(|_| rng.gen_range(-10_000_000..=10_000_000))
                .collect::<Vec<_>>();
            let b = (0..size)
                .map(|_| rng.gen_range(-10_000_000..=10_000_000))
                .collect::<Vec<_>>();
            let naive = convolution_naive(&a, &b);
            let fast = convolution_i64(&a, &b);
            assert_eq!(naive, fast);
        }
        do_test(1);
        do_test(10);
        do_test(100);
        do_test(1_000);
        do_test(10_000);
    }
}
