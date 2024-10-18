use internal_modint::ModInt;

/// n以下のmod逆元列挙を`O(n)`で行う  
/// index 0には便宜的に0を入れたn+1の長さの配列を返す
pub fn enumerate_invs<M: ModInt>(n: usize) -> Vec<M> {
    let n = n + 1;
    assert!(n <= M::modulus() as usize);
    let mut invs = vec![M::raw(0); n];
    if n <= 1 {
        return invs;
    }
    invs[1] = M::raw(1);
    for i in 2..n {
        invs[i] = -invs[M::modulus() as usize % i] * M::raw(M::modulus() / i as u32);
    }
    invs
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        use static_modint::ModInt998244353 as MInt;
        const SIZE: usize = 1_000_000;
        let invs = enumerate_invs::<MInt>(SIZE);
        for i in 1..=SIZE {
            assert_eq!(invs[i] * MInt::new(i), MInt::new(1));
        }
    }
}
