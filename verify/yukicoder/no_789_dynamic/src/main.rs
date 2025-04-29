// verification-helper: PROBLEM https://yukicoder.me/problems/no/789

use algebra::Monoid;
use dynamic_segtree::DynamicSegTree;
use proconio::{fastout, input};

struct SumMonoid;
impl Monoid for SumMonoid {
    type Target = usize;
    fn id_element() -> Self::Target {
        0
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        a + b
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        t_x_y: [(u8, usize, usize); n],
    }
    let mut seg = DynamicSegTree::<SumMonoid>::new(1000_000_010);
    let mut ans = 0;
    for (t, x, y) in t_x_y {
        if t == 0 {
            let pre_val = seg.get(x);
            seg.set(x, pre_val + y);
        } else {
            ans += seg.prod(x..=y);
        }
    }
    println!("{}", ans);
}
