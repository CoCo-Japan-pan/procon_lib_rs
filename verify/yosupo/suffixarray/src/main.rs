// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use atcoder_string::suffix_array;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let sa = suffix_array(&s);
    for (i, s) in sa.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", s);
    }
    println!();
}
