// verification-helper: PROBLEM: https://judge.yosupo.jp/problem/convolution_mod

use ntt::convolution_998244353;
use proconio::{fastout, input};
use static_modint::ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [ModInt998244353; n],
        b: [ModInt998244353; m],
    }
    let c = convolution_998244353(&a, &b);
    println!(
        "{}",
        c.iter()
            .map(|&c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
