// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use cartesian_tree::CartesianTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let tree = CartesianTree::new(&a, true);
    for i in 0..n {
        print!("{}", tree.parent[i].unwrap_or(i));
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
