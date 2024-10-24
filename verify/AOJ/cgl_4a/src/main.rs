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
    let hull = {
        let (mut lower_hull, mut upper_hull) = monotone_chain(&points, true);
        lower_hull.pop();
        upper_hull.pop();
        lower_hull.append(&mut upper_hull);
        lower_hull
    };
    println!("{}", hull.len());
    for p in hull {
        println!("{}", p);
    }
}
