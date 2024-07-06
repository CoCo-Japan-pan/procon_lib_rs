// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_add_point_get

use algebra::{Commutative, Monoid};
use proconio::{fastout, input};
use segtree_2d_compressed::SegTree2DCompressed;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add((i32, i32, i32, i32, i64)),
    Get(i32, i32),
}

#[derive(Debug)]
struct AddMonoid;
impl Monoid for AddMonoid {
    type Target = i64;
    fn id_element() -> Self::Target {
        0
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a + *b
    }
}
impl Commutative for AddMonoid {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        l_d_r_u_w: [(i32, i32, i32, i32, i64); n],
    }
    let querys = {
        let mut querys = Vec::with_capacity(q);
        for _ in 0..q {
            input! {
                t: i32,
            }
            match t {
                0 => {
                    input! {
                        l_d_r_u_w: (i32, i32, i32, i32, i64),
                    }
                    querys.push(Query::Add(l_d_r_u_w));
                }
                1 => {
                    input! {
                        x: i32,
                        y: i32,
                    }
                    querys.push(Query::Get(x, y));
                }
                _ => unreachable!(),
            }
        }
        querys
    };
    let update_points = {
        let mut update_points = Vec::with_capacity(n);
        for (l, d, r, u, _) in l_d_r_u_w.iter() {
            update_points.push((*l, *d));
            update_points.push((*r, *u));
            update_points.push((*l, *u));
            update_points.push((*r, *d));
        }
        for q in &querys {
            match q {
                Query::Add((l, d, r, u, _)) => {
                    update_points.push((*l, *d));
                    update_points.push((*r, *u));
                    update_points.push((*l, *u));
                    update_points.push((*r, *d));
                }
                Query::Get(..) => {}
            }
        }
        update_points
    };
    let mut seg2d = SegTree2DCompressed::<AddMonoid, _>::new(&update_points);
    for (l, d, r, u, w) in l_d_r_u_w {
        seg2d.set(l, d, seg2d.get(l, d) + w);
        seg2d.set(r, u, seg2d.get(r, u) + w);
        seg2d.set(l, u, seg2d.get(l, u) - w);
        seg2d.set(r, d, seg2d.get(r, d) - w);
    }
    for q in querys {
        match q {
            Query::Add((l, d, r, u, w)) => {
                seg2d.set(l, d, seg2d.get(l, d) + w);
                seg2d.set(r, u, seg2d.get(r, u) + w);
                seg2d.set(l, u, seg2d.get(l, u) - w);
                seg2d.set(r, d, seg2d.get(r, d) - w);
            }
            Query::Get(x, y) => {
                let ans = seg2d.prod(..=x, ..=y);
                println!("{}", ans);
            }
        }
    }
}
