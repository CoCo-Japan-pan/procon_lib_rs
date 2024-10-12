// verification-helper: PROBLEM https://yukicoder.me/problems/no/738

use proconio::{fastout, input};
use wavelet_matrix::WaveletMatrix;
use wavelet_matrix_cum_sum::WaveletMatrixCumSum;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let sorted = {
        let mut ret = a.clone();
        ret.sort();
        ret.dedup();
        ret
    };
    let compressed: Vec<usize> = a.iter().map(|x| sorted.binary_search(x).unwrap()).collect();
    let wm = WaveletMatrix::new(&compressed);
    let wm_sum = WaveletMatrixCumSum::new(&compressed, &a);
    let mid = k / 2;
    let mut ans = i64::MAX;
    for start in 0..=n - k {
        let end = start + k;
        let medium = wm.quantile(start..end, mid);
        let (less, _, more) = wm.rank_less_eq_more(medium, start..end);
        let less_sum = wm_sum.rect_sum(start..end, ..medium);
        let less_diff = less as i64 * sorted[medium] - less_sum;
        let more_sum = wm_sum.rect_sum(start..end, medium + 1..);
        let more_diff = more_sum - more as i64 * sorted[medium];
        ans = ans.min(less_diff + more_diff);
    }
    println!("{}", ans);
}
