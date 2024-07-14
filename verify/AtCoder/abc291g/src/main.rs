// https://atcoder.jp/contests/abc291/tasks/abc291_h
use capture::crecurse;
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
    let mut cd = CentroidDecomposition::new(&graph);
    let mut ans = vec![!0; n];
    crecurse!(
        unsafe fn dfs(subtree_root: usize, p: usize) {
            let centroid = cd.get_centroid(subtree_root);
            cd.used[centroid] = true;
            ans[centroid] = p;
            for &u in &graph[centroid] {
                if cd.used[u] || u == p {
                    continue;
                }
                dfs!(u, centroid);
            }
        }
    )(0, !0);
    println!(
        "{}",
        ans.iter()
            .map(|x| if *x == !0 { -1 } else { (*x as isize) + 1 })
            .format(" ")
    );
}
