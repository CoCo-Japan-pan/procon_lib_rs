// https://atcoder.jp/contests/typical90/tasks/typical90_be

use bit_matrix::BitMatrix;
use proconio::{fastout, input, marker::Usize1};
use static_modint::ModInt998244353 as MInt;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let a = {
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            input! {
                t: usize,
                list: [Usize1; t],
            }
            a.push(list);
        }
        a
    };
    let mat = {
        let mut mat = BitMatrix::new(m, n);
        for (i, list) in a.into_iter().enumerate() {
            for &j in list.iter() {
                mat.set(j, i, true);
            }
        }
        mat
    };
    input! {
        s: [usize; m],
    }
    let b = (0..m).into_iter().map(|i| s[i] == 1).collect::<Vec<_>>();
    if let Some((freedom, _)) = mat.linear_equation(&b) {
        let ans = MInt::new(2).pow(freedom as u64);
        println!("{}", ans);
    } else {
        println!("0");
    }
}
