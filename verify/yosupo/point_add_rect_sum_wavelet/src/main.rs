// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum
use algebra::{Commutative, Group, Monoid};
use proconio::{fastout, input};
use wavelet_matrix_segtree::WMSegWrapper;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add(i32, i32, i64),
    Prod(i32, i32, i32, i32),
}

#[derive(Debug)]
struct AddGroup;
impl Monoid for AddGroup {
    type Target = i64;
    fn id_element() -> Self::Target {
        0
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a + *b
    }
}
impl Commutative for AddGroup {}
impl Group for AddGroup {
    fn inverse(a: &Self::Target) -> Self::Target {
        -a
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut x_y_w: [(i32, i32, i64); n],
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
    let update_points = x_y_w
        .iter()
        .map(|&(x, y, _)| (x, y))
        .chain(queries.iter().filter_map(|q| match q {
            Query::Add(x, y, _) => Some((*x, *y)),
            Query::Prod(..) => None,
        }))
        .collect::<Vec<_>>();
    x_y_w.sort_unstable();
    x_y_w.dedup_by(|(x1, y1, w1), (x2, y2, w2)| {
        (x1, y1) == (x2, y2) && {
            *w2 += *w1;
            true
        }
    });
    let mut wm_seg = WMSegWrapper::<AddGroup, _>::from_weight(update_points, &x_y_w);
    for q in queries {
        match q {
            Query::Add(x, y, w) => {
                let prev = wm_seg.get(x, y);
                wm_seg.set(x, y, prev + w);
            }
            Query::Prod(xl, yl, xr, yr) => {
                let ans = wm_seg.rect_sum_group(xl..xr, yl..yr);
                println!("{}", ans);
            }
        }
    }
}
