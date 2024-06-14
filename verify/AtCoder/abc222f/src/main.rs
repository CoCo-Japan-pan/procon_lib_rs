// verification-helper: PROBLEM https://atcoder.jp/contests/abc222/tasks/abc222_f

use algebra::{Commutative, Monoid};
use proconio::{fastout, input, marker::Usize1};
use rerooting::Rerooting;
use rustc_hash::FxHashMap;

pub struct MaxMonoid {}
impl Monoid for MaxMonoid {
    type Target = u64;
    fn id_element() -> Self::Target {
        0
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.max(b)
    }
}
impl Commutative for MaxMonoid {}

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b_c: [(Usize1, Usize1, u64); n - 1],
        d: [u64; n],
    }
    let mut graph = vec![vec![]; n];
    for (a, b, _) in &a_b_c {
        graph[*a].push(*b);
        graph[*b].push(*a);
    }
    let edge_cost: FxHashMap<(usize, usize), u64> = a_b_c
        .into_iter()
        .map(|(a, b, c)| ((a.min(b), a.max(b)), c))
        .collect();
    let add_root = |subtree: &u64, subtree_root: usize, new_root: usize| {
        let edge_cost = edge_cost
            .get(&(subtree_root.min(new_root), subtree_root.max(new_root)))
            .unwrap();
        subtree.max(&d[subtree_root]) + edge_cost
    };
    let rerooted = Rerooting::new(&graph, add_root, MaxMonoid {});
    for i in 0..n {
        println!("{}", rerooted.get_ans(i));
    }
}
