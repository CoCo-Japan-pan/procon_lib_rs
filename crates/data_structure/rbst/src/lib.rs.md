---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/rand/pcg32/src/lib.rs
    title: crates/rand/pcg32/src/lib.rs
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
  code: "//! [\u5E73\u8861\u4E8C\u5206\u63A2\u7D22\u6728](https://www.slideshare.net/slideshow/2-12188757/12188757)\n\
    //! merge-split \u30D9\u30FC\u30B9\u306ERBST  \n//! n\u30CE\u30FC\u30C9\u306E\u6728\
    \u3068m\u30CE\u30FC\u30C9\u306E\u6728\u3092merge\u3059\u308B\u3068\u304D\u3001\
    \u78BA\u7387 n/(n+m) \u3067\u5DE6\u306E\u6728\u306E\u6839\u3092\u65B0\u305F\u306A\
    \u6839\u3068\u3059\u308B  \n//! `std::collections::BTreeSet` \u3068\u7570\u306A\
    \u308A\u3001k\u756A\u76EE\u306E\u5024\u3092`O(logN)`\u3067\u53D6\u308A\u51FA\u305B\
    \u308B  \n\nuse pcg32::Pcg32;\nuse std::cmp::Ordering;\nuse std::fmt::Display;\n\
    \nfn get_rand_usize() -> usize {\n    static mut RNG: Option<Pcg32> = None;\n\
    \    unsafe {\n        if RNG.is_none() {\n            RNG = Some(Pcg32::init_rand());\n\
    \        }\n        RNG.unwrap().next_u32() as usize\n    }\n}\ntype Tree<T> =\
    \ Option<Box<Node<T>>>;\n\n#[derive(Debug)]\nstruct Node<T> {\n    left: Tree<T>,\n\
    \    right: Tree<T>,\n    value: T,\n    len: usize,\n}\n\nimpl<T> Node<T> {\n\
    \    fn new(value: T) -> Node<T> {\n        Self {\n            left: None,\n\
    \            right: None,\n            value,\n            len: 1,\n        }\n\
    \    }\n    fn update(&mut self) {\n        self.len = len(&self.left) + len(&self.right)\
    \ + 1;\n    }\n}\n\nimpl<T: Display> Display for Node<T> {\n    fn fmt(&self,\
    \ f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        if let Some(left)\
    \ = &self.left {\n            write!(f, \"{}\", left)?;\n        }\n        write!(f,\
    \ \"{}, \", self.value)?;\n        if let Some(right) = &self.right {\n      \
    \      write!(f, \"{}\", right)?;\n        }\n        Ok(())\n    }\n}\n\nfn len<T>(tree:\
    \ &Tree<T>) -> usize {\n    tree.as_ref().map_or(0, |t| t.len)\n}\n\nfn merge<T>(left:\
    \ Tree<T>, right: Tree<T>) -> Tree<T> {\n    match (left, right) {\n        (Some(mut\
    \ left), Some(mut right)) => {\n            let mut new_tree = if get_rand_usize()\
    \ % (left.len + right.len) < left.len {\n                left.right = merge(left.right,\
    \ Some(right));\n                left\n            } else {\n                right.left\
    \ = merge(Some(left), right.left);\n                right\n            };\n  \
    \          new_tree.update();\n            Some(new_tree)\n        }\n       \
    \ (left, None) => left,\n        (None, right) => right,\n    }\n}\n\n/// split\
    \ into [0, index), [index, n)\nfn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>,\
    \ Tree<T>) {\n    let Some(mut tree) = tree else {\n        return (None, None);\n\
    \    };\n    let left_len = len(&tree.left);\n    if index <= left_len {\n   \
    \     let sub = split(tree.left, index);\n        tree.left = sub.1;\n       \
    \ tree.update();\n        (sub.0, Some(tree))\n    } else {\n        let sub =\
    \ split(tree.right, index - left_len - 1);\n        tree.right = sub.0;\n    \
    \    tree.update();\n        (Some(tree), sub.1)\n    }\n}\n\nfn insert_by_idx<T>(tree:\
    \ Tree<T>, index: usize, value: T) -> Tree<T> {\n    assert!(index <= len(&tree));\n\
    \    let new_node = Some(Box::new(Node::new(value)));\n    if tree.is_none() {\n\
    \        return new_node;\n    };\n    let (left, right) = split(tree, index);\n\
    \    let left = merge(left, new_node);\n    merge(left, right)\n}\n\nfn insert<T:\
    \ PartialOrd>(tree: Tree<T>, value: T) -> Tree<T> {\n    let index = lower_bound(&tree,\
    \ &value);\n    insert_by_idx(tree, index, value)\n}\n\nfn erase_by_idx<T>(tree:\
    \ Tree<T>, index: usize) -> Tree<T> {\n    assert!(index < len(&tree));\n    let\
    \ (left, right) = split(tree, index);\n    let (_, right) = split(right, 1);\n\
    \    merge(left, right)\n}\n\nfn erase<T: PartialOrd>(tree: Tree<T>, value: &T)\
    \ -> Tree<T> {\n    if count(&tree, value) == 0 {\n        return tree;\n    }\n\
    \    let index = lower_bound(&tree, value);\n    erase_by_idx(tree, index)\n}\n\
    \n/// value\u4EE5\u4E0A\u306E\u6700\u521D\u306E\u5024\u306Eindex\nfn lower_bound<T:\
    \ PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    let Some(tree) = tree\
    \ else {\n        return 0;\n    };\n    if value <= &tree.value {\n        lower_bound(&tree.left,\
    \ value)\n    } else {\n        len(&tree.left) + lower_bound(&tree.right, value)\
    \ + 1\n    }\n}\n\n/// value\u3088\u308A\u5927\u304D\u3044\u6700\u521D\u306E\u5024\
    \u306Eindex\nfn upper_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize\
    \ {\n    let Some(tree) = tree else {\n        return 0;\n    };\n    if value\
    \ >= &tree.value {\n        len(&tree.left) + upper_bound(&tree.right, value)\
    \ + 1\n    } else {\n        upper_bound(&tree.left, value)\n    }\n}\n\nfn count<T:\
    \ PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    upper_bound(tree, value)\
    \ - lower_bound(tree, value)\n}\n\nfn get<T>(tree: &Tree<T>, index: usize) ->\
    \ Option<&T> {\n    if len(tree) <= index {\n        return None;\n    }\n   \
    \ let Some(tree) = tree else {\n        return None;\n    };\n    let left_len\
    \ = len(&tree.left);\n    match index.cmp(&left_len) {\n        Ordering::Less\
    \ => get(&tree.left, index),\n        Ordering::Equal => Some(&tree.value),\n\
    \        Ordering::Greater => get(&tree.right, index - left_len - 1),\n    }\n\
    }\n\n#[derive(Debug)]\npub struct RBST<T> {\n    root: Tree<T>,\n}\n\nimpl<T:\
    \ Display> Display for RBST<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        if let Some(root) = &self.root {\n          \
    \  write!(f, \"{}\", root)\n        } else {\n            write!(f, \"Empty\"\
    )\n        }\n    }\n}\n\nimpl<T> RBST<T> {\n    pub fn new() -> Self {\n    \
    \    Self { root: None }\n    }\n\n    pub fn len(&self) -> usize {\n        len(&self.root)\n\
    \    }\n\n    pub fn is_empty(&self) -> bool {\n        self.root.is_none()\n\
    \    }\n\n    pub fn lower_bound(&self, value: &T) -> usize\n    where\n     \
    \   T: PartialOrd,\n    {\n        lower_bound(&self.root, value)\n    }\n\n \
    \   pub fn upper_bound(&self, value: &T) -> usize\n    where\n        T: PartialOrd,\n\
    \    {\n        upper_bound(&self.root, value)\n    }\n\n    /// index\u756A\u76EE\
    (0-base)\u306E\u5024\u3092\u53D6\u5F97\n    pub fn get(&self, index: usize) ->\
    \ Option<&T> {\n        get(&self.root, index)\n    }\n\n    pub fn insert(&mut\
    \ self, value: T)\n    where\n        T: PartialOrd,\n    {\n        let mut dummy\
    \ = None;\n        std::mem::swap(&mut dummy, &mut self.root);\n        dummy\
    \ = insert(dummy, value);\n        self.root = dummy;\n    }\n\n    pub fn erase(&mut\
    \ self, value: &T)\n    where\n        T: PartialOrd,\n    {\n        let mut\
    \ dummy = None;\n        std::mem::swap(&mut dummy, &mut self.root);\n       \
    \ dummy = erase(dummy, value);\n        self.root = dummy;\n    }\n\n    pub fn\
    \ count(&self, value: &T) -> usize\n    where\n        T: PartialOrd,\n    {\n\
    \        count(&self.root, value)\n    }\n}\n\nimpl<T> Default for RBST<T> {\n\
    \    fn default() -> Self {\n        Self::new()\n    }\n}\n\n#[cfg(test)]\nmod\
    \ test {\n    use super::*;\n    use rand::prelude::*;\n    use std::collections::{BTreeMap,\
    \ BTreeSet};\n\n    #[test]\n    fn test_cnt() {\n        let mut rng = thread_rng();\n\
    \        let mut rbst = RBST::<i32>::new();\n        let mut set = BTreeMap::new();\n\
    \        for _ in 0..1000 {\n            let value = rng.gen_range(-100..=100);\n\
    \            rbst.insert(value);\n            *set.entry(value).or_insert(0) +=\
    \ 1;\n        }\n        for cnt in 0..1000 {\n            if cnt % 2 == 0 {\n\
    \                let idx = rng.gen_range(0..set.len());\n                let value\
    \ = set.iter().nth(idx).unwrap();\n                assert_eq!(rbst.count(value.0),\
    \ *value.1);\n            } else if set.is_empty() || rng.gen() {\n          \
    \      let value = rng.gen_range(-100..=100);\n                rbst.insert(value);\n\
    \                *set.entry(value).or_insert(0) += 1;\n            } else {\n\
    \                let value = rng.gen_range(-100..=100);\n                rbst.erase(&value);\n\
    \                set.entry(value).and_modify(|x| *x -= 1);\n                if\
    \ let Some(x) = set.get(&value) {\n                    if *x == 0 {\n        \
    \                set.remove(&value);\n                    }\n                }\n\
    \            }\n        }\n    }\n\n    #[test]\n    fn test_kth_by_no_dups()\
    \ {\n        let mut rng = thread_rng();\n        let mut rbst = RBST::<i32>::new();\n\
    \        let mut set = BTreeSet::new();\n        for _ in 0..1000 {\n        \
    \    let value = rng.gen_range(-100..=100);\n            if rbst.count(&value)\
    \ == 0 {\n                rbst.insert(value);\n            }\n            set.insert(value);\n\
    \        }\n        for cnt in 0..1000 {\n            if cnt % 2 == 0 {\n    \
    \            let idx = rng.gen_range(0..set.len());\n                let value\
    \ = set.iter().nth(idx).unwrap();\n                assert_eq!(rbst.get(idx).unwrap(),\
    \ value);\n            } else if set.is_empty() || rng.gen() {\n             \
    \   let value = rng.gen_range(-100..=100);\n                if rbst.count(&value)\
    \ == 0 {\n                    rbst.insert(value);\n                }\n       \
    \         set.insert(value);\n            } else {\n                let value\
    \ = rng.gen_range(-100..=100);\n                rbst.erase(&value);\n        \
    \        set.remove(&value);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/rand/pcg32/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/rbst/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-22 19:22:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/rbst/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/rbst/src/lib.rs
- /library/crates/data_structure/rbst/src/lib.rs.html
title: crates/data_structure/rbst/src/lib.rs
---
