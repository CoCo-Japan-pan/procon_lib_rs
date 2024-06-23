// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use euler_tour::EulerTour;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n - 1],
        u_v: [(usize, usize); q],
    }
    let mut graph = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        graph[p].push(i + 1);
        graph[i + 1].push(p);
    }
    let euler_tour = EulerTour::new(&graph, 0);
    for (u, v) in u_v {
        println!("{}", euler_tour.lca(u, v));
    }
}
