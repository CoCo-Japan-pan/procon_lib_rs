//! 有効グラフの強連結成分分解を行います。  
//! DFSを二回行う方針  

#[derive(Debug, Clone)]
pub struct SccGraph {
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
    vertices: usize,
}

impl From<Vec<Vec<usize>>> for SccGraph {
    fn from(graph: Vec<Vec<usize>>) -> Self {
        let vertices = graph.len();
        let mut rev_graph = vec![vec![]; vertices];
        for (from, tos) in graph.iter().enumerate() {
            for &to in tos {
                rev_graph[to].push(from);
            }
        }
        Self {
            graph,
            rev_graph,
            vertices,
        }
    }
}

impl SccGraph {
    pub fn new(vertices: usize) -> Self {
        Self {
            graph: vec![vec![]; vertices],
            rev_graph: vec![vec![]; vertices],
            vertices,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.vertices && to < self.vertices);
        self.graph[from].push(to);
        self.rev_graph[to].push(from);
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.vertices];
        let mut order = Vec::with_capacity(self.vertices);
        for i in 0..self.vertices {
            if !visited[i] {
                self.dfs(i, &mut visited, &mut order, false);
            }
        }
        visited.fill(false);
        let mut scc = vec![];
        for &i in order.iter().rev() {
            if !visited[i] {
                let mut group = vec![];
                self.dfs(i, &mut visited, &mut group, true);
                scc.push(group);
            }
        }
        scc
    }

    fn dfs(&self, v: usize, visited: &mut [bool], order: &mut Vec<usize>, is_rev: bool) {
        visited[v] = true;
        for &to in if is_rev {
            &self.rev_graph[v]
        } else {
            &self.graph[v]
        } {
            if !visited[to] {
                self.dfs(to, visited, order, is_rev);
            }
        }
        order.push(v);
    }
}

/// From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/scc.rs>
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scc_simple() {
        let mut graph = SccGraph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);
        let scc = graph.scc();
        assert_eq!(scc.len(), 1);
    }

    #[test]
    fn test_scc_self_loop() {
        let mut graph = SccGraph::new(2);
        graph.add_edge(0, 0);
        graph.add_edge(0, 0);
        graph.add_edge(1, 1);
        let scc = graph.scc();
        assert_eq!(scc.len(), 2);
    }

    #[test]
    fn solve_alpc_g_sample1() {
        // https://atcoder.jp/contests/practice2/tasks/practice2_g
        let n: usize = 6;
        let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

        let mut graph = SccGraph::new(n);
        for (u, v) in edges.into_iter() {
            graph.add_edge(u, v);
        }

        let scc = graph.scc();
        assert_eq!(scc, vec![vec![5], vec![4, 1], vec![2], vec![3, 0]]);
    }
}
