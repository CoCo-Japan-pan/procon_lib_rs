// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_H

use lazy_segtree::LazySegTree;
use proconio::{fastout, input};

struct MinMonoid {}
impl algebra::Monoid for MinMonoid {
    type Target = i32;
    fn id_element() -> Self::Target {
        i32::MAX
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct AddMap {
    add_val: i32,
}
impl algebra::Action for AddMap {
    type Target = i32;
    fn id_action() -> Self {
        AddMap { add_val: 0 }
    }
    fn composition(&mut self, rhs: &Self) {
        self.add_val += rhs.add_val;
    }
    fn apply(&self, target: &mut Self::Target) {
        *target += self.add_val;
    }
}
impl algebra::Commutative for AddMap {}
struct RmqRaq {}
impl algebra::ActionMonoid for RmqRaq {
    type M = MinMonoid;
    type A = AddMap;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lazy_seg = LazySegTree::<RmqRaq>::from(vec![0; n]);
    for _ in 0..q {
        input! {
            com: usize,
        }
        if com == 0 {
            input! {
                s: usize,
                t: usize,
                x: i32,
            }
            let map = AddMap { add_val: x };
            lazy_seg.apply_range_commutative(s..=t, &map);
        } else {
            input! {
                s: usize,
                t: usize,
            }
            println!("{}", lazy_seg.prod(s..=t));
        }
    }
}
