// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
use algebra::{IdempotentMonoid, Monoid};
use proconio::{fastout, input};
use sparse_table::SparseTable;

pub enum MinMonoid {}
impl Monoid for MinMonoid {
    type Target = u32;
    fn id_element() -> Self::Target {
        u32::MAX
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}
impl IdempotentMonoid for MinMonoid {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }
    let st = SparseTable::<MinMonoid>::new(a);
    for _ in 0..q {
        input! { l: usize, r: usize }
        println!("{}", st.prod(l..r));
    }
}
