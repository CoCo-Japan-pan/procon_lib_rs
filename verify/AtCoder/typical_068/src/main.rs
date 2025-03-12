//! https://atcoder.jp/contests/typical90/tasks/typical90_bp

use algebra::{Group, Monoid};
use potentialized_unionfind::PotentializedUnionFind;
use proconio::{fastout, input, marker::Usize1};

/// y = +-x + bias を表す群
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AffineGroup {
    keisuu: bool,
    bias: i64,
}

impl AffineGroup {
    fn apply(&self, x: i64) -> i64 {
        if self.keisuu {
            x + self.bias
        } else {
            self.bias - x
        }
    }
}

impl Monoid for AffineGroup {
    type Target = Self;
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        AffineGroup {
            keisuu: !(a.keisuu ^ b.keisuu),
            bias: if b.keisuu {
                b.bias + a.bias
            } else {
                b.bias - a.bias
            },
        }
    }
    fn id_element() -> Self::Target {
        Self {
            keisuu: true,
            bias: 0,
        }
    }
}

impl Group for AffineGroup {
    fn inverse(a: &Self::Target) -> Self::Target {
        Self {
            keisuu: a.keisuu,
            bias: if a.keisuu { -a.bias } else { a.bias },
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = PotentializedUnionFind::<AffineGroup>::new(n);
    for _ in 0..q {
        input! {
            t: u8,
            x: Usize1,
            y: Usize1,
            v: i64,
        }
        if t == 0 {
            // y = -x + v
            uf.relate(
                x,
                y,
                AffineGroup {
                    keisuu: false,
                    bias: v,
                },
            )
            .unwrap();
        } else {
            let diff = uf.diff(x, y);
            if let Some(diff) = diff {
                println!("{}", diff.apply(v));
            } else {
                println!("Ambiguous");
            }
        }
    }
}
