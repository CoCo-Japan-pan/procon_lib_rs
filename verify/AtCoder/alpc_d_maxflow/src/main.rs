// verification-helper: PROBLEM https://atcoder.jp/contests/practice2/tasks/practice2_d
#![allow(non_snake_case)]
use itertools::iproduct;
use itertools::Itertools;
use maxflow::MaxFlow;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [Chars; N]
    }
    let id = |i: usize, j: usize| i * M + j;
    let mut mf = MaxFlow::new(N * M + 2);
    let start = N * M;
    let goal = N * M + 1;
    for (i, j) in iproduct!(0..N, 0..M) {
        if S[i][j] == '#' {
            continue;
        }
        if (i + j) % 2 == 0 {
            mf.add_edge(start, id(i, j), 1);
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0 || x >= N as i32 || y < 0 || y >= M as i32 {
                    continue;
                }
                if S[x as usize][y as usize] == '.' {
                    mf.add_edge(id(i, j), id(x as usize, y as usize), 1);
                }
            }
        } else {
            mf.add_edge(id(i, j), goal, 1);
        }
    }
    let flow = mf.flow(start, goal);
    println!("{}", flow);
    let mut ans = S;
    for edge in mf.edges() {
        if edge.from == start || edge.to == goal || edge.flow == 0 {
            continue;
        }
        let (i1, j1) = (edge.from / M, edge.from % M);
        let (i2, j2) = (edge.to / M, edge.to % M);
        if i1 == i2 {
            if j1 < j2 {
                ans[i1][j1] = '>';
                ans[i2][j2] = '<';
            } else {
                ans[i1][j1] = '<';
                ans[i2][j2] = '>';
            }
        } else {
            if i1 < i2 {
                ans[i1][j1] = 'v';
                ans[i2][j2] = '^';
            } else {
                ans[i1][j1] = '^';
                ans[i2][j2] = 'v';
            }
        }
    }
    for ans in ans.iter() {
        println!("{}", ans.iter().format(""));
    }
}
