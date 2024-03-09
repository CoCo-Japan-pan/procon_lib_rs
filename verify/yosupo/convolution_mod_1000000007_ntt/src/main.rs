// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007

use ntt_arbitrary_mod::convolution_aribtrary_u32_mod;
use proconio::{fastout, input};
use static_modint::ModInt1000000007;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [ModInt1000000007; n],
        b: [ModInt1000000007; m],
    }
    let c: Vec<ModInt1000000007> = convolution_aribtrary_u32_mod(&a, &b);
    for c in c {
        print!("{} ", c);
    }
    println!();
}
