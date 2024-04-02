// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1068

use algebra::Monoid;
use proconio::{fastout, input};
use seg_tree_2d::SegTree2D;

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
        let seg = SegTree2D::<MinMonoid>::from(&grid);
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
