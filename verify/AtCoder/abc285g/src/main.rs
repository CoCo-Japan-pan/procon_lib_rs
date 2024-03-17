// verification-helper: PROBLEM https://atcoder.jp/contests/abc285/tasks/abc285_g

#![allow(non_snake_case)]
use itertools::iproduct;
use maxflow_lower_bound::MaxFlowLowerBound;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        C: [Chars; H],
    }
    let mut mf = MaxFlowLowerBound::new(H * W + 2);
    let start = H * W;
    let goal = H * W + 1;
    let id = |i: usize, j: usize| i * W + j;
    // 2と?のある市松模様で2部グラフを作る 2は最低容量1の制約付き
    for (i, j) in iproduct!(0..H, 0..W) {
        if C[i][j] == '1' {
            continue;
        }
        if (i + j) % 2 == 0 {
            if C[i][j] == '2' {
                mf.add_edge_with_lower_bound(start, id(i, j), 1..=1);
            } else {
                mf.add_edge(start, id(i, j), 1);
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (ni, nj) = (i as i32 + dx, j as i32 + dy);
                if ni < 0 || ni >= H as i32 || nj < 0 || nj >= W as i32 {
                    continue;
                }
                if C[ni as usize][nj as usize] == '1' {
                    continue;
                }
                mf.add_edge(id(i, j), id(ni as usize, nj as usize), 1);
            }
        } else {
            if C[i][j] == '2' {
                mf.add_edge_with_lower_bound(id(i, j), goal, 1..=1);
            } else {
                mf.add_edge(id(i, j), goal, 1);
            }
        }
    }
    // 最小流量制限を満たせるならYesを出力
    if mf.flow(start, goal).is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
