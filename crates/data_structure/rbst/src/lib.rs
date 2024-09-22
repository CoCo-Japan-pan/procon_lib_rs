//! [平衡二分探索木](https://www.slideshare.net/slideshow/2-12188757/12188757)
//! merge-split ベースのRBST  
//! nノードの木とmノードの木をmergeするとき、確率 n/(n+m) で左の木の根を新たな根とする  
//! `std::collections::BTreeSet` と異なり、k番目の値を`O(logN)`で取り出せる  

use pcg32::Pcg32;
use std::cmp::Ordering;
use std::fmt::Display;

fn get_rand_usize() -> usize {
    static mut RNG: Option<Pcg32> = None;
    unsafe {
        if RNG.is_none() {
            RNG = Some(Pcg32::init_rand());
        }
        RNG.unwrap().next_u32() as usize
    }
}
type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    left: Tree<T>,
    right: Tree<T>,
    value: T,
    len: usize,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Self {
            left: None,
            right: None,
            value,
            len: 1,
        }
    }
    fn update(&mut self) {
        self.len = len(&self.left) + len(&self.right) + 1;
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

fn merge<T>(left: Tree<T>, right: Tree<T>) -> Tree<T> {
    match (left, right) {
        (Some(mut left), Some(mut right)) => {
            let mut new_tree = if get_rand_usize() % (left.len + right.len) < left.len {
                left.right = merge(left.right, Some(right));
                left
            } else {
                right.left = merge(Some(left), right.left);
                right
            };
            new_tree.update();
            Some(new_tree)
        }
        (left, None) => left,
        (None, right) => right,
    }
}

/// split into [0, index), [index, n)
fn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>, Tree<T>) {
    let Some(mut tree) = tree else {
        return (None, None);
    };
    let left_len = len(&tree.left);
    if index <= left_len {
        let sub = split(tree.left, index);
        tree.left = sub.1;
        tree.update();
        (sub.0, Some(tree))
    } else {
        let sub = split(tree.right, index - left_len - 1);
        tree.right = sub.0;
        tree.update();
        (Some(tree), sub.1)
    }
}

fn insert_by_idx<T>(tree: Tree<T>, index: usize, value: T) -> Tree<T> {
    assert!(index <= len(&tree));
    let new_node = Some(Box::new(Node::new(value)));
    if tree.is_none() {
        return new_node;
    };
    let (left, right) = split(tree, index);
    let left = merge(left, new_node);
    merge(left, right)
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
        len(&tree.left) + lower_bound(&tree.right, value) + 1
    }
}

/// valueより大きい最初の値のindex
fn upper_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {
    let Some(tree) = tree else {
        return 0;
    };
    if value >= &tree.value {
        len(&tree.left) + upper_bound(&tree.right, value) + 1
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
pub struct RBST<T> {
    root: Tree<T>,
}

impl<T: Display> Display for RBST<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root)
        } else {
            write!(f, "Empty")
        }
    }
}

impl<T> RBST<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        len(&self.root)
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
        let mut dummy = None;
        std::mem::swap(&mut dummy, &mut self.root);
        dummy = insert(dummy, value);
        self.root = dummy;
    }

    pub fn erase(&mut self, value: &T)
    where
        T: PartialOrd,
    {
        let mut dummy = None;
        std::mem::swap(&mut dummy, &mut self.root);
        dummy = erase(dummy, value);
        self.root = dummy;
    }

    pub fn count(&self, value: &T) -> usize
    where
        T: PartialOrd,
    {
        count(&self.root, value)
    }
}

impl<T> Default for RBST<T> {
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
        let mut rbst = RBST::<i32>::new();
        let mut set = BTreeMap::new();
        for _ in 0..1000 {
            let value = rng.gen_range(-100..=100);
            rbst.insert(value);
            *set.entry(value).or_insert(0) += 1;
        }
        for cnt in 0..1000 {
            if cnt % 2 == 0 {
                let idx = rng.gen_range(0..set.len());
                let value = set.iter().nth(idx).unwrap();
                assert_eq!(rbst.count(value.0), *value.1);
            } else if set.is_empty() || rng.gen() {
                let value = rng.gen_range(-100..=100);
                rbst.insert(value);
                *set.entry(value).or_insert(0) += 1;
            } else {
                let value = rng.gen_range(-100..=100);
                rbst.erase(&value);
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
        let mut rbst = RBST::<i32>::new();
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
        stop_watch();
        let mut set = BTreeSet::new();
        for i in 0..250000 {
            set.insert(i);
        }
        println!("BTreeSet insert: {}", stop_watch());
        for i in 0..250000 {
            set.remove(&i);
        }
        println!("BTreeSet erase: {}", stop_watch());
        let mut set = RBST::<i32>::new();
        // currently stack overflow...
        for i in 0..250000 {
            set.insert(i);
        }
        println!("RBST insert: {}", stop_watch());
        for i in 0..250000 {
            set.erase(&i);
        }
        println!("RBST erase: {}", stop_watch());
    }
}
