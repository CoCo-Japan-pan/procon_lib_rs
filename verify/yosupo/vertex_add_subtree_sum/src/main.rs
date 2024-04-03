// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use fenwick_tree::FenwickTree;
use hld::HLD;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        p: [usize; n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (i, &j) in p.iter().enumerate() {
        graph[i + 1].push(j);
        graph[j].push(i + 1);
    }
    let hld = HLD::new(graph, 0);
    let mut ft = FenwickTree::new(n, 0);
    for (i, &x) in a.iter().enumerate() {
        ft.add(hld.get_in(i), x);
    }
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { u: usize, x: i64 }
                ft.add(hld.get_in(u), x);
            }
            1 => {
                input! { u: usize }
                let (l, r) = hld.subtree(u, true);
                println!("{}", ft.sum(l..r));
            }
            _ => unreachable!(),
        }
    }
}
