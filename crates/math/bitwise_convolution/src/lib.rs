//! bitwise AND, OR, XOR convolution

use std::ops::{Add, MulAssign, ShrAssign, Sub};
use zeta_mobius_bitset::*;

/// `c[k] = Σ a[i] * b[j] (i | j == k)`
pub fn bitwise_or_convolution<T: Copy + Add<Output = T> + Sub<Output = T> + MulAssign>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let mut a = subset_zeta(a.to_vec(), |x, y| x + y);
    let b = subset_zeta(b.to_vec(), |x, y| x + y);
    let n = a.len();
    for i in 0..n {
        a[i] *= b[i];
    }
    subset_mobius(a)
}

/// `c[k] = Σ a[i] * b[j] (i & j == k)`
pub fn bitwise_and_convolution<T: Copy + Add<Output = T> + Sub<Output = T> + MulAssign>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let mut a = superset_zeta(a.to_vec(), |x, y| x + y);
    let b = superset_zeta(b.to_vec(), |x, y| x + y);
    let n = a.len();
    for i in 0..n {
        a[i] *= b[i];
    }
    superset_mobius(a)
}

/// `c[k] = Σ a[i] * b[j] (i ^ j == k)`
pub fn bitwise_xor_convolution<
    T: Copy + Add<Output = T> + Sub<Output = T> + MulAssign + ShrAssign<u32>,
>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let mut a = fast_hadamard(a.to_vec());
    let b = fast_hadamard(b.to_vec());
    let n = a.len();
    for i in 0..n {
        a[i] *= b[i];
    }
    inv_fast_hadamard(a)
}

/// `H~n = 2^(n/2) Hn` の行列をかける `Hn`はアダマール行列
pub fn fast_hadamard<T: Copy + Add<Output = T> + Sub<Output = T>>(mut list: Vec<T>) -> Vec<T> {
    let n = list.len();
    assert!(n.is_power_of_two());
    let bit = n.trailing_zeros();
    for i in 0..bit {
        for j in 0..n {
            if j & (1 << i) == 0 {
                let x = list[j];
                let y = list[j | (1 << i)];
                list[j] = x + y;
                list[j | (1 << i)] = x - y;
            }
        }
    }
    list
}

/// `fast_hadamard`の逆行列をかける  
/// `(H~n)^(-1) = 2^(-n/2) Hn^(-1) = 2^(-n/2) Hn = 2^(-n) H~n`  
/// つまり`fast_hadamard`をして全体を長さで割るだけ
pub fn inv_fast_hadamard<T: Copy + Add<Output = T> + Sub<Output = T> + ShrAssign<u32>>(
    mut list: Vec<T>,
) -> Vec<T> {
    list = fast_hadamard(list);
    let bit = list.len().trailing_zeros();
    list.iter_mut().for_each(|x| *x >>= bit);
    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_bitor_conv() {
        let mut rng = thread_rng();
        let n = 1 << 10;
        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let c = bitwise_or_convolution(&a, &b);
        let mut c_naive = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                c_naive[i | j] += a[i] * b[j];
            }
        }
        assert_eq!(c, c_naive);
    }

    #[test]
    fn test_bitand_conv() {
        let mut rng = thread_rng();
        let n = 1 << 10;
        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let c = bitwise_and_convolution(&a, &b);
        let mut c_naive = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                c_naive[i & j] += a[i] * b[j];
            }
        }
        assert_eq!(c, c_naive);
    }

    #[test]
    fn test_xor_conv() {
        let mut rng = thread_rng();
        let n = 1 << 10;
        let a: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let b: Vec<i64> = (0..n).map(|_| rng.gen_range(-10000..=10000)).collect();
        let c = bitwise_xor_convolution(&a, &b);
        let mut c_naive = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                c_naive[i ^ j] += a[i] * b[j];
            }
        }
        assert_eq!(c, c_naive);
    }
}
