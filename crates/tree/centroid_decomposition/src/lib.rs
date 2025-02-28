//! 木の(再帰的な)重心分解を行う

pub struct CentroidDecomposition<'a> {
    graph: &'a Vec<Vec<usize>>,
    /// 使いまわす配列 部分木のサイズを保持しておく
    subtree_size: Vec<usize>,
    used: Vec<bool>,
}

impl<'a> CentroidDecomposition<'a> {
    pub fn new(graph: &'a Vec<Vec<usize>>) -> Self {
        Self {
            graph,
            subtree_size: vec![0; graph.len()],
            used: vec![false; graph.len()],
        }
    }

    /// [centroid-tree](https://www.quora.com/profile/Abbas-Rangwala-13/Centroid-Decomposition-of-a-Tree)  
    /// グラフが空の場合は(vec![], None)を返す  
    /// 返り値としては、centroid-treeの(親、子)のペアのリストとSome(根)のペアを返す  
    pub fn calc_centroid_tree(self) -> (Vec<(usize, usize)>, Option<usize>) {
        if self.graph.is_empty() {
            return (vec![], None);
        }
        struct Cls<'a> {
            slf: CentroidDecomposition<'a>,
            ret: Vec<(usize, usize)>,
            root: Option<usize>,
        }
        let len = self.graph.len();
        let mut cls = Cls {
            slf: self,
            ret: Vec::with_capacity(len),
            root: None,
        };
        fn dfs(cls: &mut Cls, subtree_root: usize, prev_centroid: usize) {
            let centroid = cls.slf.get_centroid(subtree_root);
            if prev_centroid == !0 {
                cls.root = Some(centroid);
            } else {
                cls.ret.push((prev_centroid, centroid));
            }
            cls.slf.used[centroid] = true;
            for &next_subtree_root in &cls.slf.graph[centroid] {
                if cls.slf.used[next_subtree_root] {
                    continue;
                }
                dfs(cls, next_subtree_root, centroid);
            }
        }
        dfs(&mut cls, 0, !0);
        (cls.ret, cls.root)
    }

    /// `f = |used: &[bool], centroid: usize| { ... }`  
    /// `used`がtrueの頂点は既に見た頂点 `centroid`は現在考える重心  
    /// `f`は重心をまたぐ処理  
    /// 再帰的に重心分解を行いつつ、重心をまたぐ処理を途中で行う
    pub fn run<F: FnMut(&[bool], usize)>(mut self, mut f: F) {
        self.main_dfs(0, &mut f);
    }

    fn main_dfs<F: FnMut(&[bool], usize)>(&mut self, v: usize, f: &mut F) {
        let centroid = self.get_centroid(v);

        // 重心をまたぐ処理を行う
        f(&self.used, centroid);

        self.used[centroid] = true;
        for &next_subtree_root in &self.graph[centroid] {
            if self.used[next_subtree_root] {
                continue;
            }
            self.main_dfs(next_subtree_root, f);
        }
    }

    /// usedがtrueの頂点を除いて、各頂点の部分木のサイズを計算する
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

    /// usedがtrueの頂点を除いて、subtree_rootを根とする部分木の重心を求める  
    /// このとき内部のself.subtree_sizeの配列を書き換える
    pub fn get_centroid(&mut self, subtree_root: usize) -> usize {
        self.calc_subtree_size(subtree_root, !0);
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
