//! LCAベースの圧縮木  
//! [Auxiliary Tree](https://atcoder.jp/contests/abc340/editorial/9249)
use euler_tour::EulerTour;

#[derive(Debug)]
pub struct AuxiliaryTree {
    pub euler_tour: EulerTour,
    pub pre_order_index: Vec<usize>,
}

impl AuxiliaryTree {
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let euler_tour = EulerTour::new(graph, root);
        struct Cls<'a> {
            graph: &'a [Vec<usize>],
            pre_order: Vec<usize>,
        }
        let mut cls = Cls {
            graph,
            pre_order: Vec::with_capacity(graph.len()),
        };
        fn dfs(cls: &mut Cls, v: usize, p: usize) {
            cls.pre_order.push(v);
            for &nv in &cls.graph[v] {
                if nv == p {
                    continue;
                }
                dfs(cls, nv, v);
            }
        }
        dfs(&mut cls, root, graph.len());
        let pre_order_index = {
            let mut pre_order = vec![0; graph.len()];
            for (i, v) in cls.pre_order.into_iter().enumerate() {
                pre_order[v] = i;
            }
            pre_order
        };
        Self {
            euler_tour,
            pre_order_index,
        }
    }

    /// LCAの関係を保ったまま圧縮された木を返す  
    /// (頂点集合, (親,子)のペアの集合, Some(根)) を返す  
    /// 空配列を渡すと`(vec![], vec![], None)`を返す  
    /// `O(KlogK) (K = vertex_subset.len())`  
    /// 圧縮された木のサイズは高々`2K-1`  
    pub fn gen_auxiliary_tree(
        &self,
        mut vertex_subset: Vec<usize>,
    ) -> (Vec<usize>, Vec<(usize, usize)>, Option<usize>) {
        if vertex_subset.is_empty() {
            return (vec![], vec![], None);
        }
        // pre-order順にソート
        vertex_subset.sort_by_key(|&v| self.pre_order_index[v]);
        {
            // LCAを追加
            let mut append = Vec::with_capacity(vertex_subset.len() - 1);
            for ad in vertex_subset.windows(2) {
                append.push(self.euler_tour.lca(ad[0], ad[1]));
            }
            vertex_subset.append(&mut append);
        }
        // LCAを追加したものをpre-order順にソート
        vertex_subset.sort_by_key(|&v| self.pre_order_index[v]);
        // 重複削除
        vertex_subset.dedup();
        let vertex_subset = vertex_subset;

        // 構築
        let mut par_v_pairs = Vec::with_capacity(vertex_subset.len() - 1);
        let mut stack = Vec::with_capacity(vertex_subset.len());
        stack.push(vertex_subset[0]);
        for &v in vertex_subset.iter().skip(1) {
            while let Some(&top) = stack.last() {
                if self.euler_tour.last_occurrence[top] < self.euler_tour.first_occurrence[v] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                par_v_pairs.push((top, v));
            }
            stack.push(v);
        }
        let root = stack[0];
        (vertex_subset, par_v_pairs, Some(root))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// example from https://smijake3.hatenablog.com/entry/2019/09/15/200200
    fn test_auxiliary_tree() {
        /*  0
           / \
          1   2
             / \
            3   9
            |   | \
            4   10 12
           /|\   \
          5 6 7   11
              \
               8
        => (1, 5, 8, 11)で圧縮
            0
           / \
          1   2
             / \
            4   11
           / \
          5   8
        */
        let parent = vec![usize::MAX, 0, 0, 2, 3, 4, 4, 4, 6, 2, 9, 10, 9];
        let graph = {
            let mut graph = vec![vec![]; parent.len()];
            for (i, &p) in parent.iter().enumerate().skip(1) {
                graph[p].push(i);
                graph[i].push(p);
            }
            graph
        };
        let auxiliary_tree = AuxiliaryTree::new(&graph, 0);
        let (_, par_ver_pair, root) = auxiliary_tree.gen_auxiliary_tree(vec![1, 5, 8, 11]);
        assert_eq!(root, Some(0));
        assert_eq!(
            par_ver_pair,
            vec![(0, 1), (0, 2), (2, 4), (4, 5), (4, 8), (2, 11)]
        );
    }
}
