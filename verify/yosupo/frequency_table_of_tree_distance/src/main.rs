// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance

use capture::crecurse;
use centroid_decomposition::CentroidDecomposition;
use ntt::convolution_i64;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n - 1],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for &(a, b) in &a_b {
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    };
    let mut ans = vec![0_i64; n];
    let func = |used: &[bool], centroid: usize| {
        // 深さを求める
        let mut max_depth = 0;
        crecurse!(
            unsafe fn dfs(v: usize, p: usize, d: usize) {
                max_depth = max_depth.max(d);
                for &to in &graph[v] {
                    if to == p || used[to] {
                        continue;
                    }
                    dfs!(to, v, d + 1);
                }
            }
        )(centroid, n, 0);
        let max_depth = n.min(max_depth * 2 + 1);
        // 各部分木
        let mut subtrees = vec![];
        for &nv in &graph[centroid] {
            if used[nv] {
                continue;
            }
            let mut subtree = vec![0];
            crecurse!(
                unsafe fn dfs(v: usize, p: usize, d: usize) {
                    if subtree.len() == d {
                        subtree.push(1);
                    } else {
                        subtree[d] += 1;
                    }
                    for &to in &graph[v] {
                        if to == p || used[to] {
                            continue;
                        }
                        dfs!(to, v, d + 1);
                    }
                }
            )(nv, centroid, 1);
            subtrees.push(subtree);
        }
        // eprintln!("centroid: {}, subtrees: {:?}", centroid, subtrees);
        // 各部分木の和の二乗
        let mut sum_square = vec![0_i64; max_depth];
        sum_square[0] = 1;
        // 各部分木の二乗の和
        let mut square_sum = vec![0_i64; max_depth];
        for subtree in subtrees {
            let square = convolution_i64(&subtree, &subtree);
            for (i, s) in subtree.into_iter().enumerate().take(max_depth) {
                sum_square[i] += s;
            }
            for (i, s) in square.into_iter().enumerate().take(max_depth) {
                square_sum[i] += s;
            }
        }
        sum_square = convolution_i64(&sum_square, &sum_square);
        for i in 0..max_depth {
            ans[i] += (sum_square[i] - square_sum[i]) / 2;
        }
        // eprintln!("ans: {:?}", ans);
    };
    let cd = CentroidDecomposition::new(&graph);
    cd.run(func);
    for i in 1..n {
        print!("{}{}", ans[i], if i == n - 1 { '\n' } else { ' ' });
    }
}
