// https://judge.yosupo.jp/problem/eulerian_trail_directed
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
        }
        graph
    };
    // まず次数チェック
    let (in_deg, out_deg) = {
        let mut in_deg = vec![0_usize; n];
        let mut out_deg = vec![0_usize; n];
        for &(u, v) in u_v.iter() {
            out_deg[u] += 1;
            in_deg[v] += 1;
        }
        (in_deg, out_deg)
    };
    // 差が2以上の頂点がある場合はオイラー路は存在しない
    if in_deg
        .iter()
        .zip(out_deg.iter())
        .any(|(&in_d, &out_d)| in_d.abs_diff(out_d) > 1)
    {
        return None;
    }
    let plus_one_v = in_deg
        .iter()
        .zip(out_deg.iter())
        .enumerate()
        .filter_map(
            |(v, (&in_d, &out_d))| {
                if in_d + 1 == out_d {
                    Some(v)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();
    let minus_one_v = in_deg
        .iter()
        .zip(out_deg.iter())
        .enumerate()
        .filter_map(
            |(v, (&in_d, &out_d))| {
                if in_d == out_d + 1 {
                    Some(v)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();
    if (plus_one_v.is_empty() && minus_one_v.is_empty())
        || (plus_one_v.len() == 1 && minus_one_v.len() == 1)
    {
        // 連結性チェック
        let start = if plus_one_v.is_empty() {
            out_deg.iter().position(|&d| d > 0).unwrap_or(0)
        } else {
            plus_one_v[0]
        };
        {
            let mut visited = vec![false; n];
            let mut que = VecDeque::new();
            que.push_back(start);
            visited[start] = true;
            while let Some(u) = que.pop_front() {
                for &(v, _) in graph[u].iter() {
                    if !visited[v] {
                        visited[v] = true;
                        que.push_back(v);
                    }
                }
            }
            for v in 0..n {
                if !visited[v] && (in_deg[v] > 0 || out_deg[v] > 0) {
                    return None;
                }
            }
        }
        let (vertex_trail, edge_trail) = eulerian_trail_from_edge_list(start, graph, true);
        Some((vertex_trail, edge_trail))
    } else {
        None
    }
}
