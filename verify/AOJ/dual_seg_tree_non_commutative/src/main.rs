// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D

use dual_seg_tree::{DualSegTree, Map, NonCommutativeMap};
use proconio::{fastout, input};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RUQ {
    value: Option<u32>,
}

impl Map for RUQ {
    fn id() -> Self {
        Self { value: None }
    }
    fn compostion(a: &Self, b: &Self) -> Self {
        Self {
            value: b.value.or(a.value),
        }
    }
}

impl NonCommutativeMap for RUQ {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = DualSegTree::<RUQ>::new(n);
    for _ in 0..q {
        input! {
            query_type: u32,
        }
        if query_type == 0 {
            input! {
                s: usize,
                t: usize,
                x: u32,
            }
            let map = RUQ { value: Some(x) };
            seg.propagate_and_apply(s..=t, &map);
        } else {
            input! {
                i: usize,
            }
            let composed = seg.get(i);
            println!("{}", composed.value.unwrap_or((1_u32 << 31) - 1));
        }
    }
}
