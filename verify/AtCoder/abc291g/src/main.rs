// https://atcoder.jp/contests/abc291/tasks/abc291_h
use centroid_decomposition::CentroidDecomposition;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for &(a, b) in &a_b {
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    };
    let cd = CentroidDecomposition::new(&graph);
    let par_v = cd.calc_centroid_tree();
    let mut ans = vec![!0; n];
    for (p, v) in par_v {
        ans[v] = p;
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| if *x == !0 { -1 } else { (*x as isize) + 1 })
            .format(" ")
    );
}
