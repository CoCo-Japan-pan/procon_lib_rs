//! `Algrebra`では、データ構造に乗せる代数構造のtraitを提供します。
use std::fmt::Debug;

/// 可換  
pub trait Commutative {}
/// 非可換
pub trait NonCommutative {}

/// 作用  
/// 作用自体もモノイドであることを要求  
/// 作用素を合成させてから作用させるのと、作用素を一つ一つ作用させる結果が同じであることを要求
pub trait Action: Debug + Clone {
    /// 作用の対象
    type Target: Clone;
    /// 恒等写像
    fn id_action() -> Self;
    /// 作用の合成(selfが先、rhsが後)  
    /// (atcoder libraryとは作用の順が逆なので注意)
    fn composition(&mut self, rhs: &Self);
    /// 作用の適用
    fn apply(&self, target: &mut Self::Target);
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
/// typeのMonoid,Actionだけ指定することを想定(メソッドのオーバーライドはしないでください)  
pub trait ActionMonoid {
    /// 作用の対象のモノイド
    type Monoid: Monoid;
    /// 作用素のモノイド
    type Action: Action<Target = <Self::Monoid as Monoid>::Target>;
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
    fn id_action() -> Self::Action {
        Self::Action::id_action()
    }
    /// 作用の合成(fが先、gが後)
    fn composition(f: &mut Self::Action, g: &Self::Action) {
        f.composition(g)
    }
    /// 作用の適用
    fn apply(x: &mut <Self::Monoid as Monoid>::Target, f: &Self::Action) {
        f.apply(x)
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

/// 半環  
/// 加算は可換モノイド  
/// 乗算はモノイド  
/// 乗算は加法に対して分配法則を満たす a*(b+c) = a*b + a*c, (a+b)*c = a*c + b*c  
/// 加算の単位元は乗算の零元 0*a=a*0=0
pub trait Semiring: Debug + Clone + Eq {
    type Target: Debug + Clone + Eq;
    fn zero() -> Self::Target;
    fn one() -> Self::Target;
    fn add_assign(a: &mut Self::Target, b: &Self::Target);
    fn mul(a: &Self::Target, b: &Self::Target) -> Self::Target;
}
