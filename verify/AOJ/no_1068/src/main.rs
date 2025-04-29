// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1068

use algebra::{Commutative, IdempotentMonoid, Monoid};
use proconio::{fastout, input};
use sparse_table_on_segtree::SparseTableOnSegTree;

#[derive(Debug, Clone)]
pub enum MinMonoid {}
impl Monoid for MinMonoid {
    type Target = u32;
    fn id_element() -> Self::Target {
        u32::MAX
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}
impl IdempotentMonoid for MinMonoid {}
impl Commutative for MinMonoid {}

#[fastout]
fn main() {
    loop {
        input! {
            r: usize,
            c: usize,
            q: usize,
        }
        if r == 0 {
            break;
        }
        input! {
            grid: [[u32; c]; r],
        }
        let seg = SparseTableOnSegTree::<MinMonoid>::new(grid);
        for _ in 0..q {
            input! {
                r1: usize,
                c1: usize,
                r2: usize,
                c2: usize,
            }
            let ans = seg.prod(r1..=r2, c1..=c2);
            println!("{}", ans);
        }
    }
}
