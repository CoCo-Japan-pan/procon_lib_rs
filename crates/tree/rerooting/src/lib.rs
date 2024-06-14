//! 全方位木DP  
//! 辺が向きつきで行きと帰りで異なる場合に対応しづらいので、ここでは頂点を用いて表している  
//! 従って辺のコストとかは外でhashmap等で管理することになる  

use algebra::{Commutative, Monoid};

#[derive(Debug)]
pub struct Rerooting<T: Monoid + Commutative> {
    vertex_cnt: usize,
    /// 根を0とした場合の各頂点を根とする部分木のDPテーブル
    subtree_memo: Vec<T::Target>,
    /// 各頂点を根とした木全体のDPテーブル
    ans: Vec<T::Target>,
}

impl<T: Monoid + Commutative> Rerooting<T> {
    /// モノイド`T`は`add_root`によりできた「部分木+一辺」同士をmergeする関数を二項演算として持つ  
    /// `add_root(subtree: T::Target, subtree_root: usize, new_root: usize) -> T::Target`  
    /// 部分木に頂点 subtree_root → new_root の辺を追加する
    pub fn new(
        graph: &Vec<Vec<usize>>,
        add_root: &impl Fn(&T::Target, usize, usize) -> T::Target,
    ) -> Self {
        let vertex_cnt = graph.len();
        let subtree_memo = vec![T::id_element(); vertex_cnt];
        let ans = vec![T::id_element(); vertex_cnt];
        let mut ret = Self {
            vertex_cnt,
            subtree_memo,
            ans,
        };
        ret.dfs(graph, 0, usize::MAX, add_root);
        ret.bfs(graph, 0, usize::MAX, T::id_element(), add_root);
        ret
    }

    pub fn get_ans(&self, root: usize) -> T::Target {
        assert!(root < self.vertex_cnt);
        self.ans[root].clone()
    }

    fn dfs(
        &mut self,
        graph: &Vec<Vec<usize>>,
        v: usize,
        p: usize,
        add_root: &impl Fn(&T::Target, usize, usize) -> T::Target,
    ) {
        for &to in &graph[v] {
            if to == p {
                continue;
            }
            self.dfs(graph, to, v, add_root);
            let memo = add_root(&self.subtree_memo[to], to, v);
            self.subtree_memo[v] = T::binary_operation(&self.subtree_memo[v], &memo);
        }
    }

    fn bfs(
        &mut self,
        graph: &Vec<Vec<usize>>,
        v: usize,
        p: usize,
        par_val: T::Target,
        add_root: &impl Fn(&T::Target, usize, usize) -> T::Target,
    ) {
        // 左右から累積和を取っておく
        let mut buf = Vec::with_capacity(graph[v].len());
        for &to in &graph[v] {
            if to == p {
                continue;
            } else {
                buf.push(add_root(&self.subtree_memo[to], to, v));
            }
        }
        let mut left_sum = vec![T::id_element(); buf.len() + 1];
        let mut right_sum = vec![T::id_element(); buf.len() + 1];
        for i in 0..buf.len() {
            left_sum[i + 1] = T::binary_operation(&left_sum[i], &buf[i]);
        }
        for i in (0..buf.len()).rev() {
            right_sum[i] = T::binary_operation(&buf[i], &right_sum[i + 1]);
        }
        if p == usize::MAX {
            self.ans[v] = left_sum.last().unwrap().clone();
        } else {
            self.ans[v] = T::binary_operation(left_sum.last().unwrap(), &par_val);
        }

        // 子に伝播
        for (i, &to) in graph[v].iter().filter(|v| **v != p).enumerate() {
            // 一つも部分木をmergeしないなら、leafを用いる
            let propagate = T::binary_operation(
                &par_val,
                &T::binary_operation(&left_sum[i], &right_sum[i + 1]),
            );
            let par_val = add_root(&propagate, v, to);
            self.bfs(graph, to, v, par_val, add_root);
        }
    }
}
