//! 最小流量制限付き最大流  
//! <https://tubo28.me/compprog/algorithm/flow_with_lu_bound/>

use flow_cap_traits::Integral;
use maxflow::{Edge, MaxFlow};

pub struct MaxFlowLowerBound<Cap: Integral> {
    maxflow: MaxFlow<Cap>,
    vertices: usize,
    dummy_source: usize,
    dummy_sink: usize,
    lower_bound_sum: Cap,
}

impl<Cap: Integral> MaxFlowLowerBound<Cap> {
    pub fn new(n: usize) -> Self {
        let dummy_source = n;
        let dummy_sink = n + 1;
        let maxflow = MaxFlow::new(n + 2);
        Self {
            maxflow,
            vertices: n,
            dummy_source,
            dummy_sink,
            lower_bound_sum: Cap::zero(),
        }
    }

    pub fn get_edge(&self, id: usize) -> Edge<Cap> {
        self.maxflow.get_edge(id)
    }

    /// from→toへ、容量capの辺を張る(lowerの制約は無し)
    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
        self.maxflow.add_edge(from, to, cap)
    }

    /// from→toへ、`[lower,upper]`の流量制約を持つ辺を張る(返す辺のidは、from→toのcap=upper-lowerの辺のid)
    pub fn add_edge_with_lower_bound(
        &mut self,
        from: usize,
        to: usize,
        lower: Cap,
        upper: Cap,
    ) -> usize {
        assert!(Cap::zero() <= lower && lower <= upper);
        assert!(from < self.vertices && to < self.vertices);
        assert!(from != to && upper > Cap::zero());
        if lower == Cap::zero() {
            return self.maxflow.add_edge(from, to, upper);
        }
        self.lower_bound_sum += lower;
        self.maxflow.add_edge(self.dummy_source, to, lower);
        self.maxflow.add_edge(from, self.dummy_sink, lower);
        self.maxflow.add_edge(from, to, upper - lower)
    }

    /// 最小流量制限を満たせるならその最大流量を返し、満たせないならNoneを返す
    pub fn flow(&mut self, source: usize, sink: usize) -> Option<Cap> {
        let a = self.maxflow.flow(self.dummy_source, self.dummy_sink);
        let b = self.maxflow.flow(source, self.dummy_sink);
        let c = self.maxflow.flow(self.dummy_source, sink);
        let d = self.maxflow.flow(source, sink);
        if a + c == self.lower_bound_sum && a + b == self.lower_bound_sum {
            Some(b + d)
        } else {
            None
        }
    }
}
