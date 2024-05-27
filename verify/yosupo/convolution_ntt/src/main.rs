// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod

use itertools::Itertools;
use ntt::convolution;
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
    let a: Vec<ModInt998244353> = a.into_iter().map(ModInt998244353::raw).collect();
    let b: Vec<ModInt998244353> = b.into_iter().map(ModInt998244353::raw).collect();
    let c = convolution(&a, &b);
    println!("{}", c.iter().format(" "));
}
