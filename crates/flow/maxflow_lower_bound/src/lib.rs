//! 最小流量制限付き最大流  
//! <https://atcoder.jp/contests/abc285/editorial/5500>  
//! <https://tubo28.me/compprog/algorithm/flow_with_lu_bound/>

use internal_type_traits::Integral;
use maxflow::{Edge, MaxFlow};
use std::ops::RangeBounds;

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

    /// from→toへ、rangeの流量制約を持つ辺を張る(返す辺のidは、from→toのcap=upper-lowerの辺のid)  
    /// (大抵は大丈夫だが)excludedな境界についてはCap::one()を足し引きしていることに注意
    pub fn add_edge_with_lower_bound<R: RangeBounds<Cap>>(
        &mut self,
        from: usize,
        to: usize,
        range: R,
    ) -> usize {
        let lower = match range.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + Cap::one(),
            std::ops::Bound::Unbounded => Cap::zero(),
        };
        let upper = match range.end_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x - Cap::one(),
            std::ops::Bound::Unbounded => Cap::max_value(),
        };
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
