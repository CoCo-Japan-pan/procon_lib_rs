// https://atcoder.jp/contests/abc348/tasks/abc348_e

use algebra::{Commutative, Monoid};
use proconio::{input, marker::Usize1};
use rerooting::Rerooting;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DP {
    ans: i64,
    c_sum: i64,
}
impl Monoid for DP {
    type Target = Self;
    fn id_element() -> Self::Target {
        DP { ans: 0, c_sum: 0 }
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        DP {
            ans: a.ans + b.ans,
            c_sum: a.c_sum + b.c_sum,
        }
    }
}
impl Commutative for DP {}

fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
        c: [i64; n],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for (a, b) in a_b {
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    };
    let reroot = Rerooting::<DP, _>::new(&graph, |dp, subtree_root, _new_root| DP {
        ans: dp.ans + dp.c_sum + c[subtree_root],
        c_sum: dp.c_sum + c[subtree_root],
    });
    let ans = (0..n).map(|i| reroot.get_ans(i).ans).min().unwrap();
    println!("{}", ans);
}
