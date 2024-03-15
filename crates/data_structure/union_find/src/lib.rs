//! merge以外は(意味的に)&selfにしたいので、RefCellを使用している

use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind {
    n: usize,
    /// rootなら、その集合のサイズを負の値で持つ
    /// それ以外なら、親のノード番号を持つ
    parent_or_size: RefCell<Vec<i32>>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            n: size,
            parent_or_size: RefCell::new(vec![-1; size]),
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        let mut par = self.parent_or_size.borrow_mut();
        if -par[x] < -par[y] {
            std::mem::swap(&mut x, &mut y);
        }
        par[x] += par[y];
        par[y] = x as i32;
        x
    }

    pub fn leader(&self, mut a: usize) -> usize {
        assert!(a < self.n);
        let mut par = self.parent_or_size.borrow_mut();
        let mut leader = a;
        while par[leader] >= 0 {
            leader = par[leader] as usize;
        }
        // 経路圧縮
        while par[a] >= 0 {
            let next = par[a] as usize;
            par[a] = leader as i32;
            a = next;
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
        -self.parent_or_size.borrow()[leader] as usize
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
