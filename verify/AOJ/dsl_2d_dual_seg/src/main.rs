// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D

use algebra::Map;
use dual_segtree::DualSegTree;
use proconio::{fastout, input};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RUQ {
    time_stamp: usize,
    value: u32,
}

impl algebra::Map for RUQ {
    type Target = Self;
    fn id_map() -> Self {
        Self {
            time_stamp: 0,
            value: (1_u32 << 31) - 1,
        }
    }
    fn composition(&mut self, rhs: &Self) {
        if self.time_stamp < rhs.time_stamp {
            *self = *rhs;
        }
    }
    fn mapping(&self, target: &mut Self::Target) {
        if self.time_stamp > target.time_stamp {
            *target = *self;
        }
    }
}

impl algebra::CommutativeMap for RUQ {}

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
            seg.apply_commutative(s..=t, &map);
        } else {
            input! {
                i: usize,
            }
            let composed = seg.get_composition(i);
            let mut target = RUQ::id_map();
            composed.mapping(&mut target);
            println!("{}", target.value);
        }
    }
}
