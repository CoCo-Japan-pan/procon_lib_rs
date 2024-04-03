// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2667

use hld::HLD;
use proconio::{fastout, input};
use raq_rsq::RAQRSQ;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in a_b {
        graph[a].push(b);
        graph[b].push(a);
    }
    let hld = HLD::new(graph, 0);
    let mut ft = RAQRSQ::new(n, 0_i64);
    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }
        if t == 0 {
            let mut sum = 0;
            for path in hld.path(a, b, false) {
                match path {
                    hld::Path::Ascending(l, r) => {
                        sum += ft.sum(l..r);
                    }
                    hld::Path::Descending(l, r) => {
                        sum += ft.sum(l..r);
                    }
                }
            }
            println!("{}", sum);
        } else {
            let (l, r) = hld.subtree(a, false);
            ft.add(l..r, b as i64);
        }
    }
}
