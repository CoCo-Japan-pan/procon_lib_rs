//! # Algebra
//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。

/// 可換な作用
pub trait CommutativeMap {
    /// 恒等写像
    fn id() -> Self;
    /// 作用の合成
    fn compostion(a: &Self, b: &Self) -> Self;
}
