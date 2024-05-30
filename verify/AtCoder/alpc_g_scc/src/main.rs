// verification-helper: PROBLEM https://atcoder.jp/contests/practice2/tasks/practice2_g

use itertools::Itertools;
use proconio::{fastout, input};
use scc::SccGraph;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(usize, usize); m],
    }
    let mut graph = SccGraph::new(n);
    for (a, b) in a_b {
        graph.add_edge(a, b);
    }
    let scc = graph.scc();
    println!("{}", scc.len());
    for component in scc {
        print!("{}", component.len());
        println!("{}", component.iter().format(" "));
    }
}
