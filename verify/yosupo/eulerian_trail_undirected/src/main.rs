// https://judge.yosupo.jp/problem/eulerian_trail_undirected
use eulerian_trail::eulerian_trail_from_edge_list;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        let ans = solve();
        if let Some((vertex_trail, edge_trail)) = ans {
            println!("Yes");
            for v in vertex_trail {
                print!("{} ", v);
            }
            println!();
            for e in edge_trail {
                print!("{} ", e);
            }
            println!();
        } else {
            println!("No");
        }
    }
}

fn solve() -> Option<(Vec<usize>, Vec<usize>)> {
    input! {
        n: usize,
        m: usize,
        u_v: [(usize, usize); m],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for (id, &(u, v)) in u_v.iter().enumerate() {
            graph[u].push((v, id));
            graph[v].push((u, id));
        }
        graph
    };
    // まず次数チェック
    let odd_v = (0..n)
        .filter(|&v| graph[v].len() % 2 == 1)
        .collect::<Vec<_>>();
    if odd_v.len() != 0 && odd_v.len() != 2 {
        return None;
    }
    let start = if odd_v.len() == 2 {
        odd_v[0]
    } else {
        graph.iter().position(|x| x.len() > 0).unwrap_or(0)
    };
    // 連結チェック
    {
        let mut visited = vec![false; n];
        let mut que = VecDeque::new();
        que.push_back(start);
        visited[start] = true;
        while let Some(u) = que.pop_front() {
            for &(v, _) in &graph[u] {
                if !visited[v] {
                    visited[v] = true;
                    que.push_back(v);
                }
            }
        }
        for v in 0..n {
            if !visited[v] && graph[v].len() > 0 {
                return None;
            }
        }
    }
    Some(eulerian_trail_from_edge_list(start, graph, false))
}
