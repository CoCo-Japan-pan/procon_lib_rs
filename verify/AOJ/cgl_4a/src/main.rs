// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/CGL_4_A

use convex_hull::{monotone_chain, Point};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x_y: [(i64, i64); n],
    }
    let mut points = x_y.into_iter().map(|p| p.into()).collect::<Vec<Point>>();
    let start_point = *points.iter().min_by_key(|&&p| (p.y, p.x)).unwrap();
    points.sort_unstable();
    let hull = {
        let (mut lower_hull, mut upper_hull) = monotone_chain(&points, true);
        lower_hull.pop();
        upper_hull.pop();
        lower_hull.append(&mut upper_hull);
        lower_hull
    };
    println!("{}", hull.len());
    let start_id = hull.iter().position(|&p| p == start_point).unwrap();
    for p in hull.iter().cycle().skip(start_id).take(hull.len()) {
        println!("{} {}", p.x, p.y);
    }
}
