//! bitの上位・下位集合に関する高速ゼータ・メビウス変換  
//! <https://ikatakos.com/pot/programming_algorithm/dynamic_programming/subset_convolution>

use std::ops::Sub;

/// bitの上位集合に関する高速ゼータ変換  
/// list[i] = func({list[iのsuperset達]}) に変換する  
/// 可換な二項演算`func`を指定する
pub fn superset_zeta<T: Copy>(mut list: Vec<T>, func: impl Fn(T, T) -> T) -> Vec<T> {
    let len = list.len();
    assert!(len.is_power_of_two());
    let bit = len.trailing_zeros();
    for j in 0..bit {
        for i in 0..len {
            if i & (1 << j) == 0 {
                list[i] = func(list[i], list[i | (1 << j)]);
            }
        }
    }
    list
}

/// bitの上位集合に関する高速メビウス変換(加算の逆演算)
pub fn superset_mobius<T: Sub<Output = T> + Copy>(mut list: Vec<T>) -> Vec<T> {
    let len = list.len();
    assert!(len.is_power_of_two());
    let bit = len.trailing_zeros();
    for j in 0..bit {
        for i in 0..len {
            if i & (1 << j) == 0 {
                list[i] = list[i] - list[i | (1 << j)];
            }
        }
    }
    list
}

/// bitの下位集合に関する高速ゼータ変換
/// list[i] = func({list[iのsubset達]}) に変換する
/// 可換な二項演算`func`を指定する
pub fn subset_zeta<T: Copy>(mut list: Vec<T>, func: impl Fn(T, T) -> T) -> Vec<T> {
    let len = list.len();
    assert!(len.is_power_of_two());
    let bit = len.trailing_zeros();
    for j in 0..bit {
        for i in 0..len {
            if i & (1 << j) != 0 {
                list[i] = func(list[i], list[i ^ (1 << j)]);
            }
        }
    }
    list
}

/// bitの下位集合に関する高速メビウス変換(加算の逆演算)
pub fn subset_mobius<T: Sub<Output = T> + Copy>(mut list: Vec<T>) -> Vec<T> {
    let len = list.len();
    assert!(len.is_power_of_two());
    let bit = len.trailing_zeros();
    for j in 0..bit {
        for i in 0..len {
            if i & (1 << j) != 0 {
                list[i] = list[i] - list[i ^ (1 << j)];
            }
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_superset_zeta() {
        const N: usize = 1 << 10;
        let mut rng = thread_rng();
        let list = (0..N)
            .map(|_| rng.gen_range(-100..=100))
            .collect::<Vec<i64>>();
        let superset = superset_zeta(list.clone(), |a, b| a + b);
        let naive = (0..N)
            .map(|i| {
                (0..N)
                    .filter(|&j| (i & j) == i)
                    .map(|j| list[j])
                    .sum::<i64>()
            })
            .collect::<Vec<_>>();
        assert_eq!(superset, naive);
    }

    #[test]
    fn test_superset_mobius() {
        const N: usize = 1 << 10;
        let mut rng = thread_rng();
        let list = (0..N)
            .map(|_| rng.gen_range(-100..=100))
            .collect::<Vec<i64>>();
        let superset = superset_zeta(list.clone(), |a, b| a + b);
        let mobius = superset_mobius(superset);
        assert_eq!(list, mobius);
    }

    #[test]
    fn test_subset_zeta() {
        const N: usize = 1 << 10;
        let mut rng = thread_rng();
        let list = (0..N)
            .map(|_| rng.gen_range(-100..=100))
            .collect::<Vec<i64>>();
        let subset = subset_zeta(list.clone(), |a, b| a + b);
        let naive = (0..N)
            .map(|i| {
                (0..N)
                    .filter(|&j| (i | j) == i)
                    .map(|j| list[j])
                    .sum::<i64>()
            })
            .collect::<Vec<_>>();
        assert_eq!(subset, naive);
    }

    #[test]
    fn test_subset_mobius() {
        const N: usize = 1 << 10;
        let mut rng = thread_rng();
        let list = (0..N)
            .map(|_| rng.gen_range(-100..=100))
            .collect::<Vec<i64>>();
        let subset = subset_zeta(list.clone(), |a, b| a + b);
        let mobius = subset_mobius(subset);
        assert_eq!(list, mobius);
    }
}
