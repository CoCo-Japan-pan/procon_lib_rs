// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};
use rolling_hash::RollingHash;

#[fastout]
fn main() {
    input! {
        S: Chars,
    }
    let rh = RollingHash::new(&S);
    for i in 0..S.len() {
        // [i, i) ~ [i, S.len() + 1) で二分探索
        let mut left = i;
        let mut right = S.len() + 1;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if rh.get_hash(i, mid) == rh.get_prefix_hash(mid - i) {
                left = mid;
            } else {
                right = mid;
            }
        }
        print!("{} ", left - i);
    }
}
