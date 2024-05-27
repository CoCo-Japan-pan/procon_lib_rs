//! [FFT](https://www.creativ.xyz/fast-fourier-transform/)  
//! [原始根, NTT friendly MOD](https://www.mathenachia.blog/ntt-mod-list-01/)  
//! [高速化](https://tayu0110.hatenablog.com/entry/2023/05/06/023244)  
//! [Reference](https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/convolution.rs)  
//! ジェネリックに紐づくstaticの実現がむずくてキャッシュは毎回とることにした...  

use dynamic_modint::{DynamicModInt, ModContainer};
use modint_traits::ModInt;
use static_modint::StaticModInt;

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

/// NTTによる畳み込み
fn convolution_ntt_friendly<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    a: &[StaticModInt<NTT_MOD>],
    b: &[StaticModInt<NTT_MOD>],
) -> Vec<StaticModInt<NTT_MOD>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
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
    // どれも原子根3で、2^24乗根がある
    const MOD1: u32 = 167772161;
    const MOD2: u32 = 469762049;
    const MOD3: u32 = 1224736769;
    let x = convolution_ntt_friendly::<MOD1, 3>(
        a.iter()
            .map(|x| StaticModInt::<MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let y = convolution_ntt_friendly::<MOD2, 3>(
        a.iter()
            .map(|x| StaticModInt::<MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let z = convolution_ntt_friendly::<MOD3, 3>(
        a.iter()
            .map(|x| StaticModInt::<MOD3>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<MOD3>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );

    let m1_inv_m2 = StaticModInt::<MOD2>::new(MOD1).inv();
    let m12_inv_m3 = StaticModInt::<MOD3>::new(MOD1 as u64 * MOD2 as u64).inv();
    let m12_mod = M::new(MOD1 as u64 * MOD2 as u64);
    let mut ret = vec![M::raw(0); x.len()];
    for (i, r) in ret.iter_mut().enumerate() {
        let v1 = ((StaticModInt::<MOD2>::new(y[i].value())
            - StaticModInt::<MOD2>::new(x[i].value()))
            * m1_inv_m2)
            .value();
        let v2 = ((StaticModInt::<MOD3>::new(z[i].value())
            - StaticModInt::<MOD3>::new(x[i].value())
            - StaticModInt::<MOD3>::new(MOD1) * StaticModInt::<MOD3>::new(v1))
            * m12_inv_m3)
            .value();
        let constants = M::new(x[i].value()) + M::new(MOD1) * M::new(v1) + m12_mod * M::new(v2);
        *r = constants;
    }
    ret
}

/// ModIntに畳み込みも追加しかトレイト
pub trait ConvHelper: ModInt {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self>;
}

impl<const MOD: u32> ConvHelper for StaticModInt<MOD> {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self> {
        if MOD == 998244353 {
            convolution_ntt_friendly::<MOD, 3>(a, b)
        } else {
            convolution_aribtrary_u32_mod(a, b)
        }
    }
}

impl<MOD: ModContainer> ConvHelper for DynamicModInt<MOD> {
    fn convolution(a: &[Self], b: &[Self]) -> Vec<Self> {
        convolution_aribtrary_u32_mod(a, b)
    }
}

pub fn convolution<M: ConvHelper>(a: &[M], b: &[M]) -> Vec<M> {
    M::convolution(a, b)
}
