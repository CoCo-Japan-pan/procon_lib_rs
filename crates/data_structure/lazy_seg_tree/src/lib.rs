use algebra::{CommutativeMapMonoid, MapMonoid, Monoid, NonCommutativeMapMonoid};
use std::ops::RangeBounds;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazySegTree<F: MapMonoid> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    data: Vec<<F::M as Monoid>::S>,
    lazy: Vec<F::F>,
}

impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegTree<F> {
    fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
        let range_size = v.len();
        let mut leaf_size = 1;
        let mut log = 0;
        while leaf_size < range_size {
            leaf_size *= 2;
            log += 1;
        }
        let mut data = vec![F::id_element(); 2 * leaf_size];
        let lazy = vec![F::id_map(); leaf_size];
        data[leaf_size..(leaf_size + range_size)].clone_from_slice(&v);
        let mut ret = Self {
            range_size,
            leaf_size,
            log,
            data,
            lazy,
        };
        for i in (1..leaf_size).rev() {
            ret.update(i);
        }
        ret
    }
}

impl<F: MapMonoid> LazySegTree<F> {
    pub fn new(n: usize) -> Self {
        vec![F::id_element(); n].into()
    }

    pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p].clone()
    }

    pub fn prod<R: RangeBounds<usize>>(&mut self, range: R) -> <F::M as Monoid>::S {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        if l == r {
            return F::id_element();
        }
        if l == 0 && r == self.range_size {
            return self.all_prod();
        }

        l += self.leaf_size;
        r += self.leaf_size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }

        let mut sml = F::id_element();
        let mut smr = F::id_element();
        while l < r {
            if l & 1 != 0 {
                sml = F::binary_operation(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = F::binary_operation(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        F::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> <F::M as Monoid>::S {
        self.data[1].clone()
    }

    pub fn apply(&mut self, mut p: usize, f: &F::F) {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = F::mapping(f, &self.data[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
}

impl<F: CommutativeMapMonoid> LazySegTree<F> {
    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::F) {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        if l == r {
            return;
        }

        l += self.leaf_size;
        r += self.leaf_size;

        // 可換なのでここの伝播をサボる

        {
            let l_copy = l;
            let r_copy = r;
            while l < r {
                if l & 1 != 0 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l_copy;
            r = r_copy;
        }

        // 伝播をサボったので、ここでlazyを考慮して更新する
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update_considering_lazy(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update_considering_lazy((r - 1) >> i);
            }
        }
    }

    fn update_considering_lazy(&mut self, k: usize) {
        self.data[k] = F::mapping(
            &self.lazy[k],
            &F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]),
        );
    }
}

impl<F: NonCommutativeMapMonoid> LazySegTree<F> {
    pub fn apply_range_non_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::F) {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        if l == r {
            return;
        }

        l += self.leaf_size;
        r += self.leaf_size;

        // 非可換なので、先に上から伝播しておく
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        {
            let l_copy = l;
            let r_copy = r;
            while l < r {
                if l & 1 != 0 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l_copy;
            r = r_copy;
        }

        // 先に伝播しているのでlazyの値を考慮せずに更新できる
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }
}

impl<F: MapMonoid> LazySegTree<F> {
    /// dataを子から更新する
    fn update(&mut self, k: usize) {
        self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
    }
    /// 作用を適用し、lazyノードがあれば(子があれば)作用を合成する
    fn all_apply(&mut self, k: usize, f: &F::F) {
        self.data[k] = F::mapping(f, &self.data[k]);
        if k < self.leaf_size {
            F::composition(&mut self.lazy[k], f);
        }
    }
    /// 作用を子に押し込む
    fn push(&mut self, k: usize) {
        let mut parent = F::id_map();
        std::mem::swap(&mut parent, &mut self.lazy[k]);
        self.all_apply(2 * k, &parent);
        self.all_apply(2 * k + 1, &parent);
    }
}
