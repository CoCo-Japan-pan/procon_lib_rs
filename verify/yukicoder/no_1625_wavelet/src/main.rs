// verification-helper: PROBLEM https://yukicoder.me/problems/no/1625

use algebra::{Commutative, Monoid};
use proconio::{fastout, input};
use wavelet_matrix_segtree::WaveletMatrixSegTree;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add(i64, i64, i64),
    Get(i64, i64),
}

#[derive(Clone, Copy, Debug)]
enum MaxMonoid {}
impl Monoid for MaxMonoid {
    type Target = i64;
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.max(b)
    }
    fn id_element() -> Self::Target {
        -1
    }
}
impl Commutative for MaxMonoid {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b_c_d_e_f: [(i64, i64, i64, i64, i64, i64); n],
    }
    let mut queries = a_b_c_d_e_f.into_iter().map(l_r_area).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            query_type: u8,
        }
        if query_type == 1 {
            input! {
                add: (i64, i64, i64, i64, i64, i64)
            }
            queries.push(l_r_area(add));
        } else {
            input! {
                l: i64,
                r: i64,
            }
            queries.push(Query::Get(l, r));
        }
    }
    let queries = queries;
    let x_y = {
        let mut x_y = queries
            .iter()
            .filter_map(|query| match query {
                Query::Add(x, y, _) => Some((*x, *y)),
                Query::Get(_, _) => None,
            })
            .collect::<Vec<_>>();
        x_y.sort_unstable();
        x_y.dedup();
        x_y
    };
    let init_weight = {
        let mut init_weight = vec![-1; x_y.len()];
        for query in queries.iter().take(n) {
            if let Query::Add(x, y, area) = query {
                let id = x_y.binary_search(&(*x, *y)).unwrap();
                init_weight[id] = init_weight[id].max(*area);
            }
        }
        init_weight
    };
    let y = x_y.iter().map(|(_, y)| *y).collect::<Vec<_>>();
    let sorted_y = {
        let mut sorted_y = y.clone();
        sorted_y.sort_unstable();
        sorted_y.dedup();
        sorted_y
    };
    let y = y
        .into_iter()
        .map(|y| sorted_y.binary_search(&y).unwrap())
        .collect::<Vec<_>>();
    let mut wm_seg = WaveletMatrixSegTree::<MaxMonoid>::from_weight(&y, &init_weight);
    for query in queries.into_iter().skip(n) {
        match query {
            Query::Add(x, y, area) => {
                let id = x_y.binary_search(&(x, y)).unwrap();
                let old_val = wm_seg.get_weight(id);
                wm_seg.set(id, old_val.max(area));
            }
            Query::Get(l, r) => {
                let xl = x_y.partition_point(|&(x, _)| x < l);
                let xr = x_y.partition_point(|&(x, _)| x <= r);
                let yl = sorted_y.partition_point(|&y| y < l);
                let yr = sorted_y.partition_point(|&y| y <= r);
                let ans = wm_seg.rect_sum_monoid(xl..xr, yl..yr);
                println!("{}", ans);
            }
        }
    }
}

fn l_r_area(x: (i64, i64, i64, i64, i64, i64)) -> Query {
    let x_list = [x.0, x.2, x.4];
    let left = *x_list.iter().min().unwrap();
    let right = *x_list.iter().max().unwrap();
    let y_list = [x.1, x.3, x.5];
    let x_list_parralel = x_list
        .into_iter()
        .map(|x| x - x_list[0])
        .collect::<Vec<_>>();
    let y_list_parralel = y_list
        .into_iter()
        .map(|y| y - y_list[0])
        .collect::<Vec<_>>();
    let area =
        (x_list_parralel[1] * y_list_parralel[2] - x_list_parralel[2] * y_list_parralel[1]).abs();
    Query::Add(left, right, area)
}
