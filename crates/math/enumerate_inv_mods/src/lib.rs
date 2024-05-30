use modint_traits::ModInt;

/// [1, n)のmod逆元列挙を`O(n)`で行う (index 0には便宜的に0を入れておく)
pub fn enumerate_invs<M: ModInt>(n: usize) -> Vec<M> {
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
