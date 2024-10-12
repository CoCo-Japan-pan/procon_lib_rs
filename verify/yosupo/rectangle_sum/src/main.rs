// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum

use proconio::{fastout, input};
use wavelet_matrix_rect_sum::WaveletMatrixRectSum;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut x_y_w: [(usize, (usize, i64)); n],
        l_d_r_u: [(usize, usize, usize, usize); q],
    }
    x_y_w.sort_unstable();
    let (x, (y, w)): (Vec<_>, (Vec<_>, Vec<_>)) = x_y_w.into_iter().unzip();
    let sorted_y = {
        let mut sorted_y = y.clone();
        sorted_y.sort_unstable();
        sorted_y.dedup();
        sorted_y
    };
    let y = y
        .into_iter()
        .map(|y| sorted_y.binary_search(&y).unwrap())
        .collect::<Vec<_>>();
    let wm = WaveletMatrixRectSum::new(&y, &w);
    for &(l, d, r, u) in &l_d_r_u {
        let l = x.partition_point(|&x| x < l);
        let r = x.partition_point(|&x| x < r);
        let d = sorted_y.partition_point(|&y| y < d);
        let u = sorted_y.partition_point(|&y| y < u);
        println!("{}", wm.rect_sum(l..r, d..u));
    }
}
