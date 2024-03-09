//! 任意mod: https://math314.hateblo.jp/entry/2015/05/07/014908  

use modint_traits::ModInt;
use ntt::convolution;
use static_modint::StaticModInt;

/// 取りうる最大値を超えるmodを表現できるようなmodの組を選んで畳み込み、Garnerで復元
pub fn convolution_aribtrary_u32_mod<M: ModInt>(a: &[M], b: &[M]) -> Vec<M> {
    // どれも原子根3で、2^24乗根がある
    const MOD1: u32 = 167772161;
    const MOD2: u32 = 469762049;
    const MOD3: u32 = 1224736769;
    let x = convolution::<MOD1, 3>(
        a.iter()
            .map(|x| StaticModInt::<MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<MOD1>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let y = convolution::<MOD2, 3>(
        a.iter()
            .map(|x| StaticModInt::<MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
        b.iter()
            .map(|x| StaticModInt::<MOD2>::new(x.value()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let z = convolution::<MOD3, 3>(
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
