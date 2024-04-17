// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use fenwick_tree::FenwickTree;
use mo::{MoFuncs, MoRunner};
use proconio::{fastout, input};

struct MoStates {
    compressed_a: Vec<usize>,
    ft: FenwickTree<i64>,
    ans: Vec<i64>,
    cur_inv: i64,
}

impl MoFuncs for MoStates {
    fn add_left(&mut self, left: usize) {
        let num = self.compressed_a[left];
        self.cur_inv += self.ft.sum(..num);
        self.ft.add(num, 1);
    }
    fn add_right(&mut self, right: usize) {
        let num = self.compressed_a[right];
        self.cur_inv += self.ft.sum(num + 1..);
        self.ft.add(num, 1);
    }
    fn remove_left(&mut self, left: usize) {
        let num = self.compressed_a[left];
        self.cur_inv -= self.ft.sum(..num);
        self.ft.add(num, -1);
    }
    fn remove_right(&mut self, right: usize) {
        let num = self.compressed_a[right];
        self.cur_inv -= self.ft.sum(num + 1..);
        self.ft.add(num, -1);
    }
    fn memo(&mut self, id: usize) {
        self.ans[id] = self.cur_inv;
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
    let mo_runner = MoRunner::new(n, l_r);
    mo_runner.run(&mut mo_state);
    for ans in mo_state.ans {
        println!("{}", ans);
    }
}
