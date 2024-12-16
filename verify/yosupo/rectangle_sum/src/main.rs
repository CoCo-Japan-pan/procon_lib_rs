// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum

use proconio::{fastout, input};
use wavelet_matrix_cum_sum::WMCumSumWrapper;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x_y_w: [(usize, usize, i64); n],
        l_d_r_u: [(usize, usize, usize, usize); q],
    }
    let wm = WMCumSumWrapper::new(x_y_w);
    for &(l, d, r, u) in &l_d_r_u {
        println!("{}", wm.rect_sum(l..r, d..u));
    }
}
