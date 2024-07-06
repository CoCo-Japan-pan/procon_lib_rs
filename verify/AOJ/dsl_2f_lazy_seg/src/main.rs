// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_F

use lazy_segtree::LazySegTree;
use proconio::{fastout, input};

#[derive(Clone, Debug, PartialEq, Eq)]
struct MyMap {
    update: Option<u32>,
}
impl algebra::Action for MyMap {
    type Target = u32;
    fn id_map() -> Self {
        MyMap { update: None }
    }
    fn composition(&mut self, rhs: &Self) {
        if let Some(x) = rhs.update {
            self.update = Some(x);
        }
    }
    fn mapping(&self, target: &mut Self::Target) {
        if let Some(x) = self.update {
            *target = x;
        }
    }
}
impl algebra::NonCommutative for MyMap {}

struct MinMonoid {}
impl algebra::Monoid for MinMonoid {
    type Target = u32;
    fn id_element() -> Self::Target {
        u32::MAX
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}

struct RmqRuq {}
impl algebra::ActionMonoid for RmqRuq {
    type Monoid = MinMonoid;
    type Action = MyMap;
}

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
