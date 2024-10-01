// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1549

use proconio::{fastout, input};
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        l_r_d: [(usize, usize, usize); q],
    }
    let wm = WaveletMatrix::new(&a);
    for (l, r, d) in l_r_d {
        let r = r + 1;
        let pre = wm.prev_value(l..r, d);
        let next = wm.next_value(l..r, d);
        let ans = match (pre, next) {
            (Some(pre), Some(next)) => (next - d).min(d - pre),
            (Some(pre), None) => d - pre,
            (None, Some(next)) => next - d,
            (None, None) => unreachable!(),
        };
        println!("{}", ans);
    }
}
