// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear

use floor_ceil_sum::floor_sum;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n_m_a_b: [(i64, i64, i64, i64); t],
    }
    for (n, m, a, b) in n_m_a_b {
        println!("{}", floor_sum(n, m, a, b));
    }
}
