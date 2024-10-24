// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/CGL_4_A

use convex_hull::monotone_chain;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x_y: [(i64, i64); n],
    }
    let mut points = x_y.into_iter().map(|p| p.into()).collect::<Vec<_>>();
    points.sort_unstable();
    let ch = monotone_chain(&points, true);
    println!("{}", ch.len() - 1);
    for p in ch.iter().take(ch.len() - 1) {
        println!("{}", p);
    }
}
