//! 木の(再帰的な)重心分解を行う

pub struct CentroidDecomposition<'a> {
    graph: &'a Vec<Vec<usize>>,
    pub subtree_size: Vec<usize>,
    pub used: Vec<bool>,
}

impl<'a> CentroidDecomposition<'a> {
    pub fn new(graph: &'a Vec<Vec<usize>>) -> Self {
        Self {
            graph,
            subtree_size: vec![0; graph.len()],
            used: vec![false; graph.len()],
        }
    }

    pub fn get_centroid_once(&self) -> usize {
        self.get_centroid(0)
    }

    /// `f = |used: &[bool], centroid: usize| { ... }`  
    /// `used`がtrueの頂点は既に見た頂点 `centroid`は現在考える重心  
    /// `f`は重心をまたぐ処理  
    /// 再帰的に重心分解を行いつつ、重心をまたぐ処理を途中で行う
    pub fn run<F: FnMut(&[bool], usize)>(&mut self, f: F) {
        self.main_dfs(0, f);
    }

    fn main_dfs<F: FnMut(&[bool], usize)>(&mut self, v: usize, mut f: F) {
        self.calc_subtree_size(v, !0);
        let centroid = self.get_centroid(v);
        self.used[centroid] = true;

        // 重心をまたぐ処理を行う
        f(&self.used, centroid);

        for &next_subtree_root in &self.graph[centroid] {
            if self.used[next_subtree_root] {
                continue;
            }
            self.main_dfs(next_subtree_root, &mut f);
        }
    }

    fn calc_subtree_size(&mut self, v: usize, p: usize) {
        self.subtree_size[v] = 1;
        for &u in &self.graph[v] {
            if u == p || self.used[u] {
                continue;
            }
            self.calc_subtree_size(u, v);
            self.subtree_size[v] += self.subtree_size[u];
        }
    }

    fn get_centroid(&self, subtree_root: usize) -> usize {
        let cur_size = self.subtree_size[subtree_root];
        self.dfs_for_centrioid(subtree_root, !0, cur_size)
    }

    fn dfs_for_centrioid(&self, v: usize, p: usize, all_size: usize) -> usize {
        for &nv in &self.graph[v] {
            if nv == p || self.used[nv] {
                continue;
            }
            if self.subtree_size[nv] * 2 > all_size {
                return self.dfs_for_centrioid(nv, v, all_size);
            }
        }
        v
    }
}
