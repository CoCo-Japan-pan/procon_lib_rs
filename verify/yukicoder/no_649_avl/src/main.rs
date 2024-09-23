// verification-helper: PROBLEM https://yukicoder.me/problems/no/649

use avl::AVL;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        k: usize,
    }
    let mut multiset = AVL::new(true);
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                v: u64,
            }
            multiset.insert(v);
        } else {
            if multiset.len() < k {
                println!("-1");
            } else {
                let ans = multiset.erase_index(k - 1).unwrap();
                println!("{}", ans);
            }
        }
    }
}
