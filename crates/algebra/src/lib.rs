//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 可換  
pub trait Commutative {}
/// 非可換
pub trait NonCommutative {}

/// 作用  
/// 作用自体もモノイドであることを要求  
/// 作用素を合成させてから作用させるのと、作用素を一つ一つ作用させる結果が同じであることを要求
pub trait Map: Clone {
    /// 作用の対象
    type Target: Clone;
    /// 恒等写像
    fn id_map() -> Self;
    /// 作用の合成(selfが先、rhsが後)
    fn composition(&mut self, rhs: &Self);
    /// 作用の適用
    fn mapping(&self, target: &mut Self::Target);
}

/// モノイド
pub trait Monoid {
    /// モノイドの要素
    type Target: Debug + Clone + Eq;
    /// 単位元
    fn id_element() -> Self::Target;
    /// 二項演算
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target;
}

/// 自己準同型性を要求  
/// つまり区間和への適用と、各要素への適用の区間和が一致することを要求  
/// typeのMonoid,Mapだけ指定することを想定(メソッドのオーバーライドはしないでください)  
pub trait MapMonoid {
    /// 作用の対象のモノイド
    type Monoid: Monoid;
    /// 作用素のモノイド
    type Map: Map<Target = <Self::Monoid as Monoid>::Target>;
    /// 単位元
    fn id_element() -> <Self::Monoid as Monoid>::Target {
        Self::Monoid::id_element()
    }
    /// 二項演算
    fn binary_operation(
        a: &<Self::Monoid as Monoid>::Target,
        b: &<Self::Monoid as Monoid>::Target,
    ) -> <Self::Monoid as Monoid>::Target {
        Self::Monoid::binary_operation(a, b)
    }
    /// 恒等写像
    fn id_map() -> Self::Map {
        Self::Map::id_map()
    }
    /// 作用の合成(fが先、gが後)
    fn composition(f: &mut Self::Map, g: &Self::Map) {
        f.composition(g)
    }
    /// 作用の適用
    fn mapping(x: &mut <Self::Monoid as Monoid>::Target, f: &Self::Map) {
        f.mapping(x)
    }
}

/// 冪等なモノイド  
/// つまり x = x op x が成り立つようなモノイド  
/// SparseTableに乗る
pub trait IdempotentMonoid: Monoid {}

/// 群   
/// モノイドに加えて、逆元を持つ  
pub trait Group: Monoid {
    fn inverse(a: &Self::Target) -> Self::Target;
}
