// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use algebra::Monoid;
use hld::{Path, HLD};
use proconio::{fastout, input};
use segtree::SegTree;
use static_modint::ModInt998244353;

#[derive(Clone, Copy, Debug)]
struct Affine {
    a: ModInt998244353,
    b: ModInt998244353,
}
enum AffineLeftMonoid {}
impl Monoid for AffineLeftMonoid {
    type Target = Affine;
    fn binary_operation(l: &Self::Target, r: &Self::Target) -> Self::Target {
        Self::Target {
            a: l.a * r.a,
            b: r.a * l.b + r.b,
        }
    }
    fn id_element() -> Self::Target {
        Self::Target {
            a: ModInt998244353::raw(1),
            b: ModInt998244353::raw(0),
        }
    }
}
enum AffineRightMonoid {}
impl Monoid for AffineRightMonoid {
    type Target = Affine;
    fn binary_operation(l: &Self::Target, r: &Self::Target) -> Self::Target {
        AffineLeftMonoid::binary_operation(r, l)
    }
    fn id_element() -> Self::Target {
        AffineLeftMonoid::id_element()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b: [(ModInt998244353, ModInt998244353); n],
        u_v: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in u_v {
        graph[u].push(v);
        graph[v].push(u);
    }
    let hld = HLD::new(graph, 0);
    let mut affine_vec = vec![AffineLeftMonoid::id_element(); n];
    for i in 0..n {
        affine_vec[hld.get_in(i)] = Affine {
            a: a_b[i].0,
            b: a_b[i].1,
        };
    }
    let mut seg_left = SegTree::<AffineLeftMonoid>::from(&affine_vec);
    let mut seg_right = SegTree::<AffineRightMonoid>::from(&affine_vec);
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { p: usize, c: ModInt998244353, d: ModInt998244353 }
                let p = hld.get_in(p);
                seg_left.set(p, Affine { a: c, b: d });
                seg_right.set(p, Affine { a: c, b: d });
            }
            1 => {
                input! { u: usize, v: usize, x: ModInt998244353 }
                let mut ret = AffineLeftMonoid::id_element();
                for path in hld.path(u, v, true) {
                    match path {
                        Path::Ascending(l, r) => {
                            ret = AffineLeftMonoid::binary_operation(&ret, &seg_right.prod(l..r));
                        }
                        Path::Descending(l, r) => {
                            ret = AffineLeftMonoid::binary_operation(&ret, &seg_left.prod(l..r));
                        }
                    }
                }
                let ans = ret.a * x + ret.b;
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
