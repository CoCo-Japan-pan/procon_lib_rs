// https://atcoder.jp/contests/abc384/tasks/abc384_g

#![allow(non_snake_case)]
use fenwick_tree::FenwickTree;
use mo::calc_mo_friendly_order;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N],
        B: [i64; N],
        K: usize,
        X_Y: [(usize, usize); K],
    }
    let order = calc_mo_friendly_order(&X_Y, N, N);
    let mut answer = vec![0; K];
    let mut cur_answer = 0;
    let mut a_data = MyData::new(A);
    let mut b_data = MyData::new(B);
    let a_to_b = a_data
        .list
        .iter()
        .map(|&x| b_data.sorted.partition_point(|&b| b < x))
        .collect::<Vec<_>>();
    let b_to_a = b_data
        .list
        .iter()
        .map(|&x| a_data.sorted.partition_point(|&a| a < x))
        .collect::<Vec<_>>();
    let (mut x, mut y) = (0, 0);
    let diff = |add_num: i64, idx: usize, data: &MyData| -> i64 {
        let mut ret = 0;
        let (sum, cnt) = data.sum_cnt_lt(idx);
        ret += add_num * cnt - sum;
        let (sum, cnt) = data.sum_cnt_ge(idx);
        ret += sum - add_num * cnt;
        ret
    };
    for id in order {
        let (nx, ny) = X_Y[id];
        while x < nx {
            a_data.add(x);
            let num = a_data.list[x];
            let idx = a_to_b[x];
            cur_answer += diff(num, idx, &b_data);
            x += 1;
        }
        while x > nx {
            x -= 1;
            a_data.remove(x);
            let num = a_data.list[x];
            let idx = a_to_b[x];
            cur_answer -= diff(num, idx, &b_data);
        }
        while y < ny {
            b_data.add(y);
            let num = b_data.list[y];
            let idx = b_to_a[y];
            cur_answer += diff(num, idx, &a_data);
            y += 1;
        }
        while y > ny {
            y -= 1;
            b_data.remove(y);
            let num = b_data.list[y];
            let idx = b_to_a[y];
            cur_answer -= diff(num, idx, &a_data);
        }
        answer[id] = cur_answer;
    }
    for a in answer {
        println!("{}", a);
    }
}

/// 特定の値以下の値達の和と個数と、また全体の和と個数が求まる構造体
struct MyData {
    list: Vec<i64>,
    sorted: Vec<i64>,
    id_to_sorted_id: Vec<usize>,
    ft_cnt: FenwickTree<i64>,
    ft_sum: FenwickTree<i64>,
}

impl MyData {
    fn new(list: Vec<i64>) -> Self {
        let mut sorted = list.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let id_to_sorted_id = list
            .iter()
            .map(|&x| sorted.binary_search(&x).unwrap())
            .collect();
        let n = sorted.len();
        let ft_cnt = FenwickTree::new(n, 0_i64);
        let ft_sum = FenwickTree::new(n, 0_i64);
        Self {
            list,
            sorted,
            id_to_sorted_id,
            ft_cnt,
            ft_sum,
        }
    }

    /// list[idx]を追加する
    fn add(&mut self, idx: usize) {
        let num = self.list[idx];
        let idx = self.id_to_sorted_id[idx];
        self.ft_cnt.add(idx, 1);
        self.ft_sum.add(idx, num);
    }

    /// list[idx]を削除する
    fn remove(&mut self, idx: usize) {
        let num = self.list[idx];
        let idx = self.id_to_sorted_id[idx];
        self.ft_cnt.add(idx, -1);
        self.ft_sum.add(idx, -num);
    }

    /// ...idxの値の和と個数を返す
    fn sum_cnt_lt(&self, idx: usize) -> (i64, i64) {
        let sum = self.ft_sum.sum(..idx);
        let cnt = self.ft_cnt.sum(..idx);
        (sum, cnt)
    }

    /// idx..の値の和と個数を返す
    fn sum_cnt_ge(&self, idx: usize) -> (i64, i64) {
        let (sum, cnt) = self.sum_cnt_lt(idx);
        let all_sum = self.ft_sum.sum(..);
        let all_cnt = self.ft_cnt.sum(..);
        (all_sum - sum, all_cnt - cnt)
    }
}
