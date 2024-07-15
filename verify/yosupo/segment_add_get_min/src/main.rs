// verification-helper: PROBLEM https://judge.yosupo.jp/problem/segment_add_get_min

use cht_offline::{CHTOffline, MinCompare};
use proconio::{fastout, input};

enum Query {
    AddSegment(i64, i64, i64, i64),
    GetMin(i64),
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        l_r_a_b: [(i64, i64, i64, i64); n],
    }
    let queries = {
        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            input! { t: i64 }
            match t {
                0 => {
                    input! { l: i64, r: i64, a: i64, b: i64 }
                    queries.push(Query::AddSegment(l, r, a, b));
                }
                1 => {
                    input! { x: i64 }
                    queries.push(Query::GetMin(x));
                }
                _ => unreachable!(),
            }
        }
        queries
    };
    let points = queries
        .iter()
        .filter_map(|q| match q {
            Query::AddSegment(..) => None,
            Query::GetMin(x) => Some(*x),
        })
        .collect::<Vec<_>>();
    let mut cht = CHTOffline::<MinCompare>::new(points);
    for (l, r, a, b) in l_r_a_b {
        cht.add_line_segment(a, b, l..r);
    }
    for query in queries {
        match query {
            Query::AddSegment(l, r, a, b) => cht.add_line_segment(a, b, l..r),
            Query::GetMin(x) => {
                let ans = cht.get(x);
                if ans == i64::MAX {
                    println!("INFINITY");
                } else {
                    println!("{}", ans);
                }
            }
        }
    }
}
