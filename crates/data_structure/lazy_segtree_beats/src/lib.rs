//! <https://nyaannyaan.github.io/library/segment-tree/segment-tree-beats-abstract.hpp>
//! をもとにしています  
//! 失敗せずに作用を適用できるものについてのみ`push`による遅延伝播を行い、失敗するものについてはボトムアップに計算する  
//! 作用の失敗回数によい上界が存在するように設計する必要がある  
//! 作用の成功部分を部分的に伝播させたいので、作用の合成は定義せず、
//! 成功した作用の情報を載せたノードからその子ノードへの`push`を定義する  
//! 作用については、`apply`で定義し、成功したら`true`、失敗したら`false`を返すようにする

use internal_bits::ceil_log2;

/// Segment Tree Beats における内部のノード  
/// モノイド構造を持ちつつ、部分的な作用の伝播も行う
pub trait BeatsNode: Clone {
    type Action;
    fn id_node() -> Self;
    fn binary_operation(lhs: &Self, rhs: &Self) -> Self;
    /// 成功した作用の情報を載せたノードからその子ノードへの伝播
    fn push(&self, child_node: &mut Self);
    /// 作用の適用 成功したら`true`、失敗したら`false`を返す  
    /// 区間の長さ1にたいしては必ず成功する
    fn apply(&mut self, action: &Self::Action) -> bool;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LazySegtreeBeats<Node: BeatsNode> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    nodes: Vec<Node>,
}

impl<Node: BeatsNode> From<Vec<Node>> for LazySegtreeBeats<Node> {
    fn from(mut v: Vec<Node>) -> Self {
        let range_size = v.len();
        let log = ceil_log2(range_size as u32) as usize;
        let leaf_size = 1 << log;
        let mut nodes = vec![Node::id_node(); leaf_size];
        nodes.reserve(leaf_size);
        nodes.append(&mut v);
        nodes.append(&mut vec![Node::id_node(); leaf_size - range_size]);
        let mut ret = Self {
            range_size,
            leaf_size,
            log,
            nodes,
        };
        for i in (1..leaf_size).rev() {
            ret.update(i);
        }
        ret
    }
}

impl<Node: BeatsNode> LazySegtreeBeats<Node> {
    fn update(&mut self, k: usize) {
        self.nodes[k] = Node::binary_operation(&self.nodes[k << 1], &self.nodes[(k << 1) | 1]);
    }
}
