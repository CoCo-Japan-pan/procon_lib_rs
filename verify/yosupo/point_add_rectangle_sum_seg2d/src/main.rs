// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum

use algebra::{Commutative, Monoid};
use proconio::{fastout, input};
use segtree_2d_compressed::SegTree2DCompressed;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add(i32, i32, i64),
    Prod(i32, i32, i32, i32),
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
        x_y_w: [(i32, i32, i64); n],
    }
    let queries = {
        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            input! {
                t: i32,
            }
            match t {
                0 => {
                    input! {
                        x: i32,
                        y: i32,
                        w: i64,
                    }
                    queries.push(Query::Add(x, y, w));
                }
                1 => {
                    input! {
                        l: i32,
                        d: i32,
                        r: i32,
                        u: i32,
                    }
                    queries.push(Query::Prod(l, d, r, u));
                }
                _ => unreachable!(),
            }
        }
        queries
    };
    let update_points = {
        let query_update = queries
            .iter()
            .filter(|q| matches!(q, Query::Add(..)))
            .count();
        let mut update_points = Vec::with_capacity(n + query_update);
        for (x, y, _) in x_y_w.iter() {
            update_points.push((*x, *y));
        }
        for query in queries.iter() {
            match query {
                Query::Add(x, y, _) => {
                    update_points.push((*x, *y));
                }
                _ => {}
            }
        }
        update_points
    };
    let mut seg2d = SegTree2DCompressed::<AddMonoid, _>::new(&update_points);
    for (x, y, w) in x_y_w {
        seg2d.add(x, y, w);
    }
    for query in queries {
        match query {
            Query::Add(x, y, w) => {
                seg2d.add(x, y, w);
            }
            Query::Prod(l, d, r, u) => {
                println!("{}", seg2d.prod(l..r, d..u));
            }
        }
    }
}
