// verification-helper: PROBLEM https://yukicoder.me/problems/no/1092

use dynamic_modint::DynamicModInt;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        p: u32,
        n: u32,
        a: [u32; n],
        s: Chars,
    }
    DynamicModInt::set_modulus(p);
    let a = a.into_iter().map(DynamicModInt::raw).collect::<Vec<_>>();
    let ans = a
        .iter()
        .skip(1)
        .zip(s.iter())
        .fold(a[0], |acc, (x, &c)| match c {
            '+' => acc + *x,
            '-' => acc - *x,
            '*' => acc * *x,
            '/' => acc / *x,
            _ => unreachable!(),
        });
    println!("{}", ans);
}