//! [AVL木](https://qiita.com/QCFium/items/3cf26a6dc2d49ef490d7)  
//! `std::collections::BTreeSet` と異なり、k番目の値を`O(logN)`で取り出せる  
//! 列を管理する

use std::cmp::Ordering;
use std::fmt::Display;
use std::mem::swap;
type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    left: Tree<T>,
    right: Tree<T>,
    value: T,
    len: usize,
    height: u8,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Self {
            left: None,
            right: None,
            value,
            len: 1,
            height: 1,
        }
    }
    fn update(&mut self) {
        self.len = len(&self.left) + len(&self.right) + 1;
        self.height = height(&self.left).max(height(&self.right)) + 1;
    }
    fn rotate_right(&mut self) {
        let mut x = self.left.take().unwrap();
        let b = x.right.take();
        swap(self, &mut x);
        x.left = b;
        x.update();
        self.right = Some(x);
        self.update();
    }
    fn rotate_left(&mut self) {
        let mut x = self.right.take().unwrap();
        let b = x.left.take();
        swap(self, &mut x);
        x.right = b;
        x.update();
        self.left = Some(x);
        self.update();
    }
    fn balance(&mut self) {
        self.update();
        if height(&self.left).abs_diff(height(&self.right)) <= 1 {
            return;
        }
        if height(&self.left) > height(&self.right) {
            // 左の子の右が重ければ左回転
            let left_child = self.left.as_mut().unwrap();
            if height(&left_child.left) < height(&left_child.right) {
                left_child.rotate_left();
            }
            self.rotate_right();
        } else {
            // 右の子の左が重ければ右回転
            let right_child = self.right.as_mut().unwrap();
            if height(&right_child.left) > height(&right_child.right) {
                right_child.rotate_right();
            }
            self.rotate_left();
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(left) = &self.left {
            write!(f, "{}", left)?;
        }
        write!(f, "{}, ", self.value)?;
        if let Some(right) = &self.right {
            write!(f, "{}", right)?;
        }
        Ok(())
    }
}

fn len<T>(tree: &Tree<T>) -> usize {
    tree.as_ref().map_or(0, |t| t.len)
}

fn height<T>(tree: &Tree<T>) -> u8 {
    tree.as_ref().map_or(0, |t| t.height)
}

fn remove_rightest<T>(tree: &mut Tree<T>) -> Tree<T> {
    if tree.is_none() {
        return None;
    }
    let tree_in = tree.as_mut().unwrap();
    if tree_in.right.is_some() {
        let rightest = remove_rightest(&mut tree_in.right);
        tree_in.balance();
        rightest
    } else {
        let mut left = tree_in.left.take();
        tree_in.update();
        swap(&mut left, tree);
        left
    }
}

fn merge<T>(mut left: Tree<T>, right: Tree<T>) -> Tree<T> {
    match (left.is_some(), right.is_some()) {
        (true, true) => {
            let rightest = remove_rightest(&mut left);
            merge_with_root(left, *rightest.unwrap(), right)
        }
        (false, _) => right,
        (_, false) => left,
    }
}

fn merge_with_root<T>(mut left: Tree<T>, root: Node<T>, mut right: Tree<T>) -> Tree<T> {
    if height(&left).abs_diff(height(&right)) <= 1 {
        let mut new_node = Node {
            left,
            right,
            value: root.value,
            len: 1,
            height: 1,
        };
        new_node.update();
        return Some(Box::new(new_node));
    }
    if height(&left) > height(&right) {
        let mut left = left.take().unwrap();
        let new_left_right = merge_with_root(left.right, root, right);
        left.right = new_left_right;
        left.balance();
        Some(left)
    } else {
        let mut right = right.take().unwrap();
        let new_right_left = merge_with_root(left, root, right.left);
        right.left = new_right_left;
        right.balance();
        Some(right)
    }
}

