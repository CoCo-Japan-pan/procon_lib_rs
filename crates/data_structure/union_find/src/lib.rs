//! merge以外は(意味的に)&selfにしたいので、RefCellを使用している
use std::cell::RefCell;
use ParentOrSize::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParentOrSize {
    /// 親のノード番号
    Parent(usize),
    /// 自身が親なら、その集合のサイズを持つ
    Size(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind {
    n: usize,
    parent_or_size: RefCell<Vec<ParentOrSize>>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            n: size,
            parent_or_size: RefCell::new(vec![Size(1); size]),
        }
    }

    /// 合併しつつ、合併した集合の代表元を返す
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (x, y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        let mut par = self.parent_or_size.borrow_mut();
        let (bigger, smaller, size_sum) = {
            if let (Size(x_size), Size(y_size)) = (par[x], par[y]) {
                if x_size < y_size {
                    (y, x, x_size + y_size)
                } else {
                    (x, y, x_size + y_size)
                }
            } else {
                unreachable!()
            }
        };
        par[bigger] = Size(size_sum);
        par[smaller] = Parent(bigger);
        bigger
    }

    pub fn leader(&self, mut a: usize) -> usize {
        assert!(a < self.n);
        let mut par = self.parent_or_size.borrow_mut();
        let mut leader = a;
        while let Parent(p) = par[leader] {
            leader = p;
        }
        // 経路圧縮
        while let Parent(p) = par[a] {
            par[a] = Parent(leader);
            a = p;
        }
        leader
    }

    pub fn same(&self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn size(&self, a: usize) -> usize {
        assert!(a < self.n);
        let leader = self.leader(a);
        if let Size(size) = self.parent_or_size.borrow()[leader] {
            size
        } else {
            unreachable!()
        }
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![vec![]; self.n];
        for (res, size) in result.iter_mut().zip(group_size) {
            res.reserve(size);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}
