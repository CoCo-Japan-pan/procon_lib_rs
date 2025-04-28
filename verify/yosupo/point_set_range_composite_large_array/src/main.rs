// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

use algebra::Monoid;
use dynamic_segtree::DynamicSegTree;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as MInt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MyMonoid {}
impl Monoid for MyMonoid {
    type Target = (MInt, MInt);
    fn id_element() -> Self::Target {
        (MInt::new(1), MInt::new(0))
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        (a.0 * b.0, a.1 * b.0 + b.1)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = DynamicSegTree::<MyMonoid>::new(n);
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { p: usize, c: (MInt, MInt) }
                seg.set(p, c);
            }
            1 => {
                input! { l: usize, r: usize, x: MInt }
                let (a, b) = seg.prod(l..r);
                println!("{}", a * x + b);
            }
            _ => unreachable!(),
        }
    }
}