/// split into [0, index), [index, n)
fn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>, Tree<T>) {
    let Some(mut node) = tree else {
        return (None, None);
    };
    let left = node.left.take();
    let right = node.right.take();
    node.update();
    let left_size = len(&left);
    match index.cmp(&left_size) {
        Ordering::Equal => (left, merge_with_root(None, *node, right)),
        Ordering::Less => {
            let tmp = split(left, index);
            (tmp.0, merge_with_root(tmp.1, *node, right))
        }
        Ordering::Greater => {
            let tmp = split(right, index - left_size - 1);
            (merge_with_root(left, *node, tmp.0), tmp.1)
        }
    }
}

fn insert_by_idx<T>(tree: Tree<T>, index: usize, value: T) -> Tree<T> {
    assert!(index <= len(&tree));
    let new_node = Node::new(value);
    if tree.is_none() {
        return Some(Box::new(new_node));
    };
    let (left, right) = split(tree, index);
    merge_with_root(left, new_node, right)
}

fn insert<T: PartialOrd>(tree: Tree<T>, value: T) -> Tree<T> {
    let index = lower_bound(&tree, &value);
    insert_by_idx(tree, index, value)
}

fn erase_by_idx<T>(tree: Tree<T>, index: usize) -> Tree<T> {
    assert!(index < len(&tree));
    let (left, right) = split(tree, index);
    let (_, right) = split(right, 1);
    merge(left, right)
}

fn erase<T: PartialOrd>(tree: Tree<T>, value: &T) -> Tree<T> {
    if count(&tree, value) == 0 {
        return tree;
    }
    let index = lower_bound(&tree, value);
    erase_by_idx(tree, index)
}

/// value以上の最初の値のindex
fn lower_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {
    let Some(tree) = tree else {
        return 0;
    };
    if value <= &tree.value {
        lower_bound(&tree.left, value)
    } else {
        len(&tree.left) + 1 + lower_bound(&tree.right, value)
    }
}

/// valueより大きい最初の値のindex
fn upper_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {
    let Some(tree) = tree else {
        return 0;
    };
    if value >= &tree.value {
        len(&tree.left) + 1 + upper_bound(&tree.right, value)
    } else {
        upper_bound(&tree.left, value)
    }
}

fn count<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {
    upper_bound(tree, value) - lower_bound(tree, value)
}

fn get<T>(tree: &Tree<T>, index: usize) -> Option<&T> {
    if len(tree) <= index {
        return None;
    }
    let Some(tree) = tree else {
        return None;
    };
    let left_len = len(&tree.left);
    match index.cmp(&left_len) {
        Ordering::Less => get(&tree.left, index),
        Ordering::Equal => Some(&tree.value),
        Ordering::Greater => get(&tree.right, index - left_len - 1),
    }
}

#[derive(Debug)]
pub struct AVL<T> {
    root: Tree<T>,
}

impl<T: Display> Display for AVL<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root)
        } else {
            write!(f, "Empty")
        }
    }
}

impl<T> AVL<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        len(&self.root)
    }

    pub fn height(&self) -> u8 {
        height(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn lower_bound(&self, value: &T) -> usize
    where
        T: PartialOrd,
    {
        lower_bound(&self.root, value)
    }

    pub fn upper_bound(&self, value: &T) -> usize
    where
        T: PartialOrd,
    {
        upper_bound(&self.root, value)
    }

    /// index番目(0-base)の値を取得
    pub fn get(&self, index: usize) -> Option<&T> {
        get(&self.root, index)
    }

    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        let inner = self.root.take();
        self.root = insert(inner, value);
    }

    pub fn erase(&mut self, value: &T)
    where
        T: PartialOrd,
    {
        let inner = self.root.take();
        self.root = erase(inner, value);
    }

    pub fn count(&self, value: &T) -> usize
    where
        T: PartialOrd,
    {
        count(&self.root, value)
    }
}

