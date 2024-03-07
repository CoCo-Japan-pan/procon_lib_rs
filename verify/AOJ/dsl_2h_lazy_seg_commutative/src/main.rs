// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_H

use algebra::CommutativeMapMonoid;
use lazy_seg_tree::LazySegTree;
use proconio::{fastout, input};

struct MinMonoid {}
impl algebra::Monoid for MinMonoid {
    type S = i32;
    fn id_element() -> Self::S {
        i32::MAX
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct AddMap {
    add_val: i32,
}
impl algebra::Map for AddMap {
    type Target = i32;
    fn id_map() -> Self {
        AddMap { add_val: 0 }
    }
    fn composition(&mut self, rhs: &Self) {
        self.add_val += rhs.add_val;
    }
    fn mapping(&self, target: &mut Self::Target) {
        *target += self.add_val;
    }
}
struct RmqRaq {}
impl algebra::MapMonoid for RmqRaq {
    type M = MinMonoid;
    type F = AddMap;
}
impl CommutativeMapMonoid for RmqRaq {}

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
