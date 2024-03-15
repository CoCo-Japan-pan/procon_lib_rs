// verification-helper: PROBLEM https://yukicoder.me/problems/no/649

use fenwick_tree::FenwickTree;
use proconio::{fastout, input};

#[derive(Debug, Clone, Copy)]
enum Query {
    Add(u64),
    Ask,
}

#[fastout]
fn main() {
    input! {
        q: usize,
        k: i32,
    }
    let mut queries = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    a: u64,
                }
                queries.push(Query::Add(a));
            }
            2 => {
                queries.push(Query::Ask);
            }
            _ => unreachable!(),
        }
    }
    let queries = queries;
    let mut num_list = queries
        .iter()
        .filter_map(|q| {
            if let Query::Add(a) = q {
                Some(*a)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    num_list.sort();
    num_list.dedup();
    let num_list = num_list;
    let mut fenwick = FenwickTree::<i32>::new(num_list.len());
    let mut sum = 0;
    for query in queries {
        match query {
            Query::Add(a) => {
                let idx = num_list.binary_search(&a).unwrap();
                fenwick.add(idx, 1);
                sum += 1;
            }
            Query::Ask => {
                if sum < k {
                    println!("-1");
                } else {
                    let id = fenwick.lower_bound(k);
                    println!("{}", num_list[id]);
                    fenwick.add(id, -1);
                    sum -= 1;
                }
            }
        }
    }
}
