// https://atcoder.jp/contests/abc359/tasks/abc359_g
use auxiliary_tree::AuxiliaryTree;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        u_v: [(Usize1, Usize1); n - 1],
        color_list: [Usize1; n],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for (u, v) in u_v {
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    };
    let vertex_per_color = {
        let mut vertex_per_color = vec![vec![]; n];
        for (i, &a) in color_list.iter().enumerate() {
            vertex_per_color[a].push(i);
        }
        vertex_per_color
    };
    let aux_tree = AuxiliaryTree::new(&graph, 0);
    let mut ans = 0;
    for (color, vertices) in vertex_per_color.into_iter().enumerate() {
        let all_size = vertices.len();
        let (new_vertices, par_v, Some(root)) = aux_tree.gen_auxiliary_tree(vertices) else {
            continue;
        };
        let get_idx = |v| {
            new_vertices
                .binary_search(&v)
                .unwrap_or_else(|e| panic!("v = {}, e = {}", v, e))
        };
        let is_base = new_vertices
            .iter()
            .map(|&i| color_list[i] == color)
            .collect::<Vec<_>>();
        let new_graph = {
            let mut new_graph = vec![vec![]; new_vertices.len()];
            for (p, v) in par_v {
                let diff = aux_tree.euler_tour.depth[p].abs_diff(aux_tree.euler_tour.depth[v]);
                let p = get_idx(p);
                let v = get_idx(v);
                new_graph[p].push((v, diff));
            }
            new_graph
        };
        struct Cls<'a> {
            ans: usize,
            is_base: &'a [bool],
            all_size: &'a usize,
            graph: &'a [Vec<(usize, usize)>],
        }
        fn dfs(cls: &mut Cls, v: usize) -> usize {
            let mut ret = if cls.is_base[v] { 1 } else { 0 };
            for &(chd, diff) in &cls.graph[v] {
                let size = dfs(cls, chd);
                cls.ans += size * (cls.all_size - size) * diff;
                ret += size;
            }
            ret
        }
        let mut cls = Cls {
            ans: 0,
            is_base: &is_base,
            all_size: &all_size,
            graph: &new_graph,
        };
        dfs(&mut cls, get_idx(root));
        ans += cls.ans;
    }
    println!("{}", ans);
}
