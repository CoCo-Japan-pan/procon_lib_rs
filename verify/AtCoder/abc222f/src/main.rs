// verification-helper: PROBLEM https://atcoder.jp/contests/abc222/tasks/abc222_f

use algebra::{Commutative, Monoid};
use proconio::{fastout, input, marker::Usize1};
use rerooting::{Rerootable, Rerooting};
use rustc_hash::FxHashMap;
use std::sync::OnceLock;

static EDGE_COST: OnceLock<FxHashMap<(usize, usize), u64>> = OnceLock::new();
static VERTEX_COST: OnceLock<Vec<u64>> = OnceLock::new();

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
pub struct DP {}
impl Rerootable for DP {
    type DPMonoid = MaxMonoid;
    fn add_root(
        subtree: &<Self::DPMonoid as Monoid>::Target,
        subtree_root: usize,
        new_root: usize,
    ) -> <Self::DPMonoid as Monoid>::Target {
        let min_v = subtree_root.min(new_root);
        let max_v = subtree_root.max(new_root);
        let edge_cost = EDGE_COST.get().unwrap().get(&(min_v, max_v)).unwrap();
        subtree.max(&VERTEX_COST.get().unwrap()[subtree_root]) + edge_cost
    }
    fn leaf(vertex: usize) -> <Self::DPMonoid as Monoid>::Target {
        VERTEX_COST.get().unwrap()[vertex]
    }
}

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
    EDGE_COST
        .set(
            a_b_c
                .into_iter()
                .map(|(a, b, c)| ((a.min(b), a.max(b)), c))
                .collect(),
        )
        .unwrap();
    VERTEX_COST.set(d).unwrap();
    let rerooted = Rerooting::<DP>::new(&graph);
    for i in 0..n {
        println!("{}", rerooted.get_ans(i));
    }
}
