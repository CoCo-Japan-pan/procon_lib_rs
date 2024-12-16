// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum
use proconio::{fastout, input};
use wavelet_matrix_fenwick::WMFenwickWrapper;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add(i32, i32, i64),
    Prod(i32, i32, i32, i32),
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
    let update_points = x_y_w
        .iter()
        .map(|&(x, y, _)| (x, y))
        .chain(queries.iter().filter_map(|q| match q {
            Query::Add(x, y, _) => Some((*x, *y)),
            Query::Prod(..) => None,
        }))
        .collect::<Vec<_>>();
    let mut wm_seg = WMFenwickWrapper::from_weight(update_points, &x_y_w);
    for q in queries {
        match q {
            Query::Add(x, y, w) => {
                wm_seg.add(x, y, w);
            }
            Query::Prod(xl, yl, xr, yr) => {
                let ans = wm_seg.rect_sum(xl..xr, yl..yr);
                println!("{}", ans);
            }
        }
    }
}
