// verification-helper: PROBLEM https://atcoder.jp/contests/practice2/tasks/practice2_l

use lazy_segtree::LazySegTree;
use proconio::{fastout, input, marker::Usize1};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InvNum {
    inv_num: u64,
    zero_num: u64,
    one_num: u64,
}

impl InvNum {
    fn new(num: u32) -> Self {
        if num == 0 {
            InvNum {
                inv_num: 0,
                zero_num: 1,
                one_num: 0,
            }
        } else {
            InvNum {
                inv_num: 0,
                zero_num: 0,
                one_num: 1,
            }
        }
    }
}

impl algebra::Monoid for InvNum {
    type Target = Self;
    fn id_element() -> Self::Target {
        InvNum {
            inv_num: 0,
            zero_num: 0,
            one_num: 0,
        }
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        InvNum {
            inv_num: a.inv_num + b.inv_num + a.one_num * b.zero_num,
            zero_num: a.zero_num + b.zero_num,
            one_num: a.one_num + b.one_num,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FlipMap {
    flip: bool,
}
impl algebra::Action for FlipMap {
    type Target = InvNum;
    fn id_map() -> Self {
        FlipMap { flip: false }
    }
    fn composition(&mut self, rhs: &Self) {
        self.flip ^= rhs.flip;
    }
    fn mapping(&self, target: &mut Self::Target) {
        if self.flip {
            *target = InvNum {
                inv_num: target.zero_num * target.one_num - target.inv_num,
                zero_num: target.one_num,
                one_num: target.zero_num,
            }
        }
    }
}
impl algebra::Commutative for FlipMap {}
struct MyMapMonoid;
impl algebra::ActionMonoid for MyMapMonoid {
    type Monoid = InvNum;
    type Action = FlipMap;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }
    let mut lazy_seg =
        LazySegTree::<MyMapMonoid>::from(a.iter().map(|&x| InvNum::new(x)).collect::<Vec<_>>());
    for _ in 0..q {
        input! {t: u32, l: Usize1, r: Usize1}
        match t {
            1 => {
                lazy_seg.apply_range_commutative(l..=r, &FlipMap { flip: true });
            }
            2 => {
                println!("{}", lazy_seg.prod(l..=r).inv_num);
            }
            _ => unreachable!(),
        }
    }
}
