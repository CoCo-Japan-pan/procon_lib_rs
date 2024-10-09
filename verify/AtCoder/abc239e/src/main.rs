// verification-helper: PROBLEM https://atcoder.jp/contests/abc239/tasks/abc239_e

#![allow(non_snake_case)]
use hld::HLD;
use proconio::{fastout, input, marker::Usize1};
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [i64; N],
        A_B: [(Usize1, Usize1); N - 1],
        V_K: [(Usize1, Usize1); Q],
    }
    let graph = {
        let mut graph = vec![vec![]; N];
        for &(a, b) in &A_B {
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    };
    let hld = HLD::new(graph, 0);
    let sorted_X = {
        let mut sorted_X = X.clone();
        sorted_X.sort();
        sorted_X.dedup();
        sorted_X
    };
    let wm_list = {
        let mut wm_list = vec![0; N];
        for i in 0..N {
            let id = sorted_X.binary_search(&X[i]).unwrap();
            wm_list[hld.hld_in[i]] = id;
        }
        wm_list
    };
    let wm = WaveletMatrix::new(&wm_list);
    for (v, k) in V_K {
        let (l, r) = hld.subtree(v, true);
        let id = wm.quantile(l..r, r - l - 1 - k);
        println!("{}", sorted_X[id]);
    }
}
