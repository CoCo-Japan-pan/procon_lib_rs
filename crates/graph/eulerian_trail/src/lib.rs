//! [オイラー路の構築](https://kokiymgch.hatenablog.com/entry/2017/12/07/193238)

use std::vec;

/// 隣接リスト表現(行先頂点のみ)のグラフに対してオイラー路を求める。(疎なグラフを期待)  
/// (孤立点を除いて)連結であり、オイラー路が存在することが前提  
/// 頂点列を返す
pub fn eulerian_trail_from_vertex_list(
    start: usize,
    mut adj_vertex_list: Vec<Vec<usize>>,
    directed: bool,
) -> Vec<usize> {
    fn dfs(trail: &mut Vec<usize>, u: usize, adj_list: &mut Vec<Vec<usize>>, directed: bool) {
        while !adj_list[u].is_empty() {
            let v = adj_list[u].pop().unwrap();
            if !directed {
                let pos = adj_list[v].iter().rposition(|&x| x == u).unwrap();
                adj_list[v].swap_remove(pos);
            }
            dfs(trail, v, adj_list, directed);
        }
        trail.push(u);
    }
    let edge_count = if directed {
        adj_vertex_list.iter().map(|x| x.len()).sum::<usize>()
    } else {
        adj_vertex_list.iter().map(|x| x.len()).sum::<usize>() / 2
    };
    let mut trail = Vec::with_capacity(edge_count + 1);
    dfs(&mut trail, start, &mut adj_vertex_list, directed);
    trail.reverse();
    trail
}

/// 隣接リスト表現(行先頂点と、辺のindexのペア)のグラフに対してオイラー路を求める。(疎なグラフを期待)  
/// (孤立点を除いて)連結であり、オイラー路が存在することが前提  
/// (オイラー路の頂点列、辺のindex列)のタプルを返す
pub fn eulerian_trail_from_edge_list(
    start: usize,
    adj_edge_list: Vec<Vec<(usize, usize)>>,
    directed: bool,
) -> (Vec<usize>, Vec<usize>) {
    struct Env {
        vertex_trail: Vec<usize>,
        edge_trail: Vec<usize>,
        adj_edge_list: Vec<Vec<(usize, usize)>>,
        edge_stack: Vec<usize>,
        directed: bool,
    }
    fn dfs(env: &mut Env, u: usize) {
        while !env.adj_edge_list[u].is_empty() {
            let (v, edge_idx) = env.adj_edge_list[u].pop().unwrap();
            if !env.directed {
                let pos = env.adj_edge_list[v]
                    .iter()
                    .rposition(|&(x, e)| x == u && e == edge_idx)
                    .unwrap();
                env.adj_edge_list[v].swap_remove(pos);
            }
            env.edge_stack.push(edge_idx);
            dfs(env, v);
        }
        env.vertex_trail.push(u);
        if let Some(edge_idx) = env.edge_stack.pop() {
            env.edge_trail.push(edge_idx);
        }
    }
    let edge_cnt = if directed {
        adj_edge_list.iter().map(|x| x.len()).sum::<usize>()
    } else {
        adj_edge_list.iter().map(|x| x.len()).sum::<usize>() / 2
    };
    let mut env = Env {
        vertex_trail: Vec::with_capacity(edge_cnt + 1),
        edge_trail: Vec::with_capacity(edge_cnt),
        adj_edge_list,
        edge_stack: vec![],
        directed,
    };
    dfs(&mut env, start);
    env.vertex_trail.reverse();
    env.edge_trail.reverse();
    (env.vertex_trail, env.edge_trail)
}

/// 隣接行列表現のグラフに対してオイラー路を求める。(密なグラフを期待)  
/// `adj_matrix[u][v]` = u->vの辺の数  
/// (孤立点を除いて)連結であり、オイラー路が存在することが前提  
/// 頂点列を返す
pub fn eulerian_trail_from_matrix(
    start: usize,
    mut adj_matrix: Vec<Vec<usize>>,
    directed: bool,
) -> Vec<usize> {
    fn dfs(
        trail: &mut Vec<usize>,
        non_zero: &mut [usize],
        u: usize,
        adj_matrix: &mut Vec<Vec<usize>>,
        directed: bool,
    ) {
        // 既に消えた辺をスキップするためにnon_zeroを導入
        let mut v = non_zero[u];
        while v < adj_matrix.len() {
            for _ in 0..adj_matrix[u][v] {
                adj_matrix[u][v] -= 1;
                if !directed {
                    adj_matrix[v][u] -= 1;
                }
                dfs(trail, non_zero, v, adj_matrix, directed);
            }
            non_zero[u] = non_zero[u].max(v + 1);
            v = non_zero[u];
        }
        trail.push(u);
    }
    let edge_cnt = if directed {
        adj_matrix
            .iter()
            .map(|x| x.iter().sum::<usize>())
            .sum::<usize>()
    } else {
        adj_matrix
            .iter()
            .map(|x| x.iter().sum::<usize>())
            .sum::<usize>()
            / 2
    };
    let mut trail = Vec::with_capacity(edge_cnt + 1);
    let mut non_zero = vec![0; adj_matrix.len()];
    dfs(&mut trail, &mut non_zero, start, &mut adj_matrix, directed);
    trail.reverse();
    trail
}
