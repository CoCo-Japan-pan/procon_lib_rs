// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use algebra::{Group, Monoid};
use potentialized_unionfind::PotentializedUnionFind;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as MInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct M2([MInt; 4]);

impl Monoid for M2 {
    type Target = Self;
    fn id_element() -> Self::Target {
        M2([MInt::new(1), MInt::new(0), MInt::new(0), MInt::new(1)])
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        let mut res = [MInt::new(0); 4];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res[i * 2 + j] += a.0[i * 2 + k] * b.0[k * 2 + j];
                }
            }
        }
        M2(res)
    }
}

impl Group for M2 {
    fn inverse(a: &Self::Target) -> Self::Target {
        M2([a.0[3], -a.0[1], -a.0[2], a.0[0]])
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut puf = PotentializedUnionFind::<M2>::new(n);
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 0 {
            input! {
                u: usize,
                v: usize,
                x: [MInt; 4],
            }
            let diff = {
                let mut ret = [MInt::new(0); 4];
                for i in 0..4 {
                    ret[i] = x[i];
                }
                M2(ret)
            };
            if puf.relate(v, u, diff).is_err() {
                println!("0");
            } else {
                println!("1");
            }
        } else {
            input! {
                u: usize,
                v: usize,
            }
            let ans = puf.diff(v, u);
            if let Some(ans) = ans {
                println!("{} {} {} {}", ans.0[0], ans.0[1], ans.0[2], ans.0[3]);
            } else {
                println!("-1");
            }
        }
    }
}
