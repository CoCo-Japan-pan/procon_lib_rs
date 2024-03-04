//! # Algebra
//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 可換な作用
pub trait CommutativeMap: Debug + Clone + PartialEq + Eq {
    /// 恒等写像
    fn id() -> Self;
    /// 作用の合成 可換
    fn compostion(a: &Self, b: &Self) -> Self;
}
