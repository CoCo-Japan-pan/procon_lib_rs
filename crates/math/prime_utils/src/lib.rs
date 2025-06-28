//! [エラトステネスの篩](https://qiita.com/drken/items/3beb679e54266f20ab63)

use std::ops::{Add, MulAssign, Sub};

pub struct Eratosthenes {
    max_n: usize,
    primes: Vec<usize>,
    min_factor: Vec<usize>,
}

impl Eratosthenes {
    /// `O(NloglogN)` `max_n`以下の素数を求める
    pub fn new(max_n: usize) -> Self {
        if max_n == 0 {
            return Self {
                max_n: 0,
                primes: vec![],
                min_factor: vec![],
            };
        }
        let mut min_factor = vec![0; max_n + 1];
        let mut primes = vec![];
        min_factor[1] = 1;
        for num in 2..=max_n {
            if min_factor[num] != 0 {
                continue;
            }
            primes.push(num);
            let mut cur_num = num;
            while cur_num <= max_n {
                if min_factor[cur_num] == 0 {
                    min_factor[cur_num] = num;
                }
                cur_num += num;
            }
        }
        Self {
            max_n,
            primes,
            min_factor,
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n <= self.max_n);
        n >= 2 && self.min_factor[n] == n
    }

    pub fn get_primes(&self) -> &[usize] {
        &self.primes
    }

    /// `O(log n)` でnを素因数分解  
    /// (素因数、べき) の配列を返す
    pub fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        assert!(n <= self.max_n);
        let mut res = vec![];
        while n > 1 {
            let p = self.min_factor[n];
            let mut cnt = 0;
            while self.min_factor[n] == p {
                cnt += 1;
                n /= p;
            }
            res.push((p, cnt));
        }
        res
    }

    /// `√r`以下の素数を構造体のメンバとして持っていることを前提とする  
    /// 閉区間`[l, r]`の素因数分解をまとめて行う  
    /// `M = max(r - l + 1, √r)` として `O(M loglog M)`  
    /// 素因数分解の結果を二次元配列ですべて持つのでメモリ使用量に注意  
    /// <https://atcoder.jp/contests/abc227/editorial/2909>
    pub fn factorize_range(&self, l: usize, r: usize) -> Vec<Vec<(usize, usize)>> {
        if r < l {
            return vec![];
        }
        assert!(r / self.max_n <= self.max_n);
        let mut ret = vec![vec![]; r - l + 1];
        let mut nums = (l..=r).collect::<Vec<_>>();
        for &p in &self.primes {
            for num in ((l + p - 1) / p * p..=r).step_by(p) {
                if num == 0 {
                    continue;
                }
                let mut cnt = 0;
                let idx = num - l;
                while nums[idx] % p == 0 {
                    nums[idx] /= p;
                    cnt += 1;
                }
                ret[idx].push((p, cnt));
            }
        }
        for (idx, &num) in nums.iter().enumerate() {
            if num > 1 {
                ret[idx].push((num, 1));
            }
        }
        ret
    }

    /// `√r` 以下の素数を構造体のメンバとして持っていることを前提とする  
    /// 閉区間 `[l, r]` が素数か否かをまとめて判定  
    /// `M = max(r - l + 1, √r)` として `O(M loglog M)`
    pub fn is_prime_range(&self, l: usize, r: usize) -> Vec<bool> {
        if r < l {
            return vec![];
        }
        assert!(r / self.max_n <= self.max_n);
        let mut ret = vec![true; r - l + 1];
        if l == 0 {
            ret[0] = false;
        }
        if l <= 1 {
            ret[1 - l] = false;
        }
        for &p in &self.primes {
            for num in ((l + p - 1) / p * p..=r).step_by(p) {
                if num == p {
                    continue;
                }
                let idx = num - l;
                ret[idx] = false;
            }
        }
        ret
    }

    /// 約数の個数オーダーで約数列挙 特に出力はソートしていないので注意
    pub fn enumerate_divisors(&self, n: usize) -> Vec<usize> {
        let pc = self.factorize(n);
        let size = pc.iter().map(|(_, c)| c + 1).product::<usize>();
        let mut ret = Vec::with_capacity(size);
        ret.push(1);
        for (p, c) in pc {
            let cur_size = ret.len();
            for i in 0..cur_size {
                let mut new_num = ret[i];
                for _ in 0..c {
                    new_num *= p;
                    ret.push(new_num);
                }
            }
        }
        ret
    }

    /// 倍数関係に関する高速ゼータ変換  
    /// `list[i] = func({list[iの倍数達]})` に変換する  
    /// 可換な二項演算`func`を指定する  
    /// 0番目の値については何もしないので注意
    pub fn multiple_zeta<T: Copy>(&self, mut list: Vec<T>, func: impl Fn(T, T) -> T) -> Vec<T> {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in (1..=(n / p)).rev() {
                list[i] = func(list[i], list[i * p]);
            }
        }
        list
    }

    /// 倍数関係に関する高速メビウス変換(加算の逆演算)  
    /// 0番目の値については何もしないので注意
    pub fn multiple_mobius<T: Sub<Output = T> + Copy>(&self, mut list: Vec<T>) -> Vec<T> {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in 1..=(n / p) {
                list[i] = list[i] - list[i * p];
            }
        }
        list
    }

    /// 添え字gcd畳み込み  
    /// 0番目の値については何もしないので注意
    pub fn gcd_convolution<T: Add<Output = T> + Sub<Output = T> + MulAssign + Copy>(
        &self,
        f: &[T],
        g: &[T],
    ) -> Vec<T> {
        assert_eq!(f.len(), g.len());
        let n = f.len().saturating_sub(1);
        assert!(n <= self.max_n);
        let f = f.to_vec();
        let mut f = self.multiple_zeta(f, |a, b| a + b);
        let g = g.to_vec();
        let g = self.multiple_zeta(g, |a, b| a + b);
        for i in 1..=n {
            f[i] *= g[i];
        }
        self.multiple_mobius(f)
    }

    /// 約数関係に関する高速ゼータ変換  
    /// `list[i] = func({list[iの約数達]})` に変換する  
    /// 可換な二項演算`func`を指定する  
    /// 0番目の値については何もしないので注意
    pub fn divisor_zeta<T: Copy>(&self, mut list: Vec<T>, func: impl Fn(T, T) -> T) -> Vec<T> {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in 1..=(n / p) {
                list[i * p] = func(list[i * p], list[i]);
            }
        }
        list
    }

    /// 約数関係に関する高速メビウス変換(加算の逆演算)  
    /// 0番目の値については何もしないので注意
    pub fn divisor_mobius<T: Sub<Output = T> + Copy>(&self, mut list: Vec<T>) -> Vec<T> {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in (1..=(n / p)).rev() {
                list[i * p] = list[i * p] - list[i];
            }
        }
        list
    }
}

fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut res = 1;
    let mut b = (base % modulus) as u128;
    let modulus = modulus as u128;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * b) % modulus;
        }
        b = (b * b) % modulus;
        exp >>= 1;
    }
    res as u64
}

fn suspect(a: u64, mut t: u64, n: u64) -> bool {
    let mut x = mod_pow(a, t, n);
    let n1 = n - 1;
    while t != n1 && x != 1 && x != n1 {
        x = mod_pow(x, 2, n);
        t <<= 1;
    }
    ((t & 1) == 1) || x == n1
}

/// `n < 2^64`におけるミラー・ラビン素数判定法 `O(log n)`
pub fn miller_rabin(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || (n & 1) == 0 {
        return false;
    }
    let mut d = (n - 1) >> 1;
    d >>= d.trailing_zeros();
    const CHECK_LIST: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for a in CHECK_LIST.into_iter().take_while(|&a| a < n) {
        if !suspect(a, d, n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_divisors_manual() {
        let era = Eratosthenes::new(60);
        let mut divisors_60 = era.enumerate_divisors(60);
        divisors_60.sort_unstable();
        assert_eq!(divisors_60, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60])
    }

    #[test]
    fn test_multiple_zeta_manual() {
        let list = (0..=12).collect::<Vec<usize>>();
        let era = Eratosthenes::new(12);
        let list = era.multiple_zeta(list, |a, b| a + b);
        assert_eq!(list, [0, 78, 42, 30, 24, 15, 18, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_divisor_zeta_manual() {
        let list = (0..=12).collect::<Vec<usize>>();
        let era = Eratosthenes::new(12);
        let list = era.divisor_zeta(list, |a, b| a + b);
        assert_eq!(list, [0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28]);
    }

    #[test]
    fn test_zeta_mobius() {
        fn test(size: usize) {
            let mut rng = thread_rng();
            let list = (0..=size)
                .map(|_| rng.gen_range(-100_000_000..=100_000_000))
                .collect::<Vec<i64>>();
            let era = Eratosthenes::new(size);
            let zeta = era.multiple_zeta(list.clone(), |a, b| a + b);
            let mobius = era.multiple_mobius(zeta);
            assert_eq!(list, mobius);
            let zeta = era.divisor_zeta(list.clone(), |a, b| a + b);
            let mobius = era.divisor_mobius(zeta);
            assert_eq!(list, mobius);
        }
        for size in [0, 1, 10, 100, 1000, 10000, 100000, 1000000] {
            test(size);
        }
    }

    #[test]
    fn test_gcd_conv() {
        fn test(size: usize) {
            let mut rng = thread_rng();
            let f = (0..=size)
                .map(|_| rng.gen_range(-100..=100))
                .collect::<Vec<i64>>();
            let g = (0..=size)
                .map(|_| rng.gen_range(-100..=100))
                .collect::<Vec<i64>>();
            let era = Eratosthenes::new(size);
            let conv = era.gcd_convolution(&f, &g);
            let mut ans = vec![0; size + 1];
            for i in 1..=size {
                for j in 1..=size {
                    let gcd = num::integer::gcd(i, j);
                    ans[gcd] += f[i] * g[j];
                }
            }
            assert!(conv.iter().skip(1).eq(ans.iter().skip(1)));
        }
        for size in [0, 1, 10, 100, 1000] {
            test(size);
        }
    }

    #[test]
    fn test_miller_rabin() {
        const SIZE: usize = 1000000;
        let era = Eratosthenes::new(SIZE);
        for i in 1..=SIZE {
            assert_eq!(era.is_prime(i), miller_rabin(i as u64), "i = {}", i);
        }

        assert!(!miller_rabin(10_u64.pow(18) * 2 + 1));
        assert!(miller_rabin((1_u64 << 61) - 1));
    }

    #[test]
    fn test_factorize_range() {
        const SIZE: usize = 1000000;
        let era = Eratosthenes::new(SIZE);
        let fact_range = era.factorize_range(0, SIZE);
        for i in 0..=SIZE {
            let fact = era.factorize(i);
            assert_eq!(fact, fact_range[i]);
        }
    }

    #[test]
    fn test_is_prime_range() {
        const SIZE: usize = 1000000;
        let era = Eratosthenes::new(SIZE);
        let is_prime_range = era.is_prime_range(0, SIZE);
        for i in 0..=SIZE {
            assert_eq!(era.is_prime(i), is_prime_range[i], "i = {}", i);
        }
    }
}
