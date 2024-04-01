// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use algebra::{MapMonoid, NonCommutativeMapMonoid};
use lazy_seg_tree::LazySegTree;
use proconio::{fastout, input};
use static_modint::ModInt998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AddMonoid {
    sum: ModInt998244353,
    len: ModInt998244353,
}
impl algebra::Monoid for AddMonoid {
    type Target = Self;
    fn id_element() -> Self::Target {
        Self {
            sum: ModInt998244353::raw(0),
            len: ModInt998244353::raw(0),
        }
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        Self {
            sum: a.sum + b.sum,
            len: a.len + b.len,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffineMap {
    b: ModInt998244353,
    c: ModInt998244353,
}
impl algebra::Map for AffineMap {
    type Target = AddMonoid;
    fn id_map() -> Self {
        Self {
            b: ModInt998244353::raw(1),
            c: ModInt998244353::raw(0),
        }
    }
    fn composition(&mut self, rhs: &Self) {
        self.c = self.c * rhs.b + rhs.c;
        self.b *= rhs.b;
    }
    fn mapping(&self, target: &mut Self::Target) {
        target.sum = self.b * target.sum + self.c * target.len;
    }
}

struct AffineRangeSum;
impl MapMonoid for AffineRangeSum {
    type Map = AffineMap;
    type Monoid = AddMonoid;
}
impl NonCommutativeMapMonoid for AffineRangeSum {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }
    let a: Vec<AddMonoid> = a
        .into_iter()
        .map(|a| AddMonoid {
            sum: ModInt998244353::raw(a),
            len: ModInt998244353::raw(1),
        })
        .collect();
    let mut seg = LazySegTree::<AffineRangeSum>::from(a);
    for _ in 0..q {
        input! {t: u32}
        if t == 0 {
            input! {l: usize, r: usize, b: u32, c: u32}
            seg.apply_range_non_commutative(
                l..r,
                &AffineMap {
                    b: ModInt998244353::raw(b),
                    c: ModInt998244353::raw(c),
                },
            );
        } else {
            input! {l: usize, r: usize}
            println!("{}", seg.prod(l..r).sum);
        }
    }
}
