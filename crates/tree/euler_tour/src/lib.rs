//! 頂点に着目したオイラーツアー  
//! LCAをRMQに帰着させて求められる  
//! SparseTableを用いるので前時間`O(NlogN)`、クエリ時間`O(1)`  
use algebra::{IdempotentMonoid, Monoid};
use sparse_table::SparseTable;

#[derive(Debug)]
struct MinMonoid;
impl Monoid for MinMonoid {
    type Target = (usize, usize);
    fn id_element() -> Self::Target {
        (usize::MAX, usize::MAX)
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}
impl IdempotentMonoid for MinMonoid {}

#[derive(Debug)]
pub struct EulerTour {
    /// 頂点に着目したオイラーツアー
    pub euler_tour_vertex: Vec<usize>,
    /// 各頂点の深さ
    pub depth: Vec<usize>,
    /// オイラーツアーにおいて、各頂点が最初に出現するインデックス
    pub fst_occurrence: Vec<usize>,
    /// (深さ、頂点)の配列から構成されるSparseTable
    sparse_table: SparseTable<MinMonoid>,
}

impl EulerTour {
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let n = graph.len();
        struct Cls<'a> {
            graph: &'a [Vec<usize>],
            euler_tour_vertex: Vec<usize>,
            depth: Vec<usize>,
        }
        let mut cls = Cls {
            graph,
            euler_tour_vertex: Vec::with_capacity(2 * n - 1),
            depth: vec![0; n],
        };
        fn dfs(cls: &mut Cls, v: usize, p: usize) {
            cls.euler_tour_vertex.push(v);
            for &nv in &cls.graph[v] {
                if nv == p {
                    continue;
                }
                cls.depth[nv] = cls.depth[v] + 1;
                dfs(cls, nv, v);
                cls.euler_tour_vertex.push(v);
            }
        }
        dfs(&mut cls, root, n);
        let mut fst_occurrence = vec![usize::MAX; n];
        for (i, &v) in cls.euler_tour_vertex.iter().enumerate() {
            fst_occurrence[v] = fst_occurrence[v].min(i);
        }
        // オイラーツアーの深さと頂点のペアからなる配列を作成
        let depth_vertex = cls
            .euler_tour_vertex
            .iter()
            .map(|&v| (cls.depth[v], v))
            .collect();
        let sparse_table = SparseTable::new(depth_vertex);
        Self {
            euler_tour_vertex: cls.euler_tour_vertex,
            depth: cls.depth,
            fst_occurrence,
            sparse_table,
        }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        let l = self.fst_occurrence[u];
        let r = self.fst_occurrence[v];
        let (l, r) = (l.min(r), l.max(r));
        self.sparse_table.prod(l..=r).1
    }

    pub fn lca_multiple(&self, vertex_list: &[usize]) -> usize {
        let l = vertex_list
            .iter()
            .map(|&v| self.fst_occurrence[v])
            .min()
            .unwrap();
        let r = vertex_list
            .iter()
            .map(|&v| self.fst_occurrence[v])
            .max()
            .unwrap();
        self.sparse_table.prod(l..=r).1
    }
}
