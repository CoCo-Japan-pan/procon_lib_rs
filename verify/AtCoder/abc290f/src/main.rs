// verification-helper: PROBLEM https://atcoder.jp/contests/abc290/tasks/abc290_f

#![allow(non_snake_case)]
use binom::Binom;
use proconio::{fastout, input};
use static_modint::ModInt998244353;

const MAX_BINOM: usize = 2_000_010;

#[fastout]
fn main() {
    input! {
        T: usize,
    }
    let binom = Binom::<ModInt998244353>::new(MAX_BINOM);
    for _ in 0..T {
        input! {
            N: usize,
        }
        // 2N-2を1以上の整数N個の和に分解する
        // このとき最大の直径は、N+1-(1の数)
        // 1の数は2~N-1個
        let ans = binom.cmp(2 * N - 3, N - 2) * (N + 1) - binom.cmp(2 * N - 4, N - 2) * N;
        println!("{}", ans);
    }
}
