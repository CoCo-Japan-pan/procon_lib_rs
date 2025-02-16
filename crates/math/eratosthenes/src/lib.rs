//! [エラトステネスの篩](https://qiita.com/drken/items/3beb679e54266f20ab63)

use std::ops::{Add, MulAssign, SubAssign};

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

    /// `O(log N)` で素因数分解  
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
    pub fn multiple_zeta_transfrom<T: Copy>(&self, list: &mut [T], func: impl Fn(T, T) -> T) {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in (1..=(n / p)).rev() {
                list[i] = func(list[i], list[i * p]);
            }
        }
    }

    /// 倍数関係に関する高速メビウス変換(加算の逆演算)  
    /// 0番目の値については何もしないので注意
    pub fn multiple_mobius_transfrom<T: SubAssign + Copy>(&self, list: &mut [T]) {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in 1..=(n / p) {
                list[i] -= list[i * p];
            }
        }
    }

    /// 添え字gcd畳み込み  
    /// 0番目の値については何もしないので注意
    pub fn gcd_convolution<T: Add<Output = T> + SubAssign + MulAssign + Copy>(
        &self,
        f: &[T],
        g: &[T],
    ) -> Vec<T> {
        assert_eq!(f.len(), g.len());
        let n = f.len().saturating_sub(1);
        assert!(n <= self.max_n);
        let mut f = f.to_vec();
        self.multiple_zeta_transfrom(&mut f, |a, b| a + b);
        let mut g = g.to_vec();
        self.multiple_zeta_transfrom(&mut g, |a, b| a + b);
        for i in 1..=n {
            f[i] *= g[i];
        }
        self.multiple_mobius_transfrom(&mut f);
        f
    }

    /// 約数関係に関する高速ゼータ変換  
    /// `list[i] = func({list[iの約数達]})` に変換する  
    /// 可換な二項演算`func`を指定する  
    /// 0番目の値については何もしないので注意
    pub fn divisor_zeta_transfrom<T: Copy>(&self, list: &mut [T], func: impl Fn(T, T) -> T) {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in 1..=(n / p) {
                list[i * p] = func(list[i * p], list[i]);
            }
        }
    }

    /// 約数関係に関する高速メビウス変換(加算の逆演算)  
    /// 0番目の値については何もしないので注意
    pub fn divisor_mobius_transfrom<T: SubAssign + Copy>(&self, list: &mut [T]) {
        let n = list.len().saturating_sub(1);
        assert!(n <= self.max_n);
        for p in self.primes.iter().take_while(|&&p| p <= n) {
            for i in (1..=(n / p)).rev() {
                list[i * p] -= list[i];
            }
        }
    }
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
        let mut list = (0..=12).collect::<Vec<usize>>();
        let era = Eratosthenes::new(12);
        era.multiple_zeta_transfrom(&mut list, |a, b| a + b);
        assert_eq!(list, [0, 78, 42, 30, 24, 15, 18, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_divisor_zeta_manual() {
        let mut list = (0..=12).collect::<Vec<usize>>();
        let era = Eratosthenes::new(12);
        era.divisor_zeta_transfrom(&mut list, |a, b| a + b);
        assert_eq!(list, [0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28]);
    }

    #[test]
    fn test_zeta_mobius() {
        fn test(size: usize) {
            let mut rng = thread_rng();
            let list = (0..=size)
                .map(|_| rng.gen_range(-100_000_000..=100_000_000))
                .collect::<Vec<i64>>();
            let mut list_clone = list.clone();
            let era = Eratosthenes::new(size);
            era.multiple_zeta_transfrom(&mut list_clone, |a, b| a + b);
            era.multiple_mobius_transfrom(&mut list_clone);
            assert_eq!(list, list_clone);
            era.divisor_zeta_transfrom(&mut list_clone, |a, b| a + b);
            era.divisor_mobius_transfrom(&mut list_clone);
            assert_eq!(list, list_clone);
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
}