impl<T> Default for AVL<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use std::collections::{BTreeMap, BTreeSet};

    fn stop_watch() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        static mut START: f64 = 0.0;
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let current = time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9;
        unsafe {
            let ret = current - START;
            START = current;
            ret
        }
    }

    #[test]
    fn test_cnt() {
        let mut rng = thread_rng();
        let mut avl = AVL::<i32>::new();
        let mut set = BTreeMap::new();
        const SIZE: usize = 100000;
        for _ in 0..SIZE {
            let value = rng.gen_range(-100..=100);
            avl.insert(value);
            *set.entry(value).or_insert(0) += 1;
        }
        for cnt in 0..SIZE {
            if cnt % 2 == 0 {
                let value = rng.gen_range(-100..=100);
                let tree_cnt = set.get(&value).copied().unwrap_or(0);
                let rbst_cnt = avl.count(&value);
                assert_eq!(tree_cnt, rbst_cnt);
            } else if set.is_empty() || rng.gen() {
                let value = rng.gen_range(-100..=100);
                avl.insert(value);
                *set.entry(value).or_insert(0) += 1;
            } else {
                let value = rng.gen_range(-100..=100);
                avl.erase(&value);
                set.entry(value).and_modify(|x| *x -= 1);
                if let Some(x) = set.get(&value) {
                    if *x == 0 {
                        set.remove(&value);
                    }
                }
            }
        }
    }

    #[test]
    fn test_kth_by_no_dups() {
        let mut rng = thread_rng();
        let mut rbst = AVL::<i32>::new();
        let mut set = BTreeSet::new();
        for _ in 0..1000 {
            let value = rng.gen_range(-100..=100);
            if rbst.count(&value) == 0 {
                rbst.insert(value);
            }
            set.insert(value);
        }
        for cnt in 0..1000 {
            if cnt % 2 == 0 {
                let idx = rng.gen_range(0..set.len());
                let value = set.iter().nth(idx).unwrap();
                assert_eq!(rbst.get(idx).unwrap(), value);
            } else if set.is_empty() || rng.gen() {
                let value = rng.gen_range(-100..=100);
                if rbst.count(&value) == 0 {
                    rbst.insert(value);
                }
                set.insert(value);
            } else {
                let value = rng.gen_range(-100..=100);
                rbst.erase(&value);
                set.remove(&value);
            }
        }
    }

    #[test]
    fn test_bench() {
        const SIZE: usize = 250000;
        stop_watch();
        let mut set = BTreeSet::new();
        for i in 0..SIZE {
            set.insert(i);
        }
        println!("BTreeSet insert: {}", stop_watch());
        for i in 0..SIZE {
            set.remove(&i);
        }
        println!("BTreeSet erase: {}", stop_watch());
        let mut set = AVL::<usize>::new();
        for i in 0..SIZE {
            set.insert(i);
        }
        println!("AVL insert: {}", stop_watch());
        println!("AVL height: {}", set.height());
        for i in 0..SIZE {
            assert_eq!(set.get(i).unwrap(), &i);
        }
        println!("AVL get: {}", stop_watch());
        for i in 0..SIZE {
            set.erase(&i);
        }
        println!("AVL erase: {}", stop_watch());

        let mut nums = (0..SIZE).collect::<Vec<_>>();
        let mut rng = thread_rng();
        nums.shuffle(&mut rng);
        stop_watch();
        let mut set = AVL::<usize>::new();
        for i in 0..SIZE {
            set.insert(nums[i]);
        }
        println!("AVL shuffle insert: {}", stop_watch());
        println!("AVL shuffle height: {}", set.height());
        for i in 0..SIZE {
            assert_eq!(set.get(i).unwrap(), &i);
        }
        println!("AVL shuffle get: {}", stop_watch());
        for i in 0..SIZE {
            set.erase(&nums[i]);
        }
        println!("AVL shuffle erase: {}", stop_watch());
    }

    #[test]
    fn test_hack() {
        const SIZE: usize = 100000;
        let mut set = AVL::<usize>::new();
        for i in 0..SIZE {
            set.insert(i ^ 0xFFF);
        }
        println!("AVL height: {}", set.height());
    }
}
