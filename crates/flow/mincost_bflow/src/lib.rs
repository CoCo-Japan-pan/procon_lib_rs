//! b-flowに帰着することで負辺削除や、最小流量制限を機械的に扱えるようにする  
//! 内部でs-t間の最小費用流を求めるために、atcoderのライブラリを利用している

use atcoder_mincostflow::MinCostFlowGraph;
use internal_type_traits::Integral;
use std::ops::Neg;

#[derive(Debug, Clone, Default)]
pub struct MinCostBFlowResult<T> {
    /// 総コスト
    pub cost: T,
    /// 復元した流量
    pub flow: Vec<T>,
    /// ポテンシャル
    pub potential: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct MinCostBFlow<T: Integral + Neg<Output = T>> {
    result: MinCostBFlowResult<T>,
    size: usize,
    mcf: MinCostFlowGraph<T>,
    b_list: Vec<T>,
    /// 負辺の場合は逆にしている 負辺の各idごとに、逆にしていればtrue
    rev: Vec<bool>,
}

impl<T: Integral + Neg<Output = T>> MinCostBFlow<T> {
    pub fn new(n: usize) -> Self {
        Self {
            result: MinCostBFlowResult::default(),
            size: n,
            mcf: MinCostFlowGraph::new(n + 2),
            b_list: vec![T::zero(); n],
            rev: vec![],
        }
    }

    /// `from -> to` に、`lower <= cap <= upper` の流量制限がある辺を張る  
    /// 負辺の場合はあらかじめupperだけ流すので、ここはINFにせず、オーバフローしない上限を指定する！  
    /// 流量が増えるので、TLEする場合、負辺はうまく下駄をはかせる等の対処が必要かも
    pub fn add_edge(
        &mut self,
        mut from: usize,
        mut to: usize,
        mut lower: T,
        mut upper: T,
        mut cost: T,
    ) -> usize {
        assert!(from < self.size);
        assert!(to < self.size);
        assert!(T::zero() <= lower);
        assert!(lower <= upper);
        let minus_edge = cost < T::zero();
        self.rev.push(minus_edge);
        // 負辺の場合は最大まであらかじめ流し、逆の辺を張る
        if minus_edge {
            std::mem::swap(&mut from, &mut to);
            (lower, upper) = (-upper, -lower);
            cost = -cost;
        }
        // from -> to にあらかじめlowerだけ流しておく
        self.b_list[from] -= lower;
        self.b_list[to] += lower;
        self.result.flow.push(lower);
        self.result.cost += lower * cost;
        self.mcf.add_edge(from, to, upper - lower, cost)
    }

    /// 頂点vにsupply分の湧き出しを追加
    pub fn add_supply(&mut self, v: usize, supply: T) {
        assert!(v < self.size);
        assert!(supply >= T::zero());
        self.b_list[v] += supply;
    }

    /// 頂点vにdemand分の吸い込みを追加
    pub fn add_demand(&mut self, v: usize, demand: T) {
        assert!(v < self.size);
        assert!(demand >= T::zero());
        self.b_list[v] -= demand;
    }

    /// 超頂点を用意してst-flowに帰着し流し、総コストも更新  
    /// bの正の和と負の絶対値の和が等しくない場合はfalseを返す  
    /// このとき最大まで流せればtrueを返す
    fn reduce_to_st_flow(&mut self) -> bool {
        let dummy_source = self.size;
        let dummy_sink = self.size + 1;
        let mut positive_sum = T::zero();
        let mut negative_sum = T::zero();
        for (v, &b) in self.b_list.iter().enumerate() {
            use std::cmp::Ordering::*;
            match b.cmp(&T::zero()) {
                Less => {
                    self.mcf.add_edge(v, dummy_sink, -b, T::zero());
                    negative_sum += -b;
                }
                Greater => {
                    self.mcf.add_edge(dummy_source, v, b, T::zero());
                    positive_sum += b;
                }
                Equal => {}
            }
        }
        if positive_sum != negative_sum {
            return false;
        }
        let (flow, cost) = self.mcf.flow(dummy_source, dummy_sink, positive_sum);
        self.result.cost += cost;
        flow == positive_sum
    }

    fn recover_flow_potential(&mut self) {
        let edges = self.mcf.edges();
        let mut potential_edges = vec![];
        for ((res_flow, edge), rev) in self.result.flow.iter_mut().zip(edges).zip(&self.rev) {
            *res_flow += edge.flow;
            if *rev {
                *res_flow = -*res_flow;
            }

            // 相補性条件
            if edge.flow < edge.cap {
                potential_edges.push((edge.from, edge.to, edge.cost));
            }
            if edge.flow > T::zero() {
                potential_edges.push((edge.to, edge.from, -edge.cost));
            }
        }

        // ベルマンフォード
        self.result.potential.resize(self.size, T::zero());
        for _ in 0..self.size {
            for &(from, to, cost) in &potential_edges {
                let old = self.result.potential[to];
                let new = self.result.potential[from] + cost;
                self.result.potential[to] = old.min(new);
            }
        }
    }

    /// 最小費用b-flowを解く  
    /// infeasible(実行不可能)ならNoneを返す  
    /// 解ける場合はそのときの最小費用、復元した各辺の流量、各頂点のポテンシャルを返す
    pub fn mincost_bflow(&mut self) -> Option<&MinCostBFlowResult<T>> {
        if !self.reduce_to_st_flow() {
            return None;
        }
        self.recover_flow_potential();
        Some(&self.result)
    }

    /// sからtに自由なだけ流せる場合の最小費用b-flowを解く  
    /// これはただtからsに無限容量、コスト0の辺を張ってb-flowを解くだけ  
    /// bの条件を満たせない(infeasible)ならNoneを返す  
    /// sからtに流れた量と、MinCostBFlowResultのペアを返す
    pub fn st_mincost_freeflow(
        &mut self,
        s: usize,
        t: usize,
    ) -> Option<(T, &MinCostBFlowResult<T>)> {
        assert!(s < self.size && t < self.size && s != t);
        let t_to_s_id = self.mcf.add_edge(t, s, T::max_value(), T::zero());
        if !self.reduce_to_st_flow() {
            return None;
        }
        let flow = self.mcf.get_edge(t_to_s_id).flow;
        self.recover_flow_potential();
        Some((flow, &self.result))
    }

    /// 最小費用最大流を解く  
    /// bの条件を満たせない(infeasible)ならNoneを返す  
    /// sに+f, tに-fした後の制約を満たすようなフローが存在するような最大のfと、MinCostBFlowResultのペアを返す
    pub fn st_mincost_maxflow(
        &mut self,
        s: usize,
        t: usize,
    ) -> Option<(T, &MinCostBFlowResult<T>)> {
        assert!(s < self.size && t < self.size && s != t);
        // まずsからtに自由に流せるときを求める
        let t_to_s_id = self.mcf.add_edge(t, s, T::max_value(), T::zero());
        if !self.reduce_to_st_flow() {
            return None;
        }
        let first_flow = self.mcf.get_edge(t_to_s_id).flow;
        // sに+無限、tに-無限と考えて流す
        let (add_flow, add_cost) = self.mcf.flow(s, t, T::max_value());
        self.result.cost += add_cost;
        self.recover_flow_potential();
        Some((first_flow + add_flow, &self.result))
    }
}
