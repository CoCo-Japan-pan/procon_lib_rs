//! ポテンシャル付きUnion-Find  
//! 可換群を載せる  
use algebra::{Commutative, Group};
use std::cell::RefCell;
use DiffOrSize::*;

#[derive(Debug, Clone, Copy)]
enum DiffOrSize<M> {
    /// 親のノード番号と、親から見た差分
    Diff(usize, M),
    /// 自身が親なら、その集合のサイズを持つ
    Size(usize),
}

#[derive(Debug)]
pub struct PotentializedUnionFind<M: Group + Commutative> {
    n: usize,
    potential: RefCell<Vec<DiffOrSize<M::Target>>>,
}

impl<M: Group + Commutative> PotentializedUnionFind<M> {
    pub fn new(size: usize) -> Self {
        PotentializedUnionFind {
            n: size,
            potential: RefCell::new(vec![Size(1); size]),
        }
    }

    /// xからみたyの差分を追加する  
    /// 今までの関係と矛盾しない場合、呼び出し前に差分が未定義なら`Ok(true)`、定義済みなら`Ok(false)`を返す  
    /// 今までの関係と矛盾する場合、元々定義されているxから見たyの差分をeとして`Err(e)`を返す
    pub fn relate(&mut self, x: usize, y: usize, diff: M::Target) -> Result<bool, M::Target> {
        assert!(x < self.n);
        assert!(y < self.n);
        let (x, x_diff) = self.root_and_diff(x);
        let (y, y_diff) = self.root_and_diff(y);
        if x == y {
            if M::binary_operation(&x_diff, &diff) == y_diff {
                Ok(false)
            } else {
                Err(M::binary_operation(&M::inverse(&x_diff), &y_diff))
            }
        } else {
            let mut pot = self.potential.borrow_mut();
            if let (&Size(x_size), &Size(y_size)) = (&pot[x], &pot[y]) {
                if x_size > y_size {
                    let diff = M::binary_operation(
                        &M::binary_operation(&x_diff, &diff),
                        &M::inverse(&y_diff),
                    );
                    pot[x] = Size(x_size + y_size);
                    pot[y] = Diff(x, diff);
                } else {
                    let diff = M::binary_operation(
                        &M::binary_operation(&y_diff, &diff),
                        &M::inverse(&x_diff),
                    );
                    pot[y] = Size(x_size + y_size);
                    pot[x] = Diff(y, diff);
                }
                Ok(true)
            } else {
                unreachable!()
            }
        }
    }

    /// 代表元と、それから見た差分を求める
    pub fn root_and_diff(&self, mut x: usize) -> (usize, M::Target) {
        assert!(x < self.n);
        let mut pot = self.potential.borrow_mut();
        let mut buf = vec![];
        while let Diff(par, diff) = &pot[x] {
            buf.push((x, diff.clone()));
            x = *par;
        }
        buf.push((x, M::id_element()));
        buf.reverse();
        for i in 1..buf.len() {
            let (v, ref diff) = buf[i];
            let (par, _) = buf[i - 1];
            let par_pot = if let Diff(_, par_pot) = &pot[par] {
                par_pot
            } else {
                &buf[0].1
            };
            let new_diff = M::binary_operation(diff, par_pot);
            pot[v] = Diff(par, new_diff);
        }
        buf.last().unwrap().clone()
    }

    /// xから見たyの差分が定義されていれば返す
    pub fn diff(&self, x: usize, y: usize) -> Option<M::Target> {
        assert!(x < self.n);
        assert!(y < self.n);
        let (x, x_diff) = self.root_and_diff(x);
        let (y, y_diff) = self.root_and_diff(y);
        if x == y {
            Some(M::binary_operation(&M::inverse(&x_diff), &y_diff))
        } else {
            None
        }
    }
}
