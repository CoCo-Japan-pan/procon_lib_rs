//! [kd-tree](https://trap.jp/post/1489/)  
//! ここでは2次元だけで、矩形範囲クエリのみをサポートします  
//! また、点の追加と削除はできません 使わない間は単位元でも与えておいてください(オフライン前提)
//! 各点に可換モノイドを乗せて、矩形範囲の区間和を求めることができます  
//! また、可換な作用を遅延セグ木のように伝播させることもできます  
#[allow(dead_code)]
#[derive(Debug)]
struct KdTree {
    left: Option<Box<KdTree>>,
    right: Option<Box<KdTree>>,
}
