// https://judge.yosupo.jp/problem/two_edge_connected_components

use lowlink::LowLink;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(usize, usize); m],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for (a, b) in a_b {
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    };
    let ll = LowLink::new(&graph);
    let tecc = ll.two_edge_cc().0;
    println!("{}", tecc.len());
    for cc in tecc {
        print!("{}", cc.len());
        for v in cc {
            print!(" {}", v);
        }
        println!();
    }
}
