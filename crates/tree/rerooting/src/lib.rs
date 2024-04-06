use algebra::{Commutative, Monoid};

pub trait Rerootable {
    /// DPテーブルに載せる可換モノイド
    type DPMonoid: Monoid + Commutative;
    /// 葉に入れる値(デフォルトでは単位元)  
    /// 単位元以外を入れたい場合はオーバーライドしてください
    #[allow(unused_variables)]
    fn leaf(vertex: usize) -> <Self::DPMonoid as Monoid>::Target {
        <Self::DPMonoid as Monoid>::id_element()
    }
    /// 部分木に頂点v→pの辺を追加する
    fn add_root(
        subtree: <Self::DPMonoid as Monoid>::Target,
        subtree_root: usize,
        new_root: usize,
    ) -> <Self::DPMonoid as Monoid>::Target;
    /// add_rootによりできた「部分木+一辺」同士をmergeする  
    /// これはオーバーライドしないでください(モノイドの二項演算を用いる)
    fn merge(
        subtree1: &<Self::DPMonoid as Monoid>::Target,
        subtree2: &<Self::DPMonoid as Monoid>::Target,
    ) -> <Self::DPMonoid as Monoid>::Target {
        <Self::DPMonoid as Monoid>::binary_operation(subtree1, subtree2)
    }
}
