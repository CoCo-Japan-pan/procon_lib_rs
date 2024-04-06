use algebra::{Commutative, Monoid};

pub trait Rerootable {
    /// DPテーブルに載せる可換モノイド  
    /// add_rootによりできた「部分木+一辺」同士をmergeする関数を二項演算として持つ  
    type DPMonoid: Monoid + Commutative;
    /// 葉に入れる値(デフォルトでは単位元)  
    /// 単位元以外を入れたい場合はオーバーライドしてください
    #[allow(unused_variables)]
    fn leaf(vertex: usize) -> <Self::DPMonoid as Monoid>::Target {
        <Self::DPMonoid as Monoid>::id_element()
    }
    /// 部分木に頂点v→pの辺を追加する
    #[allow(unused_variables)]
    fn add_root(
        subtree: &<Self::DPMonoid as Monoid>::Target,
        subtree_root: usize,
        new_root: usize,
    ) -> <Self::DPMonoid as Monoid>::Target;
}

#[derive(Debug)]
pub struct Rerooting<T: Rerootable> {
    vertex_cnt: usize,
    /// 根を0とした場合の各頂点を根とする部分木のDPテーブル
    subtree_memo: Vec<<T::DPMonoid as Monoid>::Target>,
    /// 各頂点を根とした木全体のDPテーブル
    ans: Vec<<T::DPMonoid as Monoid>::Target>,
}

impl<T: Rerootable> Rerooting<T> {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let vertex_cnt = graph.len();
        let subtree_memo = (0..vertex_cnt).map(|v| T::leaf(v)).collect::<Vec<_>>();
        let ans = vec![<T::DPMonoid as Monoid>::id_element(); vertex_cnt];
        let mut ret = Self {
            vertex_cnt,
            subtree_memo,
            ans,
        };
        ret.dfs(graph, 0, std::usize::MAX);
        ret.bfs(
            graph,
            0,
            std::usize::MAX,
            <T::DPMonoid as Monoid>::id_element(),
        );
        ret
    }

    pub fn get_ans(&self, root: usize) -> <T::DPMonoid as Monoid>::Target {
        assert!(root < self.vertex_cnt);
        self.ans[root].clone()
    }

    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize) {
        for &to in &graph[v] {
            if to == p {
                continue;
            }
            self.dfs(graph, to, v);
            self.subtree_memo[v] = <T::DPMonoid as Monoid>::binary_operation(
                &self.subtree_memo[v],
                &T::add_root(&self.subtree_memo[to], to, v),
            );
        }
    }

    fn bfs(
        &mut self,
        graph: &Vec<Vec<usize>>,
        v: usize,
        p: usize,
        par_val: <T::DPMonoid as Monoid>::Target,
    ) {
        // 左右から累積和を取っておく
        let mut buf = Vec::with_capacity(graph[v].len());
        for &to in &graph[v] {
            if to == p {
                continue;
            } else {
                buf.push(T::add_root(&self.subtree_memo[to], to, v));
            }
        }
        let mut left_sum = vec![<T::DPMonoid as Monoid>::id_element(); buf.len() + 1];
        let mut right_sum = vec![<T::DPMonoid as Monoid>::id_element(); buf.len() + 1];
        for i in 0..buf.len() {
            left_sum[i + 1] = <T::DPMonoid as Monoid>::binary_operation(&left_sum[i], &buf[i]);
        }
        for i in (0..buf.len()).rev() {
            right_sum[i] = <T::DPMonoid as Monoid>::binary_operation(&buf[i], &right_sum[i + 1]);
        }
        if p == usize::MAX {
            self.ans[v] = left_sum.last().unwrap().clone();
        } else {
            self.ans[v] =
                <T::DPMonoid as Monoid>::binary_operation(left_sum.last().unwrap(), &par_val);
        }

        // 子に伝播
        for (i, &to) in graph[v].iter().enumerate() {
            if to == p {
                continue;
            }
            self.bfs(
                graph,
                to,
                v,
                T::add_root(
                    &<T::DPMonoid as Monoid>::binary_operation(
                        &par_val,
                        &<T::DPMonoid as Monoid>::binary_operation(&left_sum[i], &right_sum[i + 1]),
                    ),
                    v,
                    to,
                ),
            );
        }
    }
}
