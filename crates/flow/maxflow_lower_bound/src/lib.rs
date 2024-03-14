//! 最小流量制限付き最大流  
//! https://tubo28.me/compprog/algorithm/flow_with_lu_bound/

use maxflow::MaxFlow;
use flow_cap_traits::Integral;

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

    /// from→toへ、[lower,upper]の流量制約を持つ辺を張る
    pub fn add_edge(&mut self, from: usize, to: usize, lower: Cap, upper: Cap) {
        assert!(Cap::zero() <= lower && lower <= upper);
        assert!(from < self.vertices && to < self.vertices);
        if from == to || upper == Cap::zero() {
            return;
        }
        self.maxflow.add_edge(from, to, upper - lower);
        self.maxflow.add_edge(self.dummy_source, to, lower);
        self.maxflow.add_edge(from, self.dummy_sink, lower);
        self.lower_bound_sum += lower;
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