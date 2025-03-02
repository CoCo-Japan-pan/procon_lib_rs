use mincost_bflow::MinCostBFlow;
use proconio::{fastout, input};

// capacityが負の場合にまだ対応できてない(TODO)
// 容量スケーリングもまだできてない(TODO)

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [i128; n],
        s_t_l_u_c: [(usize, usize, i128, i128, i128); m],
    }
    let mut mcf = MinCostBFlow::new(n);
    for (i, b) in b.into_iter().enumerate() {
        if b < 0 {
            mcf.add_demand(i, -b);
        } else if b > 0 {
            mcf.add_supply(i, b);
        }
    }
    for (s, t, l, u, c) in s_t_l_u_c {
        mcf.add_edge(s, t, l, u, c);
    }
    if let Some(res) = mcf.mincost_bflow() {
        println!("{}", res.cost);
        for &v in &res.potential {
            println!("{}", v);
        }
        for &f in &res.flow {
            println!("{}", f);
        }
    } else {
        println!("infeasible")
    }
}
