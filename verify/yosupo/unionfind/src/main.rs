// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind
use dsu::Dsu;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        if t == 0 {
            dsu.merge(u, v);
        } else {
            println!("{}", if dsu.same(u, v) { 1 } else { 0 });
        }
    }
}
