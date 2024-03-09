// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use algebra::Monoid;
use proconio::{fastout, input};
use seg_tree::SegTree;
use static_modint::ModInt998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MyMonoid {}
impl Monoid for MyMonoid {
    type S = (ModInt998244353, ModInt998244353);
    fn id_element() -> Self::S {
        (ModInt998244353::new(1), ModInt998244353::new(0))
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 * b.0, a.1 * b.0 + b.1)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b: [(ModInt998244353, ModInt998244353); n],
    }
    let mut seg = SegTree::<MyMonoid>::from(a_b);
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { p: usize, c: (ModInt998244353, ModInt998244353) }
                seg.set(p, c);
            }
            1 => {
                input! { l: usize, r: usize, x: ModInt998244353 }
                let (a, b) = seg.prod(l..r);
                println!("{}", a * x + b);
            }
            _ => unreachable!(),
        }
    }
}
