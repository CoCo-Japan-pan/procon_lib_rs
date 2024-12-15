// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use fenwick_tree::FenwickTree;
use mo::{MoFuncs, MoRunner};
use proconio::{fastout, input};

struct MoStates {
    compressed_a: Vec<usize>,
    ft: FenwickTree<i64>,
    cur_inv: i64,
    ans: Vec<i64>,
}

impl MoFuncs for MoStates {
    fn x_minus(&mut self, x: usize) {
        let num = self.compressed_a[x - 1];
        self.cur_inv += self.ft.sum(..num);
        self.ft.add(num, 1);
    }
    fn y_plus(&mut self, y: usize) {
        let num = self.compressed_a[y];
        self.cur_inv += self.ft.sum(num + 1..);
        self.ft.add(num, 1);
    }
    fn x_plus(&mut self, x: usize) {
        let num = self.compressed_a[x];
        self.cur_inv -= self.ft.sum(..num);
        self.ft.add(num, -1);
    }
    fn y_minus(&mut self, y: usize) {
        let num = self.compressed_a[y - 1];
        self.cur_inv -= self.ft.sum(num + 1..);
        self.ft.add(num, -1);
    }
    fn memo(&mut self, idx: usize) {
        self.ans[idx] = self.cur_inv;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        l_r: [(usize, usize); q],
    }
    let a = {
        let mut a_cpy = a.clone();
        a_cpy.sort();
        a_cpy.dedup();
        let mut ret = vec![0; a.len()];
        for (r, a) in ret.iter_mut().zip(a) {
            *r = a_cpy.binary_search(&a).unwrap();
        }
        ret
    };
    let mut mo_state = MoStates {
        ft: FenwickTree::new(a.len(), 0),
        compressed_a: a,
        ans: vec![0; q],
        cur_inv: 0,
    };
    let mo_runner = MoRunner::new(&l_r, n, n);
    mo_runner.run(&mut mo_state);
    for ans in mo_state.ans {
        println!("{}", ans);
    }
}
