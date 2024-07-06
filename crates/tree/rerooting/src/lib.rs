//! 全方位木DP  
//! 辺が向きつきで行きと帰りで異なる場合に対応しづらいので、ここでは頂点を用いて表している  
//! 従って辺のコストとかは外でhashmap等で管理することになる  

use algebra::{Commutative, Monoid};

#[derive(Debug)]
pub struct Rerooting<M: Monoid + Commutative, F: FnMut(&M::Target, usize, usize) -> M::Target> {
    vertex_cnt: usize,
    /// 根を0とした場合の各頂点を根とする部分木のDPテーブル
    subtree_memo: Vec<M::Target>,
    /// 各頂点を根とした木全体のDPテーブル
    ans: Vec<M::Target>,
    add_root: F,
}

impl<M: Monoid + Commutative, F: FnMut(&M::Target, usize, usize) -> M::Target> Rerooting<M, F> {
    /// モノイド`M`は`add_root`によりできた「部分木+一辺」同士をmergeする関数を二項演算として持つ  
    /// 葉にはモノイドの単位元が入る  
    ///
    /// `add_root(subtree: &M::Target, subtree_root: usize, new_root: usize) -> M::Target`  
    /// `add_root`は部分木に頂点 `subtree_root → new_root` の辺を追加する関数  
    ///
    /// モノイドの型指定のために、`Rerooting::<Monoid, _>::new(..)`として下さい  
    pub fn new(graph: &Vec<Vec<usize>>, add_root: F) -> Self {
        let vertex_cnt = graph.len();
        let subtree_memo = vec![M::id_element(); vertex_cnt];
        let ans = vec![M::id_element(); vertex_cnt];
        let mut ret = Self {
            vertex_cnt,
            subtree_memo,
            ans,
            add_root,
        };
        ret.dfs(graph, 0, usize::MAX);
        ret.bfs(graph, 0, usize::MAX, M::id_element());
        ret
    }

    pub fn get_ans(&self, root: usize) -> M::Target {
        assert!(root < self.vertex_cnt);
        self.ans[root].clone()
    }

    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize) {
        for &to in &graph[v] {
            if to == p {
                continue;
            }
            self.dfs(graph, to, v);
            let memo = (self.add_root)(&self.subtree_memo[to], to, v);
            self.subtree_memo[v] = M::binary_operation(&self.subtree_memo[v], &memo);
        }
    }

    fn bfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize, par_val: M::Target) {
        // 左右から累積和を取っておく
        let mut buf = Vec::with_capacity(graph[v].len());
        for &to in &graph[v] {
            if to == p {
                continue;
            } else {
                buf.push((self.add_root)(&self.subtree_memo[to], to, v));
            }
        }
        let mut left_sum = vec![M::id_element(); buf.len() + 1];
        let mut right_sum = vec![M::id_element(); buf.len() + 1];
        for i in 0..buf.len() {
            left_sum[i + 1] = M::binary_operation(&left_sum[i], &buf[i]);
        }
        for i in (0..buf.len()).rev() {
            right_sum[i] = M::binary_operation(&buf[i], &right_sum[i + 1]);
        }
        if p == usize::MAX {
            self.ans[v] = left_sum.last().unwrap().clone();
        } else {
            self.ans[v] = M::binary_operation(left_sum.last().unwrap(), &par_val);
        }

        // 子に伝播
        for (i, &to) in graph[v].iter().filter(|v| **v != p).enumerate() {
            // 一つも部分木をmergeしないなら、leafを用いる
            let propagate = M::binary_operation(
                &par_val,
                &M::binary_operation(&left_sum[i], &right_sum[i + 1]),
            );
            let par_val = (self.add_root)(&propagate, v, to);
            self.bfs(graph, to, v, par_val);
        }
    }
}
