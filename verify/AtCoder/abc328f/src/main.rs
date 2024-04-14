// verification-helper: PROBLEM https://atcoder.jp/contests/abc328/tasks/abc328_f

use algebra::{Commutative, Group, Monoid};
use itertools::Itertools;
use potentialized_union_find::PotentializedUnionFind;
use proconio::{fastout, input, marker::Usize1};

#[derive(Debug)]
struct AddGroup {}
impl Monoid for AddGroup {
    type Target = i64;
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        a + b
    }
    fn id_element() -> Self::Target {
        0
    }
}
impl Group for AddGroup {
    fn inverse(a: &Self::Target) -> Self::Target {
        -a
    }
}
impl Commutative for AddGroup {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b_d: [(Usize1, Usize1, i64); q],
    }
    let mut uf = PotentializedUnionFind::<AddGroup>::new(n);
    let mut ans = Vec::with_capacity(n);
    for (i, (a, b, d)) in a_b_d.into_iter().enumerate() {
        if uf.relate(a, b, d).is_ok() {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.iter().format(" "));
}
