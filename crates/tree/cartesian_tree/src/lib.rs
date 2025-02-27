//! [Cartesian-tree](https://drken1215.hatenablog.com/entry/2023/10/19/025800)  
//! 数列のmin/maxで左右に分割していく分割統治法と相性がよい  

#[derive(Debug, Clone)]
pub struct CartesianTree {
    pub root: usize,
    pub parent: Vec<Option<usize>>,
    pub left_child: Vec<Option<usize>>,
    pub right_child: Vec<Option<usize>>,
}

impl CartesianTree {
    /// min_rootがtrueなら最小値を根に、falseなら最大値を根にする  
    /// tie-breakについては任意に順序をつけて二分木を保つ実装にしているので注意  
    pub fn new<T: PartialOrd>(list: &[T], min_root: bool) -> Self {
        let n = list.len();
        let mut parent = vec![None; n];
        let mut stack = Vec::with_capacity(n);
        let cmp = if min_root { |a, b| a > b } else { |a, b| a < b };
        for (i, a) in list.iter().enumerate() {
            let mut prev = None;
            while let Some(&j) = stack.last() {
                if cmp(&list[j], a) {
                    prev = stack.pop();
                } else {
                    break;
                }
            }
            if let Some(prev) = prev {
                parent[prev] = Some(i);
            }
            if let Some(&last) = stack.last() {
                parent[i] = Some(last);
            }
            stack.push(i);
        }
        let root = stack[0];
        let mut left_child = vec![None; n];
        let mut right_child = vec![None; n];
        for (i, p) in parent.iter().enumerate() {
            if let Some(p) = *p {
                if i < p {
                    left_child[p] = Some(i);
                } else {
                    right_child[p] = Some(i);
                }
            }
        }
        Self {
            root,
            parent,
            left_child,
            right_child,
        }
    }
}
