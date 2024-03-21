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
    /// 各頂点の親
    parent: Vec<Option<usize>>,
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
    pub fn new(_graph: &[Vec<usize>]) -> Self {
        todo!()
    }
}
