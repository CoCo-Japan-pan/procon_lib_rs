//! FFT: https://www.creativ.xyz/fast-fourier-transform/  
//! 原始根, NTT friendly MOD: https://www.mathenachia.blog/ntt-mod-list-01/#toc11  
//! 任意mod: https://math314.hateblo.jp/entry/2015/05/07/014908

use static_modint::{ModInt998244353, StaticModInt};

/// sizeを超えるような2の冪乗根を持つNTT_MODでのNTTを行う
fn ntt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    data: &mut [StaticModInt<NTT_MOD>],
    size: usize,
    inv: bool,
) {
    assert!(size.is_power_of_two());
    let height = size.trailing_zeros() as usize;
    // バタフライ演算用の配置入れ替え
    for i in 0..size {
        let j = ((i as u32).reverse_bits() >> (32 - size.trailing_zeros())) as usize;
        if i < j {
            data.swap(i, j);
        }
    }
    // base^size = 1(mod NTT_MOD) (またはその逆元)
    let nth_root = if inv {
        StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT)
            .pow((NTT_MOD as u64 - 1) / size as u64)
            .inv()
    } else {
        StaticModInt::<NTT_MOD>::raw(PRIMITIVE_ROOT).pow((NTT_MOD as u64 - 1) / size as u64)
    };
    // バタフライ演算
    for h in 0..height {
        let block = 1 << h;
        // 第log2(block) + 1 段目
        // ブロックサイズ = block * 2
        let base = nth_root.pow(size as u64 / (block as u64 * 2));
        let mut weight = StaticModInt::<NTT_MOD>::raw(1);
        for j in 0..block {
            // ブロック内j番目
            // 重み = (1の2block乗根)^(j)
            for k in (0..size).step_by(block * 2) {
                let s = data[j + k];
                let t = data[j + k + block] * weight;
                data[j + k] = s + t;
                data[j + k + block] = s - t;
            }
            weight *= base;
        }
    }
    if inv {
        let inv_n = StaticModInt::<NTT_MOD>::new(size).inv();
        for a in data.iter_mut() {
            *a *= inv_n;
        }
    }
}

/// NTTによる畳み込み
pub fn convolution<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    a: &[StaticModInt<NTT_MOD>],
    b: &[StaticModInt<NTT_MOD>],
) -> Vec<StaticModInt<NTT_MOD>> {
    let n = a.len() + b.len() - 1;
    let size = (n + 1).next_power_of_two();
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    b.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, size, false);
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b, size, false);
    for i in 0..size {
        a[i] *= b[i];
    }
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a, size, true);
    a.resize(n, StaticModInt::<NTT_MOD>::raw(0));
    a
}

/// 998244353 = 119 * 2^23 + 1 で原始根3を持つ
pub fn convolution_998244353(a: &[ModInt998244353], b: &[ModInt998244353]) -> Vec<ModInt998244353> {
    convolution::<998244353, 3>(a, b)
}
