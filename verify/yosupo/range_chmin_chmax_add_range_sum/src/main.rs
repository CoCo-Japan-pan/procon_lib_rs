// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum

use proconio::{fastout, input};
use range_chmin_max_add_sum::RangeChminMaxAddSum;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut seg = RangeChminMaxAddSum::from(a);
    for _ in 0..q {
        input! {
            t: u8,
            l: usize,
            r: usize,
        }
        match t {
            0 => {
                input! {
                    chmin: i64,
                }
                seg.range_chmin(l..r, chmin);
            }
            1 => {
                input! {
                    chmax: i64,
                }
                seg.range_chmax(l..r, chmax);
            }
            2 => {
                input! {
                    add: i64,
                }
                seg.range_add(l..r, add);
            }
            3 => {
                let ans = seg.prod_sum(l..r);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
