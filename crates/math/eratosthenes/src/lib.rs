//! [エラトステネスの篩](https://qiita.com/drken/items/3beb679e54266f20ab63)

pub struct Eratosthenes {
    max_n: usize,
    primes: Vec<usize>,
    min_factor: Vec<usize>,
}

impl Eratosthenes {
    /// `O(NloglogN)`
    pub fn new(max_n: usize) -> Self {
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

    /// 約数の個数オーダーで約数列挙 最後にソートしている
    pub fn divisors(&self, n: usize) -> Vec<usize> {
        let mut ret = vec![1];
        let pc = self.factorize(n);
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
        ret.sort_unstable();
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_divisors_60() {
        let era = Eratosthenes::new(60);
        let divisors_60 = era.divisors(60);
        assert_eq!(divisors_60, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60])
    }
}
