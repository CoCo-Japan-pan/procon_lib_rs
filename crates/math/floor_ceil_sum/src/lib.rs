/// Calculates the sum of `floor((a * i + b) / m)` for `0 <= i < n`.
/// # Example
///
/// ```
/// use floor_ceil_sum::floor_sum;
///
/// assert_eq!(floor_sum(6, 5, 4, 3), 13);
/// ```
pub fn floor_sum(mut n: i64, mut m: i64, mut a: i64, mut b: i64) -> i64 {
    assert!(0 <= n);
    assert!(1 <= m);
    let mut ans = 0_i64;
    if a < 0 {
        let a_div = a.div_euclid(m);
        ans += a_div * n * (n - 1) / 2;
        a -= a_div * m;
    }
    if b < 0 {
        let b_div = b.div_euclid(m);
        ans += b_div * n;
        b -= b_div * m;
    }
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}

/// Calculates the sum of `ceil((a * i + b) / m)` for `0 <= i < n`.
pub fn ceil_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    // ceil(x) = -floor(-x)
    -floor_sum(n, m, -a, -b)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn test_floor_sum() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(0..10000);
            let m = rng.gen_range(1..=1000_000_000);
            let a = rng.gen_range(-1000_000_000..=1000_000_000);
            let b = rng.gen_range(-1000_000_000..=1000_000_000);
            let mut ans = 0;
            for i in 0..n {
                ans += (a * i as i64 + b).div_euclid(m);
            }
            assert_eq!(floor_sum(n, m, a, b), ans);
        }
    }
    #[test]
    fn test_ceil_sum() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(0..10000);
            let m = rng.gen_range(1..=1000_000_000);
            let a = rng.gen_range(-1000_000_000..=1000_000_000);
            let b = rng.gen_range(-1000_000_000..=1000_000_000);
            let mut ans = 0;
            for i in 0..n {
                ans += (a * i as i64 + b + m - 1).div_euclid(m);
            }
            assert_eq!(ceil_sum(n, m, a, b), ans);
        }
    }
}
