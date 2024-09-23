---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://www.slideshare.net/slideshow/2-12188757/12188757)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! AVL\u6728\u3092\u5B9F\u88C5\u3057\u3088\u3046\u3068\u3057\u3066\u3001\
    \u306A\u3093\u304B\u9AD8\u3055\u304C\u304A\u304B\u3057\u3044\u306E\u3067\u30D0\
    \u30B0\u3063\u3066\u307E\u3059...\n//! [\u5E73\u8861\u4E8C\u5206\u63A2\u7D22\u6728\
    ](https://www.slideshare.net/slideshow/2-12188757/12188757)  \n//! `std::collections::BTreeSet`\
    \ \u3068\u7570\u306A\u308A\u3001k\u756A\u76EE\u306E\u5024\u3092`O(logN)`\u3067\
    \u53D6\u308A\u51FA\u305B\u308B  \n\nuse std::cmp::Ordering;\nuse std::fmt::Display;\n\
    type Tree<T> = Option<Box<Node<T>>>;\n\n#[derive(Debug)]\nstruct Node<T> {\n \
    \   left: Tree<T>,\n    right: Tree<T>,\n    value: T,\n    len: usize,\n    height:\
    \ u8,\n}\n\nimpl<T> Node<T> {\n    fn new(value: T) -> Node<T> {\n        Self\
    \ {\n            left: None,\n            right: None,\n            value,\n \
    \           len: 1,\n            height: 1,\n        }\n    }\n    fn update(&mut\
    \ self) {\n        self.len = len(&self.left) + len(&self.right) + 1;\n      \
    \  self.height = height(&self.left).max(height(&self.right)) + 1;\n    }\n}\n\n\
    fn balance<T>(node: &mut Box<Node<T>>) {\n    fn rotate_left<T>(node: &mut Box<Node<T>>)\
    \ {\n        let mut x = node.left.take().unwrap();\n        let y = x.right.take();\n\
    \        std::mem::swap(node, &mut x);\n        x.left = y;\n        x.update();\n\
    \        node.right = Some(x);\n        node.update();\n    }\n    fn rotate_right<T>(node:\
    \ &mut Box<Node<T>>) {\n        let mut x = node.right.take().unwrap();\n    \
    \    let y = x.left.take();\n        std::mem::swap(node, &mut x);\n        x.right\
    \ = y;\n        x.update();\n        node.left = Some(x);\n        node.update();\n\
    \    }\n    if height(&node.left) > 1 + height(&node.right) {\n        let left\
    \ = node.left.as_mut().unwrap();\n        if height(&left.left) < height(&left.right)\
    \ {\n            rotate_right(left);\n        }\n        rotate_left(node);\n\
    \    } else if height(&node.left) + 1 < height(&node.right) {\n        let right\
    \ = node.right.as_mut().unwrap();\n        if height(&right.left) > height(&right.right)\
    \ {\n            rotate_left(right);\n        }\n        rotate_right(node);\n\
    \    } else {\n        node.update();\n    }\n}\n\nimpl<T: Display> Display for\
    \ Node<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        if let Some(left) = &self.left {\n            write!(f, \"{}\", left)?;\n\
    \        }\n        write!(f, \"{}, \", self.value)?;\n        if let Some(right)\
    \ = &self.right {\n            write!(f, \"{}\", right)?;\n        }\n       \
    \ Ok(())\n    }\n}\n\nfn len<T>(tree: &Tree<T>) -> usize {\n    tree.as_ref().map_or(0,\
    \ |t| t.len)\n}\n\nfn height<T>(tree: &Tree<T>) -> u8 {\n    tree.as_ref().map_or(0,\
    \ |t| t.height)\n}\n\nfn merge<T>(left: Tree<T>, right: Tree<T>) -> Tree<T> {\n\
    \    match (left, right) {\n        (Some(mut left), Some(mut right)) => {\n \
    \           let mut new_tree = if left.height > right.height {\n             \
    \   left.right = merge(left.right, Some(right));\n                left\n     \
    \       } else {\n                right.left = merge(Some(left), right.left);\n\
    \                right\n            };\n            balance(&mut new_tree);\n\
    \            Some(new_tree)\n        }\n        (left, None) => left,\n      \
    \  (None, right) => right,\n    }\n}\n\n/// split into [0, index), [index, n)\n\
    fn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>, Tree<T>) {\n    let Some(mut\
    \ tree) = tree else {\n        return (None, None);\n    };\n    let left_len\
    \ = len(&tree.left);\n    if index <= left_len {\n        let sub = split(tree.left,\
    \ index);\n        tree.left = sub.1;\n        tree.update();\n        (sub.0,\
    \ Some(tree))\n    } else {\n        let sub = split(tree.right, index - left_len\
    \ - 1);\n        tree.right = sub.0;\n        tree.update();\n        (Some(tree),\
    \ sub.1)\n    }\n}\n\nfn insert_by_idx<T>(tree: Tree<T>, index: usize, value:\
    \ T) -> Tree<T> {\n    assert!(index <= len(&tree));\n    let new_node = Some(Box::new(Node::new(value)));\n\
    \    if tree.is_none() {\n        return new_node;\n    };\n    let (left, right)\
    \ = split(tree, index);\n    let left = merge(left, new_node);\n    merge(left,\
    \ right)\n}\n\nfn insert<T: PartialOrd>(tree: Tree<T>, value: T) -> Tree<T> {\n\
    \    let index = lower_bound(&tree, &value);\n    insert_by_idx(tree, index, value)\n\
    }\n\nfn erase_by_idx<T>(tree: Tree<T>, index: usize) -> Tree<T> {\n    assert!(index\
    \ < len(&tree));\n    let (left, right) = split(tree, index);\n    let (_, right)\
    \ = split(right, 1);\n    merge(left, right)\n}\n\nfn erase<T: PartialOrd>(tree:\
    \ Tree<T>, value: &T) -> Tree<T> {\n    if count(&tree, value) == 0 {\n      \
    \  return tree;\n    }\n    let index = lower_bound(&tree, value);\n    erase_by_idx(tree,\
    \ index)\n}\n\n/// value\u4EE5\u4E0A\u306E\u6700\u521D\u306E\u5024\u306Eindex\n\
    fn lower_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    let Some(tree)\
    \ = tree else {\n        return 0;\n    };\n    if value <= &tree.value {\n  \
    \      lower_bound(&tree.left, value)\n    } else {\n        len(&tree.left) +\
    \ lower_bound(&tree.right, value) + 1\n    }\n}\n\n/// value\u3088\u308A\u5927\
    \u304D\u3044\u6700\u521D\u306E\u5024\u306Eindex\nfn upper_bound<T: PartialOrd>(tree:\
    \ &Tree<T>, value: &T) -> usize {\n    let Some(tree) = tree else {\n        return\
    \ 0;\n    };\n    if value >= &tree.value {\n        len(&tree.left) + upper_bound(&tree.right,\
    \ value) + 1\n    } else {\n        upper_bound(&tree.left, value)\n    }\n}\n\
    \nfn count<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    upper_bound(tree,\
    \ value) - lower_bound(tree, value)\n}\n\nfn get<T>(tree: &Tree<T>, index: usize)\
    \ -> Option<&T> {\n    if len(tree) <= index {\n        return None;\n    }\n\
    \    let Some(tree) = tree else {\n        return None;\n    };\n    let left_len\
    \ = len(&tree.left);\n    match index.cmp(&left_len) {\n        Ordering::Less\
    \ => get(&tree.left, index),\n        Ordering::Equal => Some(&tree.value),\n\
    \        Ordering::Greater => get(&tree.right, index - left_len - 1),\n    }\n\
    }\n\n#[derive(Debug)]\npub struct AVL<T> {\n    root: Tree<T>,\n}\n\nimpl<T: Display>\
    \ Display for AVL<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->\
    \ std::fmt::Result {\n        if let Some(root) = &self.root {\n            write!(f,\
    \ \"{}\", root)\n        } else {\n            write!(f, \"Empty\")\n        }\n\
    \    }\n}\n\nimpl<T> AVL<T> {\n    pub fn new() -> Self {\n        Self { root:\
    \ None }\n    }\n\n    pub fn len(&self) -> usize {\n        len(&self.root)\n\
    \    }\n\n    pub fn height(&self) -> u8 {\n        height(&self.root)\n    }\n\
    \n    pub fn is_empty(&self) -> bool {\n        self.root.is_none()\n    }\n\n\
    \    pub fn lower_bound(&self, value: &T) -> usize\n    where\n        T: PartialOrd,\n\
    \    {\n        lower_bound(&self.root, value)\n    }\n\n    pub fn upper_bound(&self,\
    \ value: &T) -> usize\n    where\n        T: PartialOrd,\n    {\n        upper_bound(&self.root,\
    \ value)\n    }\n\n    /// index\u756A\u76EE(0-base)\u306E\u5024\u3092\u53D6\u5F97\
    \n    pub fn get(&self, index: usize) -> Option<&T> {\n        get(&self.root,\
    \ index)\n    }\n\n    pub fn insert(&mut self, value: T)\n    where\n       \
    \ T: PartialOrd,\n    {\n        let mut dummy = None;\n        std::mem::swap(&mut\
    \ dummy, &mut self.root);\n        dummy = insert(dummy, value);\n        self.root\
    \ = dummy;\n    }\n\n    pub fn erase(&mut self, value: &T)\n    where\n     \
    \   T: PartialOrd,\n    {\n        let mut dummy = None;\n        std::mem::swap(&mut\
    \ dummy, &mut self.root);\n        dummy = erase(dummy, value);\n        self.root\
    \ = dummy;\n    }\n\n    pub fn count(&self, value: &T) -> usize\n    where\n\
    \        T: PartialOrd,\n    {\n        count(&self.root, value)\n    }\n}\n\n\
    impl<T> Default for AVL<T> {\n    fn default() -> Self {\n        Self::new()\n\
    \    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \    use std::collections::{BTreeMap, BTreeSet};\n\n    fn stop_watch() -> f64\
    \ {\n        use std::time::{SystemTime, UNIX_EPOCH};\n        static mut START:\
    \ f64 = 0.0;\n        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n\
    \        let current = time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9;\n\
    \        unsafe {\n            let ret = current - START;\n            START =\
    \ current;\n            ret\n        }\n    }\n\n    #[test]\n    fn test_cnt()\
    \ {\n        let mut rng = thread_rng();\n        let mut rbst = AVL::<i32>::new();\n\
    \        let mut set = BTreeMap::new();\n        const SIZE: usize = 100000;\n\
    \        for _ in 0..SIZE {\n            let value = rng.gen_range(-100..=100);\n\
    \            rbst.insert(value);\n            *set.entry(value).or_insert(0) +=\
    \ 1;\n        }\n        for cnt in 0..SIZE {\n            if cnt % 2 == 0 {\n\
    \                let value = rng.gen_range(-100..=100);\n                let tree_cnt\
    \ = set.get(&value).copied().unwrap_or(0);\n                let rbst_cnt = rbst.count(&value);\n\
    \                assert_eq!(tree_cnt, rbst_cnt);\n            } else if set.is_empty()\
    \ || rng.gen() {\n                let value = rng.gen_range(-100..=100);\n   \
    \             rbst.insert(value);\n                *set.entry(value).or_insert(0)\
    \ += 1;\n            } else {\n                let value = rng.gen_range(-100..=100);\n\
    \                rbst.erase(&value);\n                set.entry(value).and_modify(|x|\
    \ *x -= 1);\n                if let Some(x) = set.get(&value) {\n            \
    \        if *x == 0 {\n                        set.remove(&value);\n         \
    \           }\n                }\n            }\n        }\n    }\n\n    #[test]\n\
    \    fn test_kth_by_no_dups() {\n        let mut rng = thread_rng();\n       \
    \ let mut rbst = AVL::<i32>::new();\n        let mut set = BTreeSet::new();\n\
    \        for _ in 0..1000 {\n            let value = rng.gen_range(-100..=100);\n\
    \            if rbst.count(&value) == 0 {\n                rbst.insert(value);\n\
    \            }\n            set.insert(value);\n        }\n        for cnt in\
    \ 0..1000 {\n            if cnt % 2 == 0 {\n                let idx = rng.gen_range(0..set.len());\n\
    \                let value = set.iter().nth(idx).unwrap();\n                assert_eq!(rbst.get(idx).unwrap(),\
    \ value);\n            } else if set.is_empty() || rng.gen() {\n             \
    \   let value = rng.gen_range(-100..=100);\n                if rbst.count(&value)\
    \ == 0 {\n                    rbst.insert(value);\n                }\n       \
    \         set.insert(value);\n            } else {\n                let value\
    \ = rng.gen_range(-100..=100);\n                rbst.erase(&value);\n        \
    \        set.remove(&value);\n            }\n        }\n    }\n\n    #[test]\n\
    \    fn test_bench() {\n        const SIZE: usize = 250000;\n        stop_watch();\n\
    \        let mut set = BTreeSet::new();\n        for i in 0..SIZE {\n        \
    \    set.insert(i);\n        }\n        println!(\"BTreeSet insert: {}\", stop_watch());\n\
    \        for i in 0..SIZE {\n            set.remove(&i);\n        }\n        println!(\"\
    BTreeSet erase: {}\", stop_watch());\n        let mut set = AVL::<usize>::new();\n\
    \        for i in 0..SIZE {\n            set.insert(i);\n        }\n        println!(\"\
    AVL insert: {}\", stop_watch());\n        println!(\"AVL height: {}\", set.height());\n\
    \        for i in 0..SIZE {\n            assert_eq!(set.get(i).unwrap(), &i);\n\
    \        }\n        println!(\"AVL get: {}\", stop_watch());\n        for i in\
    \ 0..SIZE {\n            set.erase(&i);\n        }\n        println!(\"AVL erase:\
    \ {}\", stop_watch());\n\n        let mut nums = (0..SIZE).collect::<Vec<_>>();\n\
    \        let mut rng = thread_rng();\n        nums.shuffle(&mut rng);\n      \
    \  stop_watch();\n        let mut set = AVL::<usize>::new();\n        for i in\
    \ 0..SIZE {\n            set.insert(nums[i]);\n        }\n        println!(\"\
    AVL shuffle insert: {}\", stop_watch());\n        println!(\"AVL shuffle height:\
    \ {}\", set.height());\n        for i in 0..SIZE {\n            assert_eq!(set.get(i).unwrap(),\
    \ &i);\n        }\n        println!(\"AVL shuffle get: {}\", stop_watch());\n\
    \        for i in 0..SIZE {\n            set.erase(&nums[i]);\n        }\n   \
    \     println!(\"AVL shuffle erase: {}\", stop_watch());\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/avl/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-23 12:26:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/avl/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/avl/src/lib.rs
- /library/crates/data_structure/avl/src/lib.rs.html
title: crates/data_structure/avl/src/lib.rs
---
