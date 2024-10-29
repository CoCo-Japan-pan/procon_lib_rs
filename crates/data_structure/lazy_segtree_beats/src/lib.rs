//! <https://nyaannyaan.github.io/library/segment-tree/segment-tree-beats-abstract.hpp>
//! をもとにしています  
//! 失敗せずに作用を適用できるものについてのみ`push`による遅延伝播を行い、失敗するものについてはボトムアップに計算する  
//! 作用の失敗回数によい上界が存在するように設計する必要がある  
//! 作用の成功部分を部分的に伝播させたいので、作用の合成は定義せず、
//! 成功した作用の情報を載せたノードからその子ノードへの`push`を定義する  
//! つまりノードは作用のうち成功している(=伝播可能な)部分的な情報をモノイドに加えて持つ  
//! 作用については、`apply`で定義し、成功したら`true`、失敗したら`false`を返すようにする

use internal_bits::ceil_log2;
use std::ops::{Bound::*, RangeBounds};

/// Segment Tree Beats における内部のノード  
/// モノイド構造を持ちつつ、部分的な作用の伝播も行う
pub trait BeatsNode: Clone {
    type Action;
    fn id_node() -> Self;
    fn binary_operation(lhs: &Self, rhs: &Self) -> Self;
    /// 成功した作用の情報を載せたノードからその子ノードへの伝播
    fn push(&self, child_node: &mut Self);
    /// 作用の適用 成功したら`true`、失敗したら`false`を返す  
    /// 区間の長さ1に対しては必ず成功する
    fn apply(&mut self, action: &Self::Action) -> bool;
}

#[derive(Debug)]
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
    pub fn prod<R: RangeBounds<usize>>(&mut self, range: R) -> Node {
        let (mut l, mut r) = self.get_range(range);
        if l == r {
            return Node::id_node();
        }
        l += self.leaf_size;
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let mut sml = Node::id_node();
        let mut smr = Node::id_node();
        while l < r {
            if l & 1 != 0 {
                sml = Node::binary_operation(&sml, &self.nodes[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = Node::binary_operation(&self.nodes[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        Node::binary_operation(&sml, &smr)
    }

    pub fn apply_range<R: RangeBounds<usize>>(&mut self, range: R, action: &Node::Action) {
        let (mut l, mut r) = self.get_range(range);
        if l == r {
            return;
        }
        l += self.leaf_size;
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l_copy = l;
            let r_copy = r;
            while l < r {
                if l & 1 != 0 {
                    self.apply(l, action);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.apply(r, action);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l_copy;
            r = r_copy;
        }
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn get_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        (l, r)
    }

    fn push(&mut self, i: usize) {
        let ptr = self.nodes.as_mut_ptr();
        unsafe {
            self.nodes[i].push(&mut *ptr.add(i << 1));
            self.nodes[i].push(&mut *ptr.add((i << 1) | 1));
        }
    }
    fn update(&mut self, i: usize) {
        self.nodes[i] = Node::binary_operation(&self.nodes[i << 1], &self.nodes[(i << 1) | 1]);
    }
    fn apply(&mut self, i: usize, action: &Node::Action) {
        let res = self.nodes[i].apply(action);
        // 作用が失敗したら子ノードに任せてボトムアップに計算
        if (i < self.leaf_size) && !res {
            self.push(i);
            self.apply(i << 1, action);
            self.apply((i << 1) | 1, action);
            self.update(i);
        }
    }
}
