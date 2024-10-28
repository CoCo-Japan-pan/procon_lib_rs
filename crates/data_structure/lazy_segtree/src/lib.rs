//! <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs> を基にしています  
//! compositionやmappingに可変参照を用いているところと、作用が可変なら伝播を一部サボる部分が異なる

use algebra::{Action, ActionMonoid, Commutative, Monoid, NonCommutative};
use internal_bits::ceil_log2;
use std::ops::{Bound::*, RangeBounds};

#[derive(Debug)]
pub struct LazySegTree<F: ActionMonoid> {
    range_size: usize,
    leaf_size: usize,
    log: usize,
    data: Vec<<F::M as Monoid>::Target>,
    lazy: Vec<F::A>,
}

impl<F: ActionMonoid> From<Vec<<F::M as Monoid>::Target>> for LazySegTree<F> {
    fn from(v: Vec<<F::M as Monoid>::Target>) -> Self {
        let range_size = v.len();
        let log = ceil_log2(range_size as u32) as usize;
        let leaf_size = 1 << log;
        let mut data = vec![F::M::id_element(); 2 * leaf_size];
        let lazy = vec![F::A::id_action(); leaf_size];
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

impl<F: ActionMonoid> LazySegTree<F> {
    pub fn new(n: usize) -> Self {
        vec![F::M::id_element(); n].into()
    }

    pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::Target) {
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

    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::Target {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p].clone()
    }

    fn get_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let l = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let r = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.range_size,
        };
        assert!(l <= r && r <= self.range_size);
        (l, r)
    }

    pub fn prod<R: RangeBounds<usize>>(&mut self, range: R) -> <F::M as Monoid>::Target {
        let (mut l, mut r) = self.get_range(range);
        if l == r {
            return F::M::id_element();
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
                self.push((r - 1) >> i);
            }
        }

        let mut sml = F::M::id_element();
        let mut smr = F::M::id_element();
        while l < r {
            if l & 1 != 0 {
                sml = F::M::binary_operation(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = F::M::binary_operation(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        F::M::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> <F::M as Monoid>::Target {
        self.data[1].clone()
    }

    pub fn apply(&mut self, mut p: usize, f: &F::A) {
        assert!(p < self.range_size);
        p += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        f.apply(&mut self.data[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(&<F::M as Monoid>::Target) -> bool,
    {
        assert!(l <= self.range_size);
        assert!(g(&F::M::id_element()));
        if l == self.range_size {
            return self.range_size;
        }
        l += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = F::M::id_element();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(&F::M::binary_operation(&sm, &self.data[l])) {
                while l < self.leaf_size {
                    self.push(l);
                    l *= 2;
                    let res = F::M::binary_operation(&sm, &self.data[l]);
                    if g(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.leaf_size;
            }
            sm = F::M::binary_operation(&sm, &self.data[l]);
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
        G: Fn(&<F::M as Monoid>::Target) -> bool,
    {
        assert!(r <= self.range_size);
        assert!(g(&F::M::id_element()));
        if r == 0 {
            return 0;
        }
        r += self.leaf_size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = F::M::id_element();
        while {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !g(&F::M::binary_operation(&self.data[r], &sm)) {
                while r < self.leaf_size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::M::binary_operation(&self.data[r], &sm);
                    if g(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.leaf_size;
            }
            sm = F::M::binary_operation(&self.data[r], &sm);
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }
}

impl<F: ActionMonoid> LazySegTree<F>
where
    F::A: Commutative,
{
    /// 可換な作用の区間適用
    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::A) {
        let (mut l, mut r) = self.get_range(range);
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
        self.data[k] = F::M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
        self.lazy[k].apply(&mut self.data[k]);
    }
}

impl<F: ActionMonoid> LazySegTree<F>
where
    F::A: NonCommutative,
{
    /// 非可換な作用の区間適用
    pub fn apply_range_non_commutative<R: RangeBounds<usize>>(&mut self, range: R, f: &F::A) {
        let (mut l, mut r) = self.get_range(range);
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

impl<F: ActionMonoid> LazySegTree<F> {
    /// dataを子から更新する
    fn update(&mut self, k: usize) {
        self.data[k] = F::M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
    }
    /// 作用を適用し、lazyノードがあれば(子があれば)作用を合成する
    fn all_apply(&mut self, k: usize, f: &F::A) {
        f.apply(&mut self.data[k]);
        if k < self.leaf_size {
            self.lazy[k].composition(f);
        }
    }
    /// 作用を子に押し込む
    fn push(&mut self, k: usize) {
        let mut parent = F::A::id_action();
        std::mem::swap(&mut parent, &mut self.lazy[k]);
        self.all_apply(2 * k, &parent);
        self.all_apply(2 * k + 1, &parent);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use algebra::Action;
    use rand::prelude::*;

    #[test]
    fn test_max_right_min_left() {
        // 区間加算、区間和
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct SumMonoid {
            sum: u64,
            len: u64,
        }
        impl Monoid for SumMonoid {
            type Target = Self;
            fn id_element() -> Self {
                Self { sum: 0, len: 0 }
            }
            fn binary_operation(a: &Self, b: &Self) -> Self {
                Self {
                    sum: a.sum + b.sum,
                    len: a.len + b.len,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct AddAction(u64);
        impl Action for AddAction {
            type Target = SumMonoid;
            fn id_action() -> Self {
                Self(0)
            }
            fn composition(&mut self, rhs: &Self) {
                self.0 += rhs.0;
            }
            fn apply(&self, target: &mut Self::Target) {
                target.sum += self.0 * target.len;
            }
        }
        impl Commutative for AddAction {}

        struct RARRSQ;
        impl ActionMonoid for RARRSQ {
            type M = SumMonoid;
            type A = AddAction;
        }

        let mut rng = rand::thread_rng();
        let n = 1000;
        let mut seg = LazySegTree::<RARRSQ>::from(
            (0..n)
                .map(|_| SumMonoid {
                    sum: rng.gen_range(0..=20000),
                    len: 1,
                })
                .collect::<Vec<_>>(),
        );
        for _ in 0..n * 10 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n);
            let x = rng.gen_range(0..=20000);
            seg.apply_range_commutative(l..r, &AddAction(x));

            let l = rng.gen_range(0..n);
            let bound = rng.gen_range(1..=200_000);
            let max_right = seg.max_right(l, |x| x.sum < bound);
            assert!(seg.prod(l..max_right).sum < bound);
            assert!(max_right == n || seg.prod(l..max_right + 1).sum >= bound);

            let r = rng.gen_range(0..n);
            let bound = rng.gen_range(1..=2000_000);
            let min_left = seg.min_left(r, |x| x.sum < bound);
            assert!(seg.prod(min_left..r).sum < bound);
            assert!(min_left == 0 || seg.prod(min_left - 1..r).sum >= bound);
        }
    }
}
