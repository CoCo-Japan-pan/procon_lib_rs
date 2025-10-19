---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_649_avl/src/main.rs
    title: verify/yukicoder/no_649_avl/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://qiita.com/QCFium/items/3cf26a6dc2d49ef490d7
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [AVL\u6728](https://qiita.com/QCFium/items/3cf26a6dc2d49ef490d7)  \n\
    //! `std::collections::BTreeSet` \u3068\u7570\u306A\u308A\u3001k\u756A\u76EE\u306E\
    \u5024\u3092`O(logN)`\u3067\u53D6\u308A\u51FA\u305B\u308B  \n//! \u89AA\u3092\u6301\
    \u3063\u3066\u3044\u306A\u3044\u306E\u3067\u3001iter\u3084range\u306F\u306A\u3044\
    \u3067\u3059...  \n//! \u30E9\u30F3\u30C0\u30E0\u3060\u3068BTreeSet\u3088\u308A\
    7\u500D\u3050\u3089\u3044\u9045\u3044\u306E\u3067\u3001\u672C\u5F53\u306B\u5FC5\
    \u8981\u306A\u3068\u304D\u3060\u3051\u4F7F\u3046\u306E\u304C\u3088\u3055\u305D\
    \u3046  \n//! \u5217\u3092\u7BA1\u7406\u3059\u308B\n\nuse std::cmp::Ordering;\n\
    use std::fmt::Display;\nuse std::iter::successors;\nuse std::mem::swap;\nuse std::ops::{Bound::*,\
    \ RangeBounds};\ntype Tree<T> = Option<Box<Node<T>>>;\n\n#[derive(Debug, Clone)]\n\
    struct Node<T> {\n    left: Tree<T>,\n    right: Tree<T>,\n    value: T,\n   \
    \ len: usize,\n    height: u8,\n}\n\nimpl<T> Node<T> {\n    fn new(value: T) ->\
    \ Node<T> {\n        Self {\n            left: None,\n            right: None,\n\
    \            value,\n            len: 1,\n            height: 1,\n        }\n\
    \    }\n    fn update(&mut self) {\n        self.len = len(&self.left) + len(&self.right)\
    \ + 1;\n        self.height = height(&self.left).max(height(&self.right)) + 1;\n\
    \    }\n    fn rotate_right(&mut self) {\n        let mut x = self.left.take().unwrap();\n\
    \        let b = x.right.take();\n        swap(self, &mut x);\n        x.left\
    \ = b;\n        x.update();\n        self.right = Some(x);\n        self.update();\n\
    \    }\n    fn rotate_left(&mut self) {\n        let mut x = self.right.take().unwrap();\n\
    \        let b = x.left.take();\n        swap(self, &mut x);\n        x.right\
    \ = b;\n        x.update();\n        self.left = Some(x);\n        self.update();\n\
    \    }\n    fn balance(&mut self) {\n        if height(&self.left).abs_diff(height(&self.right))\
    \ <= 1 {\n            self.update();\n            return;\n        }\n       \
    \ if height(&self.left) > height(&self.right) {\n            // \u5DE6\u306E\u5B50\
    \u306E\u53F3\u304C\u91CD\u3051\u308C\u3070\u5DE6\u56DE\u8EE2\n            let\
    \ left_child = self.left.as_mut().unwrap();\n            if height(&left_child.left)\
    \ < height(&left_child.right) {\n                left_child.rotate_left();\n \
    \           }\n            self.rotate_right();\n        } else {\n          \
    \  // \u53F3\u306E\u5B50\u306E\u5DE6\u304C\u91CD\u3051\u308C\u3070\u53F3\u56DE\
    \u8EE2\n            let right_child = self.right.as_mut().unwrap();\n        \
    \    if height(&right_child.left) > height(&right_child.right) {\n           \
    \     right_child.rotate_right();\n            }\n            self.rotate_left();\n\
    \        }\n    }\n\n    fn list_sub(self, ret: &mut Vec<T>) {\n        if let\
    \ Some(left) = self.left {\n            left.list_sub(ret);\n        }\n     \
    \   ret.push(self.value);\n        if let Some(right) = self.right {\n       \
    \     right.list_sub(ret);\n        }\n    }\n}\n\nimpl<T: Display> Display for\
    \ Node<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        if let Some(left) = &self.left {\n            write!(f, \"{}\", left)?;\n\
    \        }\n        write!(f, \"{}, \", self.value)?;\n        if let Some(right)\
    \ = &self.right {\n            write!(f, \"{}\", right)?;\n        }\n       \
    \ Ok(())\n    }\n}\n\nfn len<T>(tree: &Tree<T>) -> usize {\n    tree.as_ref().map_or(0,\
    \ |t| t.len)\n}\n\nfn height<T>(tree: &Tree<T>) -> u8 {\n    tree.as_ref().map_or(0,\
    \ |t| t.height)\n}\n\nfn merge<T>(left: Tree<T>, right: Tree<T>) -> Tree<T> {\n\
    \    match (left.is_some(), right.is_some()) {\n        (true, true) => {\n  \
    \          let (_, center, rhs) = split_delete(right.unwrap(), 0);\n         \
    \   Some(merge_with_root(left, center, rhs))\n        }\n        (false, _) =>\
    \ right,\n        (_, false) => left,\n    }\n}\n\nfn merge_with_root<T>(\n  \
    \  mut left: Tree<T>,\n    mut center: Box<Node<T>>,\n    mut right: Tree<T>,\n\
    ) -> Box<Node<T>> {\n    if height(&left).abs_diff(height(&right)) <= 1 {\n  \
    \      center.left = left;\n        center.right = right;\n        center.update();\n\
    \        center\n    } else if height(&left) < height(&right) {\n        let mut\
    \ root = right.take().unwrap();\n        root.left = Some(merge_with_root(left,\
    \ center, root.left.take()));\n        root.balance();\n        root\n    } else\
    \ {\n        let mut root = left.take().unwrap();\n        root.right = Some(merge_with_root(root.right.take(),\
    \ center, right));\n        root.balance();\n        root\n    }\n}\n\nfn split_delete<T>(mut\
    \ root: Box<Node<T>>, index: usize) -> (Tree<T>, Box<Node<T>>, Tree<T>) {\n  \
    \  debug_assert!((0..root.len).contains(&index));\n    let left = root.left.take();\n\
    \    let right = root.right.take();\n    let lsize = len(&left);\n    match lsize.cmp(&index)\
    \ {\n        Ordering::Equal => (left, root, right),\n        Ordering::Less =>\
    \ {\n            let mut ret = split_delete(right.unwrap(), index - lsize - 1);\n\
    \            ret.0 = Some(merge_with_root(left, root, ret.0));\n            ret\n\
    \        }\n        Ordering::Greater => {\n            let mut ret = split_delete(left.unwrap(),\
    \ index);\n            ret.2 = Some(merge_with_root(ret.2, root, right));\n  \
    \          ret\n        }\n    }\n}\n\n/// split into [0, index), [index, n)\n\
    fn split<T>(tree: Tree<T>, index: usize) -> (Tree<T>, Tree<T>) {\n    let Some(root)\
    \ = tree else {\n        return (None, None);\n    };\n    if index == 0 {\n \
    \       (None, Some(root))\n    } else if root.len == index {\n        (Some(root),\
    \ None)\n    } else {\n        let (left, center, right) = split_delete(root,\
    \ index);\n        (left, Some(merge_with_root(None, center, right)))\n    }\n\
    }\n\n/// value\u4EE5\u4E0A\u306E\u6700\u521D\u306E\u5024\u306Eindex\nfn lower_bound<T:\
    \ PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    let Some(tree) = tree\
    \ else {\n        return 0;\n    };\n    if value <= &tree.value {\n        lower_bound(&tree.left,\
    \ value)\n    } else {\n        len(&tree.left) + 1 + lower_bound(&tree.right,\
    \ value)\n    }\n}\n\n/// value\u3088\u308A\u5927\u304D\u3044\u6700\u521D\u306E\
    \u5024\u306Eindex\nfn upper_bound<T: PartialOrd>(tree: &Tree<T>, value: &T) ->\
    \ usize {\n    let Some(tree) = tree else {\n        return 0;\n    };\n    if\
    \ value >= &tree.value {\n        len(&tree.left) + 1 + upper_bound(&tree.right,\
    \ value)\n    } else {\n        upper_bound(&tree.left, value)\n    }\n}\n\nfn\
    \ count<T: PartialOrd>(tree: &Tree<T>, value: &T) -> usize {\n    upper_bound(tree,\
    \ value) - lower_bound(tree, value)\n}\n\nfn get<T>(tree: &Tree<T>, index: usize)\
    \ -> Option<&T> {\n    if len(tree) <= index {\n        return None;\n    }\n\
    \    let Some(tree) = tree else {\n        return None;\n    };\n    let left_len\
    \ = len(&tree.left);\n    match index.cmp(&left_len) {\n        Ordering::Less\
    \ => get(&tree.left, index),\n        Ordering::Equal => Some(&tree.value),\n\
    \        Ordering::Greater => get(&tree.right, index - left_len - 1),\n    }\n\
    }\n\n#[derive(Debug, Clone)]\npub struct AVL<T> {\n    root: Tree<T>,\n    multi:\
    \ bool, // \u91CD\u8907\u3092\u8A31\u3059\u304B\n}\n\nimpl<T: Display> Display\
    \ for AVL<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        if let Some(root) = &self.root {\n            write!(f, \"{}\", root)\n\
    \        } else {\n            write!(f, \"Empty\")\n        }\n    }\n}\n\nimpl<T>\
    \ AVL<T> {\n    /// \u91CD\u8907\u3092\u8A31\u3059\u306A\u3089true\n    pub fn\
    \ new(multi: bool) -> Self {\n        Self { root: None, multi }\n    }\n\n  \
    \  pub fn len(&self) -> usize {\n        len(&self.root)\n    }\n\n    pub fn\
    \ height(&self) -> u8 {\n        height(&self.root)\n    }\n\n    pub fn is_empty(&self)\
    \ -> bool {\n        self.root.is_none()\n    }\n\n    pub fn lower_bound(&self,\
    \ value: &T) -> usize\n    where\n        T: PartialOrd,\n    {\n        lower_bound(&self.root,\
    \ value)\n    }\n\n    pub fn upper_bound(&self, value: &T) -> usize\n    where\n\
    \        T: PartialOrd,\n    {\n        upper_bound(&self.root, value)\n    }\n\
    \n    /// index\u756A\u76EE(0-base)\u306E\u5024\u3092\u53D6\u5F97\n    pub fn\
    \ get(&self, index: usize) -> Option<&T> {\n        get(&self.root, index)\n \
    \   }\n\n    /// other\u306E\u4E2D\u8EAB\u3092\u7A7A\u306B\u3057\u306A\u304C\u3089\
    \u3001\u81EA\u5206\u306E\u53F3\u306B\u8FFD\u52A0\u3059\u308B\n    pub fn append(&mut\
    \ self, other: &mut Self) {\n        self.root = merge(self.root.take(), other.root.take());\n\
    \    }\n\n    /// [0, index)\u3092\u6B8B\u3057\u3001[index, n)\u3092\u8FD4\u3059\
    \n    pub fn split_off(&mut self, index: usize) -> Self {\n        assert!(index\
    \ <= self.len());\n        let (left, right) = split(self.root.take(), index);\n\
    \        self.root = left;\n        Self {\n            root: right,\n       \
    \     multi: self.multi,\n        }\n    }\n\n    fn get_left_right_range<R: RangeBounds<usize>>(&self,\
    \ range: R) -> (usize, usize) {\n        let left = match range.start_bound()\
    \ {\n            Included(&l) => l,\n            Excluded(&l) => l + 1,\n    \
    \        Unbounded => 0,\n        };\n        let right = match range.end_bound()\
    \ {\n            Included(&r) => r + 1,\n            Excluded(&r) => r,\n    \
    \        Unbounded => self.len(),\n        };\n        assert!(left <= right &&\
    \ right <= self.len());\n        (left, right)\n    }\n\n    /// range\u306E\u7BC4\
    \u56F2\u306B\u304A\u3044\u3066k\u3060\u3051\u5DE6\u56DE\u8EE2\u3059\u308B split\u3068\
    merge\u3092\u7528\u3044\u3066\u3044\u308B\u306E\u3067O(logN)\n    pub fn rotate_left<R:\
    \ RangeBounds<usize>>(&mut self, range: R, k: usize) {\n        let (left, right)\
    \ = self.get_left_right_range(range);\n        if left == right {\n          \
    \  return;\n        }\n        if k == 0 || k == right - left {\n            return;\n\
    \        }\n        assert!(k <= right - left);\n        let left_len = k;\n \
    \       let right_len = right - left - k;\n        let (left_tree, right_tree)\
    \ = split(self.root.take(), left + left_len);\n        let (left_tree, center_left_tree)\
    \ = split(left_tree, left);\n        let (center_right_tree, right_tree) = split(right_tree,\
    \ right_len);\n        let new_center_tree = merge(center_right_tree, center_left_tree);\n\
    \        self.root = merge(left_tree, merge(new_center_tree, right_tree));\n \
    \   }\n\n    /// range\u306E\u7BC4\u56F2\u306B\u304A\u3044\u3066k\u3060\u3051\u53F3\
    \u56DE\u8EE2\u3059\u308B split\u3068merge\u3092\u7528\u3044\u3066\u3044\u308B\u306E\
    \u3067O(logN)\n    pub fn rotate_right<R: RangeBounds<usize>>(&mut self, range:\
    \ R, k: usize) {\n        let (left, right) = self.get_left_right_range(range);\n\
    \        if left == right {\n            return;\n        }\n        if k == 0\
    \ || k == right - left {\n            return;\n        }\n        assert!(k <=\
    \ right - left);\n        let left_len = right - left - k;\n        let right_len\
    \ = k;\n        let (left_tree, right_tree) = split(self.root.take(), left + left_len);\n\
    \        let (left_tree, center_left_tree) = split(left_tree, left);\n       \
    \ let (center_right_tree, right_tree) = split(right_tree, right_len);\n      \
    \  let new_center_tree = merge(center_right_tree, center_left_tree);\n       \
    \ self.root = merge(left_tree, merge(new_center_tree, right_tree));\n    }\n\n\
    \    pub fn insert_by_index(&mut self, index: usize, value: T) {\n        assert!(index\
    \ <= self.len());\n        let other = self.split_off(index);\n        self.root\
    \ = Some(merge_with_root(\n            self.root.take(),\n            Box::new(Node::new(value)),\n\
    \            other.root,\n        ))\n    }\n\n    /// \u9069\u5207\u306A\u9806\
    \u5E8F\u3092\u4E8C\u5206\u63A2\u7D22\u3057\u3066\u633F\u5165\n    pub fn insert(&mut\
    \ self, value: T)\n    where\n        T: PartialOrd,\n    {\n        if !self.multi\
    \ && self.count(&value) > 0 {\n            return;\n        }\n        let index\
    \ = self.lower_bound(&value);\n        self.insert_by_index(index, value);\n \
    \   }\n\n    pub fn erase_by_index(&mut self, index: usize) -> Option<T> {\n \
    \       if index < self.len() {\n            let (left, center, right) = split_delete(self.root.take().unwrap(),\
    \ index);\n            self.root = merge(left, right);\n            Some(center.value)\n\
    \        } else {\n            None\n        }\n    }\n\n    /// \u4E8C\u5206\u63A2\
    \u7D22\u3067\u5024\u3092\u898B\u3064\u3051\u3066\u4E00\u3064\u524A\u9664\n   \
    \ pub fn erase(&mut self, value: &T) -> bool\n    where\n        T: PartialOrd,\n\
    \    {\n        if self.count(value) == 0 {\n            return false;\n     \
    \   }\n        let index = self.lower_bound(value);\n        let ret = self.erase_by_index(index);\n\
    \        ret.is_some()\n    }\n\n    pub fn contains(&self, value: &T) -> bool\n\
    \    where\n        T: PartialOrd,\n    {\n        self.count(value) > 0\n   \
    \ }\n\n    pub fn count(&self, value: &T) -> usize\n    where\n        T: PartialOrd,\n\
    \    {\n        count(&self.root, value)\n    }\n\n    pub fn into_vec(self) ->\
    \ Vec<T> {\n        let mut ret = Vec::with_capacity(self.len());\n        if\
    \ let Some(root) = self.root {\n            root.list_sub(&mut ret);\n       \
    \ }\n        ret\n    }\n\n    pub fn iter(&self) -> Iter<T> {\n        Iter {\n\
    \            stack: successors(self.root.as_deref(), |x| x.left.as_deref()).collect(),\n\
    \        }\n    }\n}\n\npub struct Iter<'a, T> {\n    stack: Vec<&'a Node<T>>,\n\
    }\n\nimpl<'a, T> Iterator for Iter<'a, T> {\n    type Item = &'a T;\n\n    fn\
    \ next(&mut self) -> Option<Self::Item> {\n        let node = self.stack.pop()?;\n\
    \        self.stack\n            .extend(successors(node.right.as_deref(), |x|\
    \ x.left.as_deref()));\n        Some(&node.value)\n    }\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n    use rand::prelude::*;\n    use std::collections::{BTreeMap,\
    \ BTreeSet};\n\n    fn stop_watch() -> f64 {\n        use std::time::{SystemTime,\
    \ UNIX_EPOCH};\n        static mut START: f64 = 0.0;\n        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n\
    \        let current = time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9;\n\
    \        unsafe {\n            let ret = current - START;\n            START =\
    \ current;\n            ret\n        }\n    }\n\n    #[test]\n    fn test_cnt()\
    \ {\n        let mut rng = thread_rng();\n        let mut avl = AVL::<i32>::new(true);\n\
    \        let mut set = BTreeMap::new();\n        const SIZE: usize = 100000;\n\
    \        for _ in 0..SIZE {\n            let value = rng.gen_range(-100..=100);\n\
    \            avl.insert(value);\n            *set.entry(value).or_insert(0) +=\
    \ 1;\n        }\n        for cnt in 0..SIZE {\n            if cnt % 2 == 0 {\n\
    \                let value = rng.gen_range(-100..=100);\n                let tree_cnt\
    \ = set.get(&value).copied().unwrap_or(0);\n                let rbst_cnt = avl.count(&value);\n\
    \                assert_eq!(tree_cnt, rbst_cnt);\n            } else if set.is_empty()\
    \ || rng.gen() {\n                let value = rng.gen_range(-100..=100);\n   \
    \             avl.insert(value);\n                *set.entry(value).or_insert(0)\
    \ += 1;\n            } else {\n                let value = rng.gen_range(-100..=100);\n\
    \                avl.erase(&value);\n                set.entry(value).and_modify(|x|\
    \ *x -= 1);\n                if let Some(x) = set.get(&value) {\n            \
    \        if *x == 0 {\n                        set.remove(&value);\n         \
    \           }\n                }\n            }\n        }\n    }\n\n    #[test]\n\
    \    fn test_kth_by_no_dups() {\n        let mut rng = thread_rng();\n       \
    \ let mut rbst = AVL::<i32>::new(false);\n        let mut set = BTreeSet::new();\n\
    \        for _ in 0..1000 {\n            let value = rng.gen_range(-100..=100);\n\
    \            if rbst.count(&value) == 0 {\n                rbst.insert(value);\n\
    \            }\n            set.insert(value);\n        }\n        for cnt in\
    \ 0..1000 {\n            if cnt % 2 == 0 {\n                let idx = rng.gen_range(0..set.len());\n\
    \                let value = set.iter().nth(idx).unwrap();\n                assert_eq!(rbst.get(idx).unwrap(),\
    \ value);\n            } else if set.is_empty() || rng.gen() {\n             \
    \   let value = rng.gen_range(-100..=100);\n                rbst.insert(value);\n\
    \                set.insert(value);\n            } else {\n                let\
    \ value = rng.gen_range(-100..=100);\n                rbst.erase(&value);\n  \
    \              set.remove(&value);\n            }\n        }\n    }\n\n    #[test]\n\
    \    fn test_bench() {\n        const SIZE: usize = 250000;\n        stop_watch();\n\
    \        let mut set = BTreeSet::new();\n        for i in 0..SIZE {\n        \
    \    set.insert(i);\n        }\n        println!(\"BTreeSet insert: {}\", stop_watch());\n\
    \        for i in 0..SIZE {\n            set.remove(&i);\n        }\n        println!(\"\
    BTreeSet erase: {}\", stop_watch());\n        let mut set = AVL::<usize>::new(true);\n\
    \        for i in 0..SIZE {\n            set.insert(i);\n        }\n        println!(\"\
    AVL insert: {}\", stop_watch());\n        println!(\"AVL height: {}\", set.height());\n\
    \        for i in 0..SIZE {\n            assert_eq!(set.get(i).unwrap(), &i);\n\
    \        }\n        println!(\"AVL get: {}\", stop_watch());\n        for i in\
    \ 0..SIZE {\n            set.erase(&i);\n        }\n        println!(\"AVL erase:\
    \ {}\", stop_watch());\n\n        let mut nums = (0..SIZE).collect::<Vec<_>>();\n\
    \        let mut rng = thread_rng();\n        nums.shuffle(&mut rng);\n      \
    \  stop_watch();\n        let mut set = BTreeSet::new();\n        for i in 0..SIZE\
    \ {\n            set.insert(nums[i]);\n        }\n        println!(\"BTreeSet\
    \ shuffle insert: {}\", stop_watch());\n        for i in 0..SIZE {\n         \
    \   assert!(set.remove(&i));\n        }\n        println!(\"BTreeSet shuffle erase:\
    \ {}\", stop_watch());\n        let mut set = AVL::<usize>::new(true);\n     \
    \   for i in 0..SIZE {\n            set.insert(nums[i]);\n        }\n        println!(\"\
    AVL shuffle insert: {}\", stop_watch());\n        println!(\"AVL shuffle height:\
    \ {}\", set.height());\n        for i in 0..SIZE {\n            assert_eq!(set.get(i).unwrap(),\
    \ &i);\n        }\n        println!(\"AVL shuffle get: {}\", stop_watch());\n\
    \        stop_watch();\n        for i in 0..SIZE {\n            set.erase(&i);\n\
    \        }\n        println!(\"AVL shuffle erase: {}\", stop_watch());\n    }\n\
    \n    #[test]\n    fn test_hack() {\n        const SIZE: usize = 250000;\n   \
    \     stop_watch();\n        let mut set = AVL::<usize>::new(true);\n        for\
    \ i in (0..SIZE).rev() {\n            set.insert(i);\n        }\n        println!(\"\
    insert rev: {}\", stop_watch());\n        println!(\"height: {}\", set.height());\n\
    \        stop_watch();\n        let mut set = AVL::<usize>::new(true);\n     \
    \   for i in 0..SIZE {\n            set.insert(i ^ 0xFFF);\n        }\n      \
    \  println!(\"insert xor: {}\", stop_watch());\n        println!(\"height: {}\"\
    , set.height());\n        stop_watch();\n        let mut set = AVL::<usize>::new(true);\n\
    \        for i in 0..SIZE {\n            if i % 2 == 0 {\n                set.insert(i);\n\
    \            } else {\n                set.insert(usize::MAX - i);\n         \
    \   }\n        }\n        println!(\"insert from edges: {}\", stop_watch());\n\
    \        println!(\"height: {}\", set.height());\n    }\n\n    #[test]\n    fn\
    \ test_iter() {\n        let mut set = AVL::<usize>::new(true);\n        const\
    \ SIZE: usize = 100000;\n        for i in 0..SIZE {\n            set.insert(i);\n\
    \        }\n        let mut iter = set.iter();\n        for i in 0..SIZE {\n \
    \           assert_eq!(iter.next(), Some(&i));\n        }\n        assert_eq!(iter.next(),\
    \ None);\n\n        let mut set = AVL::<usize>::new(true);\n        let mut rng\
    \ = thread_rng();\n        let mut nums = (0..SIZE).collect::<Vec<_>>();\n   \
    \     nums.shuffle(&mut rng);\n        for i in 0..SIZE {\n            set.insert(nums[i]);\n\
    \        }\n        let mut iter = set.iter();\n        for i in 0..SIZE {\n \
    \           assert_eq!(iter.next(), Some(&i));\n        }\n        assert_eq!(iter.next(),\
    \ None);\n    }\n\n    #[test]\n    fn test_rotate() {\n        let mut set =\
    \ AVL::<usize>::new(true);\n        const SIZE: usize = 1000;\n        for i in\
    \ 0..SIZE {\n            set.insert(i);\n        }\n        let mut vec = (0..SIZE).collect::<Vec<_>>();\n\
    \        let mut rng = thread_rng();\n        for _ in 0..SIZE {\n           \
    \ let l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..=SIZE);\n\
    \            let k = rng.gen_range(0..=r - l);\n            if rng.gen() {\n \
    \               vec[l..r].rotate_left(k);\n                set.rotate_left(l..r,\
    \ k);\n            } else {\n                vec[l..r].rotate_right(k);\n    \
    \            set.rotate_right(l..r, k);\n            }\n            assert!(vec.iter().eq(set.iter()));\n\
    \        }\n        for _ in 0..SIZE {\n            let k = rng.gen_range(0..SIZE);\n\
    \            if rng.gen() {\n                vec.rotate_left(k);\n           \
    \     set.rotate_left(.., k);\n            } else {\n                vec.rotate_right(k);\n\
    \                set.rotate_right(.., k);\n            }\n            assert!(vec.iter().eq(set.iter()));\n\
    \        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/avl/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-26 21:28:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_649_avl/src/main.rs
documentation_of: crates/data_structure/avl/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/avl/src/lib.rs
- /library/crates/data_structure/avl/src/lib.rs.html
title: crates/data_structure/avl/src/lib.rs
---
