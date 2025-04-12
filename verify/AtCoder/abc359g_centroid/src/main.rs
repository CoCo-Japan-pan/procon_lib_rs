// https://atcoder.jp/contests/abc359/tasks/abc359_g
use capture::crecurse;
use centroid_decomposition::CentroidDecomposition;
use proconio::{fastout, input, marker::Usize1};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        u_v: [(Usize1, Usize1); n - 1],
        color_list: [Usize1; n],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for &(u, v) in &u_v {
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    };
    let cd = CentroidDecomposition::new(&graph);
    let mut ans = 0_usize;
    let func = |used: &[bool], centroid: usize| {
        // 深さの和と個数のペアを色ごとに持つ
        let mut map = FxHashMap::default();
        // 重心をまたぐ寄与
        for &chd in &graph[centroid] {
            if used[chd] {
                continue;
            }
            let mut sub_map = FxHashMap::default();
            crecurse!(
                unsafe fn dfs(v: usize, p: usize, depth: usize) {
                    sub_map
                        .entry(color_list[v])
                        .and_modify(|(d, c)| {
                            *d += depth;
                            *c += 1;
                        })
                        .or_insert((depth, 1));
                    for &u in &graph[v] {
                        if u == p || used[u] {
                            continue;
                        }
                        dfs!(u, v, depth + 1);
                    }
                }
            )(chd, centroid, 1);
            for (color, (sum, cnt)) in sub_map.into_iter() {
                if let Some(&(old_sum, old_cnt)) = map.get(&color) {
                    ans += sum * old_cnt + cnt * old_sum;
                }
                map.entry(color)
                    .and_modify(|(d, c)| {
                        *d += sum;
                        *c += cnt;
                    })
                    .or_insert((sum, cnt));
            }
        }
        // 重心を用いる寄与
        if let Some(&(sum, _)) = map.get(&color_list[centroid]) {
            ans += sum;
        }
    };
    cd.run(func);
    println!("{}", ans);
}
