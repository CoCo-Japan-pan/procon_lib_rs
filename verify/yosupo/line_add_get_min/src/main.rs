// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min

use cht_offline::{CHTOffline, MinCompare};
use proconio::{fastout, input};

#[derive(Copy, Clone)]
enum Query {
    AddLine(i64, i64),
    GetMin(i64),
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a_b: [(i64, i64); n],
    }
    let queries = {
        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            input! {
                t: u8,
            }
            if t == 0 {
                input! {
                    a: i64,
                    b: i64,
                }
                queries.push(Query::AddLine(a, b));
            } else {
                input! {
                    x: i64,
                }
                queries.push(Query::GetMin(x));
            }
        }
        queries
    };
    let points = queries
        .iter()
        .filter_map(|q| match q {
            Query::AddLine(..) => None,
            Query::GetMin(x) => Some(*x),
        })
        .collect::<Vec<_>>();
    let mut cht = CHTOffline::<MinCompare>::new(points);
    for (a, b) in a_b {
        cht.add_line(a, b);
    }
    for q in queries {
        match q {
            Query::AddLine(a, b) => cht.add_line(a, b),
            Query::GetMin(x) => println!("{}", cht.get(x)),
        }
    }
}
