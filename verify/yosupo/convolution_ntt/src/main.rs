// verification-helper: PROBLEM: https://judge.yosupo.jp/problem/convolution_mod

use ntt::convolution_998244353;
use proconio::{fastout, input};
use static_modint::ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let a = a.into_iter().map(ModInt998244353::raw).collect::<Vec<_>>();
    let b = b.into_iter().map(ModInt998244353::raw).collect::<Vec<_>>();
    let c = convolution_998244353(&a, &b);
    println!(
        "{}",
        c.iter()
            .map(|&c| c.value().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
