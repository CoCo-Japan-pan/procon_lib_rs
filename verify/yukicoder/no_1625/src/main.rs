// verification-helper: PROBLEM https://yukicoder.me/problems/no/1625

use algebra::{Commutative, Monoid};
use proconio::{fastout, input};
use segtree_2d_compressed::SegTree2DCompressed;

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
            query_type: i64,
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
    let update_queries = queries
        .iter()
        .filter_map(|query| match query {
            Query::Add(l, r, _) => Some((*l, *r)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut segtree = SegTree2DCompressed::<MaxMonoid, i64>::new(&update_queries);
    for query in queries {
        match query {
            Query::Add(l, r, area) => segtree.add(l, r, area),
            Query::Get(l, r) => println!("{}", segtree.prod(l..=r, l..=r)),
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
