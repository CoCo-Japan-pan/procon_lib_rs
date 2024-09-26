//! [AVL木](https://qiita.com/QCFium/items/3cf26a6dc2d49ef490d7)  
//! `std::collections::BTreeSet` と異なり、k番目の値を`O(logN)`で取り出せる  
//! 親を持っていないので、iterやrangeはないです...  
//! ランダムだとBTreeSetより7倍ぐらい遅いので、本当に必要なときだけ使うのがよさそう  
//! 列を管理する

use std::cmp::Ordering;
use std::fmt::Display;
use std::iter::successors;
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
        if height(&self.left).abs_diff(height(&self.right)) <= 1 {
            self.update();
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

    fn list_sub(self, ret: &mut Vec<T>) {
        if let Some(left) = self.left {
            left.list_sub(ret);
        }
        ret.push(self.value);
        if let Some(right) = self.right {
            right.list_sub(ret);
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

fn merge<T>(left: Tree<T>, right: Tree<T>) -> Tree<T> {
    match (left.is_some(), right.is_some()) {
        (true, true) => {
            let (_, center, rhs) = split_delete(right.unwrap(), 0);
            Some(merge_with_root(left, center, rhs))
        }
        (false, _) => right,
        (_, false) => left,
    }
}

fn merge_with_root<T>(
    mut left: Tree<T>,
    mut center: Box<Node<T>>,
    mut right: Tree<T>,
) -> Box<Node<T>> {
    if height(&left).abs_diff(height(&right)) <= 1 {
        center.left = left;
        center.right = right;
        center.update();
        center
    } else if height(&left) < height(&right) {
        let mut root = right.take().unwrap();
        root.left = Some(merge_with_root(left, center, root.left.take()));
        root.balance();
        root
    } else {
        let mut root = left.take().unwrap();
        root.right = Some(merge_with_root(root.right.take(), center, right));
        root.balance();
        root
    }
}

fn split_delete<T>(mut root: Box<Node<T>>, index: usize) -> (Tree<T>, Box<Node<T>>, Tree<T>) {
    debug_assert!((0..root.len).contains(&index));
    let left = root.left.take();
    let right = root.right.take();
    let lsize = len(&left);
    match lsize.cmp(&index) {
        Ordering::Equal => (left, root, right),
        Ordering::Less => {
            let mut ret = split_delete(right.unwrap(), index - lsize - 1);
            ret.0 = Some(merge_with_root(left, root, ret.0));
            ret
        }
        Ordering::Greater => {
            let mut ret = split_delete(left.unwrap(), index);
            ret.2 = Some(merge_with_root(ret.2, root, right));
            ret
        }
    }
}

/// split into [0, index), [index, n)
fn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>, Tree<T>) {
    let Some(root) = tree else {
        return (None, None);
    };
    if index == 0 {
        (None, Some(root))
    } else if root.len == index {
        (Some(root), None)
    } else {
        let (left, center, right) = split_delete(root, index);
        (left, Some(merge_with_root(None, center, right)))
    }
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
    multi: bool, // 重複を許すか
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
    /// 重複を許すならtrue
    pub fn new(multi: bool) -> Self {
        Self { root: None, multi }
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

    /// otherの中身を空にしながら、自分の右に追加する
    pub fn append(&mut self, other: &mut Self) {
        self.root = merge(self.root.take(), other.root.take());
    }

    /// [0, index)を残し、[index, n)を返す
    pub fn split_off(&mut self, index: usize) -> Self {
        assert!(index <= self.len());
        let (left, right) = split(self.root.take(), index);
        self.root = left;
        Self {
            root: right,
            multi: self.multi,
        }
    }

    pub fn insert_by_index(&mut self, index: usize, value: T) {
        assert!(index <= self.len());
        let other = self.split_off(index);
        self.root = Some(merge_with_root(
            self.root.take(),
            Box::new(Node::new(value)),
            other.root,
        ))
    }

    /// 適切な順序を二分探索して挿入
    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        if !self.multi && self.count(&value) > 0 {
            return;
        }
        let index = self.lower_bound(&value);
        self.insert_by_index(index, value);
    }

    pub fn erase_by_index(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            let (left, center, right) = split_delete(self.root.take().unwrap(), index);
            self.root = merge(left, right);
            Some(center.value)
        } else {
            None
        }
    }

    /// 二分探索で値を見つけて一つ削除
    pub fn erase(&mut self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        if self.count(value) == 0 {
            return false;
        }
        let index = self.lower_bound(value);
        let ret = self.erase_by_index(index);
        ret.is_some()
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        self.count(value) > 0
    }

    pub fn count(&self, value: &T) -> usize
    where
        T: PartialOrd,
    {
        count(&self.root, value)
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::with_capacity(self.len());
        if let Some(root) = self.root {
            root.list_sub(&mut ret);
        }
        ret
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            stack: successors(self.root.as_deref(), |x| x.left.as_deref()).collect(),
        }
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.stack
            .extend(successors(node.right.as_deref(), |x| x.left.as_deref()));
        Some(&node.value)
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
        let mut avl = AVL::<i32>::new(true);
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
        let mut rbst = AVL::<i32>::new(false);
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
                rbst.insert(value);
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
        let mut set = AVL::<usize>::new(true);
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
        let mut set = BTreeSet::new();
        for i in 0..SIZE {
            set.insert(nums[i]);
        }
        println!("BTreeSet shuffle insert: {}", stop_watch());
        for i in 0..SIZE {
            assert!(set.remove(&i));
        }
        println!("BTreeSet shuffle erase: {}", stop_watch());
        let mut set = AVL::<usize>::new(true);
        for i in 0..SIZE {
            set.insert(nums[i]);
        }
        println!("AVL shuffle insert: {}", stop_watch());
        println!("AVL shuffle height: {}", set.height());
        for i in 0..SIZE {
            assert_eq!(set.get(i).unwrap(), &i);
        }
        println!("AVL shuffle get: {}", stop_watch());
        stop_watch();
        for i in 0..SIZE {
            set.erase(&i);
        }
        println!("AVL shuffle erase: {}", stop_watch());
    }

    #[test]
    fn test_hack() {
        const SIZE: usize = 250000;
        stop_watch();
        let mut set = AVL::<usize>::new(true);
        for i in (0..SIZE).rev() {
            set.insert(i);
        }
        println!("insert rev: {}", stop_watch());
        println!("height: {}", set.height());
        stop_watch();
        let mut set = AVL::<usize>::new(true);
        for i in 0..SIZE {
            set.insert(i ^ 0xFFF);
        }
        println!("insert xor: {}", stop_watch());
        println!("height: {}", set.height());
        stop_watch();
        let mut set = AVL::<usize>::new(true);
        for i in 0..SIZE {
            if i % 2 == 0 {
                set.insert(i);
            } else {
                set.insert(usize::MAX - i);
            }
        }
        println!("insert from edges: {}", stop_watch());
        println!("height: {}", set.height());
    }

    #[test]
    fn test_iter() {
        let mut set = AVL::<usize>::new(true);
        const SIZE: usize = 100000;
        for i in 0..SIZE {
            set.insert(i);
        }
        let mut iter = set.iter();
        for i in 0..SIZE {
            assert_eq!(iter.next(), Some(&i));
        }
        assert_eq!(iter.next(), None);

        let mut set = AVL::<usize>::new(true);
        let mut rng = thread_rng();
        let mut nums = (0..SIZE).collect::<Vec<_>>();
        nums.shuffle(&mut rng);
        for i in 0..SIZE {
            set.insert(nums[i]);
        }
        let mut iter = set.iter();
        for i in 0..SIZE {
            assert_eq!(iter.next(), Some(&i));
        }
        assert_eq!(iter.next(), None);
    }
}
