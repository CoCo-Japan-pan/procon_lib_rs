// verification-helper: PROBLEM https://atcoder.jp/contests/abc294/tasks/abc294_g

use fenwick_tree::FenwickTree;
use hld::{Path, HLD};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        u_v_w: [(Usize1, Usize1, i64); n - 1],
        q: usize,
    }
    let mut graph = vec![vec![]; n];
    for &(u, v, _) in &u_v_w {
        graph[u].push(v);
        graph[v].push(u);
    }
    let hld = HLD::new(graph, 0);
    let edge_hld_id = {
        let mut ret = vec![0; n - 1];
        for i in 0..n - 1 {
            let (u, v, _) = u_v_w[i];
            if hld.parent[u] == v {
                ret[i] = hld.hld_in[u];
            } else {
                ret[i] = hld.hld_in[v];
            }
        }
        ret
    };
    let mut ft = FenwickTree::new(n, 0);
    for i in 0..n - 1 {
        let w = u_v_w[i].2;
        let id = edge_hld_id[i];
        ft.add(id, w);
    }
    for _ in 0..q {
        input! { t: usize }
        if t == 1 {
            input! {
                i: Usize1,
                w: i64,
            }
            let id = edge_hld_id[i];
            ft.set(id, w);
        } else {
            input! {
                u: Usize1,
                v: Usize1,
            }
            let mut ans = 0;
            for path in hld.path(u, v, false) {
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
