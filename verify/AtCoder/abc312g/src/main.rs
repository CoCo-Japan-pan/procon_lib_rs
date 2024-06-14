// verification-helper: PROBLEM https://atcoder.jp/contests/abc312/tasks/abc312_g

use algebra::{Commutative, Monoid};
use proconio::{fastout, input, marker::Usize1};
use rerooting::Rerooting;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DP {
    prod: i64,
    sum: i64,
}
impl Monoid for DP {
    type Target = Self;
    fn id_element() -> Self::Target {
        DP { prod: 0, sum: 0 }
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        DP {
            prod: a.prod + b.prod + a.sum * b.sum,
            sum: a.sum + b.sum,
        }
    }
}
impl Commutative for DP {}

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(a, b) in &a_b {
        graph[a].push(b);
        graph[b].push(a);
    }
    let add_root = |subtree: &DP, _subtree_root: usize, _new_root: usize| DP {
        prod: 0,
        sum: subtree.sum + 1,
    };
    let rerooted = Rerooting::<DP, _>::new(&graph, &add_root);
    let path: i64 = (0..n).map(|i| rerooted.get_ans(i).prod).sum();
    let ans = (n as i64) * (n as i64 - 1) * (n as i64 - 2) / 6 - path;
    println!("{}", ans);
}
