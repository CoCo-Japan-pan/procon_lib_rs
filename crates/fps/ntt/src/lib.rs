//! FFT: https://www.creativ.xyz/fast-fourier-transform/  
//! 原始根, NTT friendly MOD: https://www.mathenachia.blog/ntt-mod-list-01/#toc11  
//! 任意mod: https://math314.hateblo.jp/entry/2015/05/07/014908

use static_modint::{ModInt998244353, StaticModInt};

fn ntt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(data: &mut [StaticModInt<NTT_MOD>]) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let mut width = size;
    let mut offset = width >> 1;
    // バタフライ演算
    while width > 1 {
        let base =
            StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT).pow((NTT_MOD as u64 - 1) / width as u64);
        for top in (0..size).step_by(width) {
            let mut weight = StaticModInt::<NTT_MOD>::raw(1);
            for i in top..top + offset {
                let x = data[i];
                let y = data[i + offset];
                data[i] = x + y;
                data[i + offset] = (x - y) * weight;
                weight *= base;
            }
        }
        width >>= 1;
        offset >>= 1;
    }
}

fn intt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(data: &mut [StaticModInt<NTT_MOD>]) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let mut width = 2;
    let mut offset = 1;
    // バタフライ演算
    while width <= size {
        let base = StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT)
            .pow((NTT_MOD as u64 - 1) / width as u64)
            .inv();
        for top in (0..size).step_by(width) {
            let mut weight = StaticModInt::<NTT_MOD>::raw(1);
            for i in top..top + offset {
                let x = data[i];
                let y = data[i + offset] * weight;
                data[i] = x + y;
                data[i + offset] = x - y;
                weight *= base;
            }
        }
        width <<= 1;
        offset <<= 1;
    }
}

/// NTTによる畳み込み
pub fn convolution<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    a: &[StaticModInt<NTT_MOD>],
    b: &[StaticModInt<NTT_MOD>],
) -> Vec<StaticModInt<NTT_MOD>> {
    let n = a.len() + b.len() - 1;
    let size = (n + 1).next_power_of_two();
    let mut a = a.to_owned();
    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a);
    let mut b = b.to_owned();
    b.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b);
    for (a, b) in a.iter_mut().zip(b.into_iter()) {
        *a *= b;
    }
    intt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a);
    a.resize(n, StaticModInt::<NTT_MOD>::raw(0));
    let inv_size = StaticModInt::<NTT_MOD>::raw(size as u32).inv();
    for a in a.iter_mut() {
        *a *= inv_size;
    }
    a
}

/// 998244353 = 119 * 2^23 + 1 で原始根3を持つ
pub fn convolution_998244353(a: &[ModInt998244353], b: &[ModInt998244353]) -> Vec<ModInt998244353> {
    convolution::<998244353, 3>(a, b)
}
