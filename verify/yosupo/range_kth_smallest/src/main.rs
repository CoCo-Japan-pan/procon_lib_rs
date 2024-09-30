// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

use proconio::{fastout, input};
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        l_r_k: [(usize, usize, usize); q],
    }
    let sorted = {
        let mut ret = a.clone();
        ret.sort();
        ret.dedup();
        ret
    };
    let compressed: Vec<usize> = a
        .into_iter()
        .map(|x| sorted.binary_search(&x).unwrap())
        .collect();
    let wm = WaveletMatrix::new(compressed);
    for (l, r, k) in l_r_k {
        let ans = wm.quantile(l..r, k);
        println!("{}", sorted[ans]);
    }
}
