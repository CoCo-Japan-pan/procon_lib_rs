use proconio::{fastout, input};
use wavelet_matrix_rect_sum::WaveletMatrixRectSum;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x_y_w: [(usize, usize, i64); n],
        l_d_r_u: [(usize, usize, usize, usize); q],
    }
    // 座標圧縮 ただし、x座標は重複しないようにする
    let sorted_x = {
        let mut ret = x_y_w
            .iter()
            .enumerate()
            .map(|(i, &(x, _, _))| (x, i))
            .collect::<Vec<_>>();
        ret.sort();
        ret
    };
    let compressed_x = x_y_w
        .iter()
        .enumerate()
        .map(|(i, &(x, _, _))| sorted_x.binary_search(&(x, i)).unwrap())
        .collect::<Vec<_>>();
    let sorted_y = {
        let mut ret = x_y_w.iter().map(|&(_, y, _)| y).collect::<Vec<_>>();
        ret.sort();
        ret.dedup();
        ret
    };
    let (compressed_list, weighted_list) = {
        let mut compressed_list = vec![0; n];
        let mut weighted_list = vec![0; n];
        for (i, x) in compressed_x.into_iter().enumerate() {
            let (_, y, w) = x_y_w[i];
            compressed_list[x] = sorted_y.binary_search(&y).unwrap();
            weighted_list[x] = w;
        }
        (compressed_list, weighted_list)
    };
    let wm = WaveletMatrixRectSum::new(&compressed_list, &weighted_list);
    for &(l, d, r, u) in &l_d_r_u {
        let x_left = sorted_x.partition_point(|&(x, _)| x < l);
        let x_right = sorted_x.partition_point(|&(x, _)| x < r);
        let y_left = sorted_y.partition_point(|&y| y < d);
        let y_right = sorted_y.partition_point(|&y| y < u);
        println!("{}", wm.rect_sum(x_left..x_right, y_left..y_right));
    }
}
