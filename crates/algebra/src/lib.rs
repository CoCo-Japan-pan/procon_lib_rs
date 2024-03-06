//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 作用  
/// 作用素を合成させてから作用させるのと、作用素を一つ一つ作用させる結果が同じであることを要求
pub trait Map: Debug + Clone + PartialEq + Eq {
    /// 作用の対象
    type Target: Debug + Clone + PartialEq + Eq;
    /// 恒等写像
    fn id_map() -> Self;
    /// 作用の合成(selfが先、rhsが後)
    fn composition(&mut self, rhs: &Self);
    /// 作用の適用
    fn mapping(&self, target: &Self::Target) -> Self::Target;
}

/// 可換な作用
pub trait CommutativeMap: Map {}

/// 非可換な作用
pub trait NonCommutativeMap: Map {}

/// モノイド
pub trait Monoid: Debug + Clone + PartialEq + Eq {
    /// 単位元
    fn id_element() -> Self;
    /// 二項演算
    fn binary_operation(a: &Self, b: &Self) -> Self;
}

/// MもF(作用)もモノイドであることを要求  
/// 作用素を合成させてから作用させるのと、作用素を一つ一つ作用させる結果が同じであることを要求  
/// 自己準同型性を要求  
/// つまり作用素を区間に適用するときに複数の区間に分割して適用しても結果が同じであることを要求
pub trait MapMonoid {
    type M: Monoid;
    type F: Map<Target = Self::M>;
    /// 単位元
    fn id_element() -> Self::M {
        Self::M::id_element()
    }
    /// 二項演算
    fn binary_operation(a: &Self::M, b: &Self::M) -> Self::M {
        Self::M::binary_operation(a, b)
    }
    /// 恒等写像
    fn id_map() -> Self::F {
        Self::F::id_map()
    }
    /// 作用の合成(fが先、gが後)
    fn composition(f: &mut Self::F, g: &Self::F) {
        f.composition(g)
    }
    /// 作用の適用
    fn mapping(f: &Self::F, x: &Self::M) -> Self::M {
        f.mapping(x)
    }
}
