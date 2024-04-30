// verification-helper: PROBLEM https://atcoder.jp/contests/abc293/tasks/abc293_e

use algebra::Semiring;
use dynamic_modint::{define_modint, DynamicModInt};
use matrix::Matrix;
use proconio::input;

define_modint!(MOD);
type MInt = DynamicModInt<MOD>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UsualSemiring;
impl Semiring for UsualSemiring {
    type Target = MInt;
    fn zero() -> Self::Target {
        MInt::raw(0)
    }
    fn one() -> Self::Target {
        MInt::new(1)
    }
    fn add_assign(a: &mut Self::Target, b: &Self::Target) {
        *a += *b;
    }
    fn mul(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a * *b
    }
}

fn main() {
    input! {
        a: u32, x: u64, m: u32,
    }
    MInt::set_modulus(m);
    let keisuu = vec![
        vec![MInt::new(a), MInt::new(1)],
        vec![MInt::new(0), MInt::new(1)],
    ];
    let keisuu = Matrix::<UsualSemiring>::from(keisuu);
    let keisuu = keisuu.pow(x - 1);
    let ans = keisuu * &Matrix::from(vec![vec![MInt::new(1)], vec![MInt::new(1)]]);
    println!("{}", ans.get(0, 0));
}
