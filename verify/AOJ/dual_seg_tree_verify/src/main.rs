// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D

use dual_seg_tree::{CommutativeMap, DualSegTree};
use proconio::{fastout, input};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RUQ {
    time_stamp: usize,
    value: u32,
}

impl CommutativeMap for RUQ {
    fn id() -> Self {
        Self {
            time_stamp: 0,
            value: u32::MAX,
        }
    }
    fn compostion(a: &Self, b: &Self) -> Self {
        match a.time_stamp.cmp(&b.time_stamp) {
            std::cmp::Ordering::Greater => *a,
            std::cmp::Ordering::Less => *b,
            std::cmp::Ordering::Equal => {
                assert!(a.value == b.value);
                *a
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = DualSegTree::<RUQ>::new(n);
    for time_stamp in 1..=q {
        input! {
            query_type: u32,
        }
        if query_type == 0 {
            input! {
                s: usize,
                t: usize,
                x: u32,
            }
            let map = RUQ {
                time_stamp,
                value: x,
            };
            seg.apply(s..=t, &map);
        } else {
            input! {
                i: usize,
            }
            let composed = seg.get(i);
            println!(
                "{}",
                if composed.time_stamp == 0 {
                    (1_u32 << 31) - 1
                } else {
                    composed.value
                }
            );
        }
    }
}
