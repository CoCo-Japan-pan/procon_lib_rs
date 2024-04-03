//! [HCPCの資料](https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)  
//! [HLDの中にsubtreeクエリも対応させる](https://codeforces.com/blog/entry/53170)  

#[derive(Debug)]
#[allow(dead_code)]
pub struct HLD {
    /// 各頂点について、heavypath(descending)が最初に来るようswapされている
    sorted_graph: Vec<Vec<usize>>,
    /// 各頂点についてそれを根とする部分木のサイズ
    subtree_size: Vec<usize>,
    /// 各頂点の深さ(根は0とする)
    depth: Vec<usize>,
    /// 各頂点の親(根にはusize::MAXを入れる)
    parent: Vec<usize>,
    /// 各頂点の属するheavy pathの先頭
    heavy_path_lowest: Vec<usize>,
    /// heavy pathを並べた配列における各頂点のindex
    hld_in: Vec<usize>,
    /// 各頂点の部分木に属する頂点が出てこなくなる最初のindex
    hld_out: Vec<usize>,
    /// 頂点の数
    vertex_cnt: usize,
}

impl HLD {
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let mut ret = Self {
            sorted_graph: graph.to_vec(),
            subtree_size: vec![0; graph.len()],
            depth: vec![0; graph.len()],
            parent: vec![usize::MAX; graph.len()],
            heavy_path_lowest: vec![root; graph.len()],
            hld_in: vec![0; graph.len()],
            hld_out: vec![0; graph.len()],
            vertex_cnt: graph.len(),
        };
        ret.dfs_sz(root, usize::MAX);
        let mut id = 0;
        ret.dfs_hld(root, &mut id);
        ret
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.vertex_cnt && v < self.vertex_cnt);
        // 同じheavy_path上に乗るまで上る
        while self.heavy_path_lowest[u] != self.heavy_path_lowest[v] {
            // 短いheavy_pathの方を上る
            if self.hld_in[u] < self.hld_in[v] {
                v = self.parent[self.heavy_path_lowest[v]];
            } else {
                u = self.parent[self.heavy_path_lowest[u]];
            }
        }
        // 同じheavy_path上に乗ったので、浅いほうを返す
        if self.depth[u] < self.depth[v] {
            u
        } else {
            v
        }
    }
}

impl HLD {
    fn dfs_sz(&mut self, v: usize, p: usize) {
        self.subtree_size[v] = 1;
        self.parent[v] = p;
        for i in 0..self.sorted_graph[v].len() {
            let u = self.sorted_graph[v][i];
            if u == p {
                continue;
            }
            self.depth[u] = self.depth[v] + 1;
            self.dfs_sz(u, v);
            self.subtree_size[v] += self.subtree_size[u];
            // heavy pathの先頭を最初に来るようswap
            if self.subtree_size[u] > self.subtree_size[self.sorted_graph[v][0]] {
                self.sorted_graph[v].swap(0, i);
            }
        }
    }

    fn dfs_hld(&mut self, v: usize, id: &mut usize) {
        self.hld_in[v] = *id;
        *id += 1;
        for i in 0..self.sorted_graph[v].len() {
            let u = self.sorted_graph[v][i];
            if u == self.parent[v] {
                continue;
            }
            self.heavy_path_lowest[u] = if i == 0 {
                // heavy path を下っている
                self.heavy_path_lowest[v]
            } else {
                // ここから新しいheavy pathが始まる
                u
            };
            self.dfs_hld(u, id);
        }
        self.hld_out[v] = *id;
    }
}
