//! FFT: https://www.creativ.xyz/fast-fourier-transform/  
//! 原始根, NTT friendly MOD: https://www.mathenachia.blog/ntt-mod-list-01/#toc11  
//! 高速化 https://tayu0110.hatenablog.com/entry/2023/05/06/023244
//! 任意mod: https://math314.hateblo.jp/entry/2015/05/07/014908
//! 結局ほとんどac-library-rsの写経になってしまった...

use static_modint::{ModInt998244353, StaticModInt};
use std::cell::RefCell;
use std::thread::LocalKey;

// ac-library-rsを模しているが、ジェネリックの回避が面倒なので、一度u32に戻している
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ButterflyCache {
    sum_e: [u32; 30],
    sum_ie: [u32; 30],
}

impl ButterflyCache {
    fn new<const NTT_MOD: u32>(
        sum_e_in: [StaticModInt<NTT_MOD>; 30],
        sum_ie_in: [StaticModInt<NTT_MOD>; 30],
    ) -> Self {
        let mut sum_e = [0_u32; 30];
        let mut sum_ie = [0_u32; 30];
        for (num, val) in sum_e.iter_mut().zip(sum_e_in) {
            *num = val.value();
        }
        for (num, val) in sum_ie.iter_mut().zip(sum_ie_in) {
            *num = val.value();
        }
        Self { sum_e, sum_ie }
    }
}

trait GetButterflyCache<const NTT_MOD: u32> {
    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache>>>;
}

impl<const NTT_MOD: u32> GetButterflyCache<NTT_MOD> for StaticModInt<NTT_MOD> {
    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache>> = RefCell::new(None);
        }
        &BUTTERFLY_CACHE
    }
}

fn prepare<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>() -> ButterflyCache {
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
    ButterflyCache::new(es, ies)
}

fn ntt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(data: &mut [StaticModInt<NTT_MOD>]) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let height = size.next_power_of_two().trailing_zeros();
    StaticModInt::<NTT_MOD>::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let sum_e = cache
            .get_or_insert_with(prepare::<NTT_MOD, PRIMITIVE_ROOT>)
            .sum_e;
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
                now *= StaticModInt::<NTT_MOD>::raw(sum_e[(!s).trailing_zeros() as usize]);
            }
        }
    })
}

fn intt<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(data: &mut [StaticModInt<NTT_MOD>]) {
    let size = data.len();
    assert!(size.is_power_of_two());
    let height = size.next_power_of_two().trailing_zeros();
    StaticModInt::<NTT_MOD>::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let sum_ie = cache
            .get_or_insert_with(prepare::<NTT_MOD, PRIMITIVE_ROOT>)
            .sum_ie;
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
                inow *= StaticModInt::<NTT_MOD>::raw(sum_ie[(!s).trailing_zeros() as usize]);
            }
        }
    })
}

/// NTTによる畳み込み
pub fn convolution<const NTT_MOD: u32, const PRIMITIVE_ROOT: u32>(
    a: &[StaticModInt<NTT_MOD>],
    b: &[StaticModInt<NTT_MOD>],
) -> Vec<StaticModInt<NTT_MOD>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len() + b.len() - 1;
    let size = n.next_power_of_two();
    let mut a = a.to_owned();
    a.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut a);
    let mut b = b.to_owned();
    b.resize(size, StaticModInt::<NTT_MOD>::raw(0));
    ntt::<NTT_MOD, PRIMITIVE_ROOT>(&mut b);
    for (a, b) in a.iter_mut().zip(b) {
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
