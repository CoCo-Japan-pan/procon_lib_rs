use proconio::{fastout, input};
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let cum_sum = {
        let mut ret = vec![0; n + 1];
        for i in 0..n {
            ret[i + 1] = ret[i] + a[i];
        }
        ret
    };
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
    let wm = WaveletMatrix::new(&compressed);
    let mid = k / 2;
    let mut ans = i64::MAX;
    for start in 0..=n - k {
        let end = start + k;
        let medium = wm.quantile(start..end, mid);
        let (less, _, more) = wm.rank_less_eq_more(medium, start..end);
        todo!();
    }
}
