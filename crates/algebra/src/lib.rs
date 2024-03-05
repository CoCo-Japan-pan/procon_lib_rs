//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 作用
pub trait Map: Debug + Clone + PartialEq + Eq {
    /// 作用の対象
    type Target: Debug + Clone + PartialEq + Eq;
    /// 恒等写像
    fn id() -> Self;
    /// 作用の合成
    fn composition(&mut self, rhs: &Self);
    /// 作用の適用
    fn mapping(&self, target: &Self::Target) -> Self::Target;
}

/// 可換な作用
pub trait CommutativeMap: Map {}

/// 非可換な作用
pub trait NonCommutativeMap: Map {}
