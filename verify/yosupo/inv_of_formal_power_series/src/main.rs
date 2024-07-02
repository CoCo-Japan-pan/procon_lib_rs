// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inv_of_formal_power_series

use fps_utils::Fps;
use proconio::input;
use static_modint::ModInt998244353;

type MInt = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [MInt; n]
    }
    let a = Fps::from(a);
    let b = a.inverse(n);
    println!("{}", b);
}
