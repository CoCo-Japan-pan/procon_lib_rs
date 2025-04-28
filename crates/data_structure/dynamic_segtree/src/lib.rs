//! 動的セグメント木 必要なノードのみ確保する  
//! 単位元で初期化する  
//! <https://lorent-kyopro.hatenablog.com/entry/2021/03/12/025644>

use algebra::Monoid;
use std::ops::RangeBounds;

struct Node<M: Monoid> {
    left_child: Option<Box<Node<M>>>,
    right_child: Option<Box<Node<M>>>,
    index: usize,
    value: M::Target,
    product: M::Target,
}

impl<M: Monoid> Node<M> {
    fn new(index: usize, value: M::Target) -> Self {
        Self {
            left_child: None,
            right_child: None,
            index,
            product: value.clone(),
            value,
        }
    }

    fn update(&mut self) {
        let id_element = M::id_element();
        let left_value = self
            .left_child
            .as_ref()
            .map_or(&id_element, |node| &node.product);
        let right_value = self
            .right_child
            .as_ref()
            .map_or(&id_element, |node| &node.product);
        self.product =
            M::binary_operation(&M::binary_operation(left_value, &self.value), right_value);
    }
}

type NodePtr<M> = Option<Box<Node<M>>>;

fn set_node<M: Monoid>(
    node: &mut NodePtr<M>,
    mut index: usize,
    mut value: M::Target,
    left: usize,
    right: usize,
) {
    if node.is_none() {
        // 余計に潜らず、メモリ確保を高々1回に抑える
        *node = Some(Box::new(Node::new(index, value)));
        return;
    }
    let mut node = node.as_mut().unwrap();
    if node.index == index {
        node.value = value;
        node.update();
        return;
    }
    let half = (left + right) >> 1;
    if index < half {
        // 左に行くほどindexが小さくなる条件を満たすようにする
        if index > node.index {
            std::mem::swap(&mut node.value, &mut value);
            std::mem::swap(&mut node.index, &mut index);
        }
        set_node(&mut node.left_child, index, value, left, half);
    } else {
        // 右に行くほどindexが大きくなる条件を満たすようにする
        if index < node.index {
            std::mem::swap(&mut node.value, &mut value);
            std::mem::swap(&mut node.index, &mut index);
        }
        set_node(&mut node.right_child, index, value, half, right);
    }
    node.update();
}

fn get_node<M: Monoid>(node: &NodePtr<M>, index: usize, left: usize, right: usize) -> M::Target {
    if node.is_none() {
        return M::id_element();
    }
    let node = node.as_ref().unwrap();
    if node.index == index {
        return node.value.clone();
    }
    let half = (left + right) >> 1;
    if index < half {
        get_node(&node.left_child, index, left, half)
    } else {
        get_node(&node.right_child, index, half, right)
    }
}

fn prod_node<M: Monoid>(
    node: &NodePtr<M>,
    prod_left: usize,
    prod_right: usize,
    left: usize,
    right: usize,
) -> M::Target {
    if node.is_none() || right <= prod_left || prod_right <= left {
        return M::id_element();
    }
    let node = node.as_ref().unwrap();
    if prod_left <= left && right <= prod_right {
        return node.product.clone();
    }
    let half = (left + right) >> 1;
    let mut res = M::id_element();
    res = M::binary_operation(
        &res,
        &prod_node(&node.left_child, prod_left, prod_right, left, half),
    );
    if prod_left <= node.index && node.index < prod_right {
        res = M::binary_operation(&res, &node.value);
    }
    res = M::binary_operation(
        &res,
        &prod_node(&node.right_child, prod_left, prod_right, half, right),
    );
    res
}

pub struct DynamicSegTree<M: Monoid> {
    range_size: usize,
    root_node: NodePtr<M>,
}

impl<M: Monoid> DynamicSegTree<M> {
    /// range_size個の単位元で初期化
    pub fn new(range_size: usize) -> Self {
        Self {
            range_size,
            root_node: None,
        }
    }

    pub fn set(&mut self, index: usize, value: M::Target) {
        assert!(index < self.range_size);
        set_node(&mut self.root_node, index, value, 0, self.range_size);
    }

    pub fn get(&self, index: usize) -> M::Target {
        assert!(index < self.range_size);
        get_node(&self.root_node, index, 0, self.range_size)
    }

    pub fn all_prod(&self) -> M::Target {
        self.root_node
            .as_ref()
            .map_or(M::id_element(), |node| node.product.clone())
    }

    pub fn prod<R: RangeBounds<usize>>(&self, range: R) -> M::Target {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
        };
        assert!(start <= end && end <= self.range_size);
        if start == 0 && end == self.range_size {
            return self.all_prod();
        }
        prod_node(&self.root_node, start, end, 0, self.range_size)
    }
}
