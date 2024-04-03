// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

use fenwick_tree::FenwickTree;
use hld::{Path, HLD};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        u_v: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in u_v {
        graph[u].push(v);
        graph[v].push(u);
    }
    let hld = HLD::new(&graph, 0);
    let mut ft = FenwickTree::new(n, 0_u64);
    for i in 0..n {
        ft.add(hld.get_in(i), a[i]);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                x: u64,
            }
            ft.add(hld.get_in(p), x);
        } else {
            input! {
                u: usize,
                v: usize,
            }
            let mut ans = 0;
            for path in hld.path(u, v, true) {
                match path {
                    Path::Ascending(l, r) | Path::Descending(l, r) => {
                        ans += ft.sum(l..r);
                    }
                }
            }
            println!("{}", ans);
        }
    }
}
