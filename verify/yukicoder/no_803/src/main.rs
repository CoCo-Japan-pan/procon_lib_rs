// verification-helper: PROBLEM https://yukicoder.me/problems/no/803

use bit_matrix::BitMatrix;
use proconio::{fastout, input, marker::Usize1};
use static_modint::ModInt1000000007 as MInt;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: u32,
        a: [u32; n],
        t_l_r: [(u8, Usize1, Usize1); m],
    }
    let (mat, b) = {
        let mut mat = BitMatrix::new(32 + m, n);
        for (i, a) in a.into_iter().enumerate() {
            for bit in 0..32 {
                if ((a >> bit) & 1) > 0 {
                    mat.set(bit, i, true);
                }
            }
        }
        let mut b = vec![false; 32 + m];
        for bit in 0..32 {
            if ((x >> bit) & 1) > 0 {
                b[bit] = true;
            }
        }
        for (i, (t, l, r)) in t_l_r.into_iter().enumerate() {
            let i = i + 32;
            b[i] = t == 1;
            for j in l..=r {
                mat.set(i, j, true);
            }
        }
        (mat, b)
    };
    let ans = if let Some((free_dom, _)) = mat.linear_equation(&b) {
        MInt::new(2).pow(free_dom as u64)
    } else {
        MInt::new(0)
    };
    println!("{}", ans);
}
