//! 頂点に着目したオイラーツアー  
//! LCAをRMQに帰着させて求められる  
//! SparseTableを用いるので前時間`O(NlogN)`、クエリ時間`O(1)`  
use algebra::{IdempotentMonoid, Monoid};
use sparse_table::SparseTable;

#[derive(Debug)]
pub struct MinMonoid;
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
    /// 各辺を2回ずつ通るので、サイズは`2|E| + 1 = "2|V| - 1`
    pub euler_tour_vertex: Vec<usize>,
    /// 各頂点の深さ
    pub depth: Vec<usize>,
    /// オイラーツアーにおいて、各頂点が最初に出現するインデックス
    pub first_occurrence: Vec<usize>,
    /// オイラーツアーにおいて、各頂点が最後に出現するインデックス
    pub last_occurrence: Vec<usize>,
    /// (深さ、頂点)の配列から構成されるSparseTable  
    /// first_occurenceの[最小、最大]の範囲で区間積を取ることで、(lcaの深さ、lcaの頂点)を求められる  
    sparse_table: SparseTable<MinMonoid>,
}

impl EulerTour {
    /// SparseTableを構築しているので、`O(NlogN)`
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
        let mut first_occurrence = vec![usize::MAX; n];
        let mut last_occurrence = vec![0; n];
        for (i, &v) in cls.euler_tour_vertex.iter().enumerate() {
            first_occurrence[v] = first_occurrence[v].min(i);
            last_occurrence[v] = i;
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
            first_occurrence,
            last_occurrence,
            sparse_table,
        }
    }

    /// SparseTableを用いているので、`O(1)`
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let l = self.first_occurrence[u];
        let r = self.first_occurrence[v];
        let (l, r) = (l.min(r), l.max(r));
        self.sparse_table.prod(l..=r).1
    }

    pub fn lca_multiple(&self, vertex_list: &[usize]) -> usize {
        let l = vertex_list
            .iter()
            .map(|&v| self.first_occurrence[v])
            .min()
            .unwrap();
        let r = vertex_list
            .iter()
            .map(|&v| self.first_occurrence[v])
            .max()
            .unwrap();
        self.sparse_table.prod(l..=r).1
    }
}
