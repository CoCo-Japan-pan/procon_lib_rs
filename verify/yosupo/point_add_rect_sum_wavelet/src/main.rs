// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum
use algebra::{Commutative, Group, Monoid};
use proconio::{fastout, input};
use wavelet_matrix_segtree::WaveletMatrixSegTree;

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
    let (x, (y, w)): (Vec<_>, (Vec<_>, Vec<_>)) = {
        let add_cnt = queries
            .iter()
            .filter(|q| matches!(q, Query::Add(..)))
            .count();
        let mut offline_x_y_w = x_y_w.clone();
        offline_x_y_w.reserve(add_cnt);
        for &q in &queries {
            if let Query::Add(x, y, _) = q {
                offline_x_y_w.push((x, y, 0));
            }
        }
        offline_x_y_w.sort_by_key(|(x, y, _)| (*x, *y));
        offline_x_y_w
            .into_iter()
            .map(|(a, b, c)| (a, (b, c)))
            .unzip()
    };
    let sorted_y = {
        let mut sorted_y = y.clone();
        sorted_y.sort_unstable();
        sorted_y.dedup();
        sorted_y
    };
    let y: Vec<usize> = y
        .into_iter()
        .map(|y| sorted_y.binary_search(&y).unwrap())
        .collect();
    let mut wm_seg = WaveletMatrixSegTree::<AddGroup>::from_weight(&y, &w);
    let x_y: Vec<(i32, usize)> = x.into_iter().zip(y).collect();
    for q in queries {
        match q {
            Query::Add(x, y, w) => {
                let y = sorted_y.binary_search(&y).unwrap();
                let id = x_y.binary_search(&(x, y)).unwrap();
                let old_weight = wm_seg.get_weight(id);
                wm_seg.set(id, w + old_weight);
            }
            Query::Prod(xl, yl, xr, yr) => {
                let xl = x_y.partition_point(|(x, _)| *x < xl);
                let xr = x_y.partition_point(|(x, _)| *x < xr);
                let yl = sorted_y.partition_point(|y| *y < yl);
                let yr = sorted_y.partition_point(|y| *y < yr);
                let ans = wm_seg.rect_sum_group(xl..xr, yl..yr);
                println!("{}", ans);
            }
        }
    }
}
