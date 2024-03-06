// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_F

use lazy_seg_tree::LazySegTree;
use proconio::{fastout, input};

#[derive(Clone, Debug, PartialEq, Eq)]
struct MyMap {
    update: Option<u32>,
}
impl algebra::Map for MyMap {
    type Target = u32;
    fn id_map() -> Self {
        MyMap { update: None }
    }
    fn composition(&mut self, rhs: &Self) {
        if let Some(x) = rhs.update {
            self.update = Some(x);
        }
    }
    fn mapping(&self, target: &Self::Target) -> Self::Target {
        if let Some(x) = self.update {
            x
        } else {
            *target
        }
    }
}

struct MinMonoid {}
impl algebra::Monoid for MinMonoid {
    type S = u32;
    fn id_element() -> Self::S {
        u32::MAX
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}

struct RmqRuq {}
impl algebra::MapMonoid for RmqRuq {
    type M = MinMonoid;
    type F = MyMap;
}
impl algebra::NonCommutativeMapMonoid for RmqRuq {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lazy_seg = LazySegTree::<RmqRuq>::from(vec![(1_u32 << 31) - 1; n]);
    for _ in 0..q {
        input! {
            t: u32,
        }
        if t == 0 {
            input! {
                s: usize,
                t: usize,
                x: u32,
            }
            let map = MyMap { update: Some(x) };
            lazy_seg.apply_range_non_commutative(s..=t, &map);
        } else {
            input! {
                s: usize,
                t: usize,
            }
            println!("{}", lazy_seg.prod(s..=t));
        }
    }
}
