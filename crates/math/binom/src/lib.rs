use internal_modint::ModInt;

pub struct Binom<T: ModInt> {
    max_n: usize,
    fact: Vec<T>,
    ifact: Vec<T>,
}

impl<T: ModInt> Binom<T> {
    pub fn new(max_n: usize) -> Self {
        let mut fact = vec![T::raw(0); max_n + 1];
        let mut ifact = vec![T::raw(0); max_n + 1];
        fact[0] = T::raw(1);
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * T::new(i as u64);
        }
        ifact[max_n] = fact[max_n].inv();
        for i in (1..=max_n).rev() {
            ifact[i - 1] = ifact[i] * T::new(i as u64);
        }
        Self { max_n, fact, ifact }
    }

    /// nCkの計算(n<kの場合は0とする)
    pub fn cmp(&self, n: usize, k: usize) -> T {
        assert!(n <= self.max_n);
        if n < k {
            T::raw(0)
        } else {
            self.fact[n] * self.ifact[k] * self.ifact[n - k]
        }
    }
}
