//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs>  
//! compositionやmappingに可変参照を用いているところと、作用が可変なら伝播を一部サボる部分が異なる

use algebra::{Commutative, MapMonoid, Monoid, NonCommutative};
use internal_bits::ceil_log2;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct LazySegTree<F: MapMonoid> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    data: Vec<<F::Monoid as Monoid>::Target>,
    lazy: Vec<F::Map>,
}

impl<F: MapMonoid> From<Vec<<F::Monoid as Monoid>::Target>> for LazySegTree<F> {
    fn from(v: Vec<<F::Monoid as Monoid>::Target>) -> Self {
        let range_size = v.len();
        let log = ceil_log2(range_size as u32) as usize;
        let leaf_size = 1 << log;
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

    pub fn set(&mut self, mut p: usize, x: <F::Monoid as Monoid>::Target) {
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

    pub fn get(&mut self, mut p: usize) -> <F::Monoid as Monoid>::Target {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p].clone()
    }

    pub fn prod<R: RangeBounds<usize>>(&mut self, range: R) -> <F::Monoid as Monoid>::Target {
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

    pub fn all_prod(&self) -> <F::Monoid as Monoid>::Target {
        self.data[1].clone()
    }

    pub fn apply(&mut self, mut p: usize, f: &F::Map) {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        F::mapping(&mut self.data[p], f);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(&<F::Monoid as Monoid>::Target) -> bool,
    {
        assert!(l <= self.range_size);
        assert!(g(&F::id_element()));
        if l == self.range_size {
            return self.range_size;
        }
        l += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = F::id_element();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(&F::binary_operation(&sm, &self.data[l])) {
                while l < self.leaf_size {
                    self.push(l);
                    l *= 2;
                    let res = F::binary_operation(&sm, &self.data[l]);
                    if !g(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.leaf_size;
            }
            sm = F::binary_operation(&sm, &self.data[l]);
            l += 1;
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.range_size
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where
        G: Fn(&<F::Monoid as Monoid>::Target) -> bool,
    {
        assert!(r <= self.range_size);
        assert!(g(&F::id_element()));
        if r == 0 {
            return 0;
        }
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = F::id_element();
        while {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !g(&F::binary_operation(&self.data[r], &sm)) {
                while r < self.leaf_size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::binary_operation(&self.data[r], &sm);
                    if !g(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.leaf_size;
            }
            sm = F::binary_operation(&self.data[r], &sm);
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }
}

impl<F: MapMonoid> LazySegTree<F>
where
    F::Map: Commutative,
{
    /// 可換な作用の区間適用
    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::Map) {
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
        self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
        F::mapping(&mut self.data[k], &self.lazy[k]);
    }
}

impl<F: MapMonoid> LazySegTree<F>
where
    F::Map: NonCommutative,
{
    /// 非可換な作用の区間適用
    pub fn apply_range_non_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::Map) {
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
    fn all_apply(&mut self, k: usize, f: &F::Map) {
        F::mapping(&mut self.data[k], f);
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
