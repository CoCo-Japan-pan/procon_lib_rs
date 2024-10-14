use std::collections::VecDeque;

struct InternalLowLink<'a> {
    graph: &'a Vec<Vec<usize>>,
    ord: Vec<usize>,
    low: Vec<usize>,
    used: Vec<bool>,
    cur_ord: usize,
    articulation_points: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}

impl<'a> InternalLowLink<'a> {
    fn build(graph: &'a Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut ret = Self {
            graph,
            ord: vec![n; n],
            low: vec![n; n],
            used: vec![false; n],
            cur_ord: 0,
            articulation_points: vec![],
            bridges: vec![],
        };
        for v in 0..n {
            if ret.used[v] {
                continue;
            }
            ret.dfs(v, !0);
        }
        ret
    }

    fn dfs(&mut self, v: usize, p: usize) {
        self.ord[v] = self.cur_ord;
        self.low[v] = self.cur_ord;
        self.cur_ord += 1;
        self.used[v] = true;
        let mut child_cnt = 0;
        let mut is_aps = false;
        // 同じ辺が二つ以上ある場合はそれは後退辺となるので注意する
        let mut checked_parent = false;
        for &to in &self.graph[v] {
            if to == p && !checked_parent {
                checked_parent = true;
                continue;
            }
            if !self.used[to] {
                child_cnt += 1;
                self.dfs(to, v);
                // 子からのlowの伝播
                self.low[v] = self.low[v].min(self.low[to]);
                // vがDFS木の根でない場合、その子toについてord[v] <= low[to]ならばvは関節点
                if p != !0 && self.ord[v] <= self.low[to] {
                    is_aps = true;
                }
                // ord[v] < low[to]ならば(v, to)は橋
                if self.ord[v] < self.low[to] {
                    self.bridges.push((v, to));
                }
            } else {
                // 後退辺
                self.low[v] = self.low[v].min(self.ord[to]);
            }
        }
        // vがDFS木の根である場合、子が2つ以上ならばvは関節点
        if p == !0 && child_cnt >= 2 {
            is_aps = true;
        }
        if is_aps {
            self.articulation_points.push(v);
        }
    }
}

/// `ord[v] < low[to]`ならば(v, to)は橋  
/// vがDFS木の根である場合、DFS木における子が2つ以上ならばvは関節点  
/// vがDFS木の根でない場合、その子toについて`ord[v] <= low[to]`ならばvは関節点
#[derive(Debug)]
pub struct LowLink<'a> {
    graph: &'a Vec<Vec<usize>>,
    pub ord: Vec<usize>,
    pub low: Vec<usize>,
    pub articulation_points: Vec<usize>,
    pub bridges: Vec<(usize, usize)>,
}

impl<'a> LowLink<'a> {
    /// 隣接リスト形式で無向グラフを受け取り、ord,low,関節点,橋を返す `O(V + E)`  
    pub fn new(graph: &'a Vec<Vec<usize>>) -> Self {
        let internal = InternalLowLink::build(graph);
        Self {
            graph,
            ord: internal.ord,
            low: internal.low,
            articulation_points: internal.articulation_points,
            bridges: internal.bridges,
        }
    }

    /// 2重辺連結成分分解 `O(V + E)`  
    /// `各連結成分の二重配列` を返す  
    /// 橋を消し、連結成分をまとめる 頂点を排他的に分解することになる  
    /// 連結成分を縮約して頂点とみなし、橋を辺とみなすことで木になる
    pub fn two_edge_cc(&self) -> (Vec<Vec<usize>>, Vec<usize>) {
        let mut cur_cc_id = 0;
        let mut ccs = vec![];
        let mut cc_id = vec![!0; self.graph.len()];
        for v in 0..self.graph.len() {
            if cc_id[v] != !0 {
                continue;
            }
            let mut component = vec![v];
            cc_id[v] = cur_cc_id;
            let mut que = VecDeque::new();
            que.push_back(v);
            while let Some(v) = que.pop_front() {
                for &to in &self.graph[v] {
                    if cc_id[to] != !0 {
                        continue;
                    }
                    // 橋
                    {
                        let (from, to) = if self.ord[v] < self.ord[to] {
                            (v, to)
                        } else {
                            (to, v)
                        };
                        if self.ord[from] < self.low[to] {
                            continue;
                        }
                    }
                    cc_id[to] = cur_cc_id;
                    component.push(to);
                    que.push_back(to);
                }
            }
            cur_cc_id += 1;
            ccs.push(component);
        }
        (ccs, cc_id)
    }
}
