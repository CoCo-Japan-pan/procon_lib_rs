// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D

use dual_seg_tree::DualSegTree;
use proconio::{fastout, input};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RUQ {
    value: Option<u32>,
}

impl algebra::Map for RUQ {
    type Target = u32;
    fn id() -> Self {
        Self { value: None }
    }
    fn composition(&mut self, rhs: &Self) {
        if rhs.value.is_some() {
            *self = *rhs;
        }
    }
    fn mapping(&self, target: &Self::Target) -> Self::Target {
        if let Some(value) = self.value {
            value
        } else {
            *target
        }
    }
}

impl algebra::NonCommutativeMap for RUQ {}

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
            seg.apply_non_commutative(s..=t, &map);
        } else {
            input! {
                i: usize,
            }
            let mapped = seg.get_mapped(i, (1_u32 << 31) - 1);
            println!("{}", mapped);
        }
    }
}
