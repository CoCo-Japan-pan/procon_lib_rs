//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 作用
pub trait Map: Debug + Clone + PartialEq + Eq {
    /// 恒等写像
    fn id() -> Self;
    /// 作用の合成
    fn compostion(&mut self, rhs: &Self);
}

/// 可換な作用
pub trait CommutativeMap: Map {}

/// 非可換な作用
pub trait NonCommutativeMap: Map {}
