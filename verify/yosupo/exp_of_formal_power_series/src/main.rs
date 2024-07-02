// verification-helper: PROBLEM https://judge.yosupo.jp/problem/exp_of_formal_power_series

use fps_utils::Fps;
use proconio::input;
use static_modint::ModInt998244353 as MInt;

fn main() {
    input! {
        n: usize,
        a: [MInt; n]
    }
    let b = Fps::from(a).exp(n);
    println!("{}", b);
}
