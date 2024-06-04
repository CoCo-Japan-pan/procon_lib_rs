// https://judge.yosupo.jp/problem/scc

use proconio::{fastout, input};
use scc::SccGraph;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(usize, usize); m],
    }
    let scc_graph = {
        let mut scc_graph = SccGraph::new(n);
        for (a, b) in a_b {
            scc_graph.add_edge(a, b);
        }
        scc_graph
    };
    let scc = scc_graph.scc();
    println!("{}", scc.len());
    for scc in scc {
        print!("{}", scc.len());
        for &v in scc.iter() {
            print!(" {}", v);
        }
        println!();
    }
}
