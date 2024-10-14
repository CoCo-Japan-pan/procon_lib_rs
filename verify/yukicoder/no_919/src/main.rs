// verification-helper: PROBLEM https://yukicoder.me/problems/no/919

#![allow(non_snake_case)]
use proconio::{fastout, input};
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }
    let sorted_a = {
        let mut a = A.clone();
        a.sort_unstable();
        a.dedup();
        a
    };
    let compressed = A
        .iter()
        .map(|a| sorted_a.binary_search(a).unwrap())
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::new(&compressed);
    let mut ans = 0;
    for k in 1..=N {
        let all_teams = N / k;
        let median_id = (k + 1) / 2 - 1;
        let left_sum = {
            let mut left_sum = vec![0; all_teams + 1];
            for i in 0..all_teams {
                // [i * k, (i + 1) * k)
                let left = i * k;
                let right = (i + 1) * k;
                let mid = wm.quantile(left..right, median_id);
                let power = sorted_a[mid] * k as i64;
                left_sum[i + 1] = left_sum[i] + power;
            }
            left_sum
        };
        let right_sum = {
            let mut right_sum = vec![0; all_teams + 1];
            for i in 0..all_teams {
                // [N - (i + 1) * k, N - i * k)
                let left = N - (i + 1) * k;
                let right = N - i * k;
                let mid = wm.quantile(left..right, median_id);
                let power = sorted_a[mid] * k as i64;
                right_sum[i + 1] = right_sum[i] + power;
            }
            right_sum
        };
        debug!(k, all_teams, left_sum, right_sum);
        // 累積Max
        let right_cnt_max = {
            let mut right_cnt_max = right_sum;
            for i in 1..=all_teams {
                right_cnt_max[i] = right_cnt_max[i].max(right_cnt_max[i - 1]);
            }
            right_cnt_max
        };
        for left_cnt in 0..=all_teams {
            let right_cnt = all_teams - left_cnt;
            // [0, right_cnt]の中での最大値
            let power = left_sum[left_cnt] + right_cnt_max[right_cnt];
            ans = ans.max(power);
        }
    }
    println!("{}", ans);
}

/// https://maguro.dev/debug-macro/
#[macro_export]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
