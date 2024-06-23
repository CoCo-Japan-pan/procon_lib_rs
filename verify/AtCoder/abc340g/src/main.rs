// verification-helper: PROBLEM https://atcoder.jp/contests/abc340/tasks/abc340_g

use auxiliary_tree::AuxiliaryTree;
use proconio::{fastout, input, marker::Usize1};
use static_modint::ModInt998244353 as MInt;

#[fastout]
fn main() {
    input! {
        n: usize,
        color_list: [Usize1; n],
        u_v: [(Usize1, Usize1); n - 1],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for &(u, v) in &u_v {
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    };
    let vertex_per_color = {
        let mut vertex_per_color = vec![vec![]; n];
        for (i, &color) in color_list.iter().enumerate() {
            vertex_per_color[color].push(i);
        }
        vertex_per_color
    };
    let mut ans = MInt::new(0);
    let aux_tree = AuxiliaryTree::new(&graph, 0);
    for (cur_color, vertices) in vertex_per_color.into_iter().enumerate() {
        let (new_vertices, par_v_pairs, Some(root)) = aux_tree.gen_auxiliary_tree(vertices) else {
            continue;
        };
        let is_base = new_vertices
            .iter()
            .map(|&v| color_list[v] == cur_color)
            .collect::<Vec<_>>();
        let new_graph = {
            let len = new_vertices.len();
            let mut new_graph = vec![vec![]; len];
            for (par, v) in par_v_pairs {
                let par = new_vertices.binary_search(&par).unwrap();
                let v = new_vertices.binary_search(&v).unwrap();
                new_graph[par].push(v);
            }
            new_graph
        };
        struct Cls<'a> {
            ans: MInt,
            graph: &'a [Vec<usize>],
            is_base: &'a [bool],
        }
        // その頂点を根とするような部分木の数を返す
        fn dfs(cls: &mut Cls, v: usize) -> MInt {
            let mut ret = MInt::new(1);
            let mut only_one_sum = MInt::new(0);
            for &chd in &cls.graph[v] {
                let chd_cnt = dfs(cls, chd);
                only_one_sum += chd_cnt;
                ret *= chd_cnt + 1;
            }
            // 今の頂点がもともとの色の場合は、全てOK
            if cls.is_base[v] {
                cls.ans += ret;
            } else {
                // そもそも一つも選ばない部分木は却下
                ret -= 1;
                // 部分木の中で2つ以上は選ばないとだめ
                cls.ans += ret - only_one_sum;
            }
            ret
        }
        let mut cls = Cls {
            ans: MInt::new(0),
            graph: &new_graph,
            is_base: &is_base,
        };
        dfs(&mut cls, new_vertices.binary_search(&root).unwrap());
        ans += cls.ans;
    }
    println!("{}", ans);
}
