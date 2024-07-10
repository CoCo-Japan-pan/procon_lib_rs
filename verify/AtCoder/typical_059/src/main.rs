// https://atcoder.jp/contests/typical90/tasks/typical90_bg

use bitset::BitSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        x_y: [(Usize1, Usize1); m],
        a_b: [(Usize1, Usize1); q],
    }
    let (mut graph_bitset, graph) = {
        // graph_bitset[i][j] = iからn-jに行けるかを示す
        let mut graph_bitset = Vec::with_capacity(n);
        for i in 0..n {
            let mut set = BitSet::new(n - i + 1);
            set.set(n - i, true);
            graph_bitset.push(set);
        }
        let mut graph = vec![vec![]; n];
        for (x, y) in x_y {
            graph_bitset[x].set(n - y, true);
            graph[x].push(y);
        }
        (graph_bitset, graph)
    };
    for v in (0..n).rev() {
        for &nv in &graph[v] {
            unsafe {
                *graph_bitset.as_mut_ptr().add(v) |= &graph_bitset[nv];
            }
        }
    }
    for (a, b) in a_b {
        println!(
            "{}",
            if graph_bitset[a].get(n - b) {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
