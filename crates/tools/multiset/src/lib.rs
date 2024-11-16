//! RustにはMultiSetが標準ライブラリにないので、BTreeMapで代用していたが、面倒なのでライブラリ化  
//! 最低限の機能しか提供していないので、`range`等使いたければ`buf`や`buf_mut`で内部のBTreeMapを取り出して使う

use std::borrow::Borrow;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MultiSet<K: Ord> {
    map: BTreeMap<K, usize>,
}

impl<K: Ord> From<Vec<K>> for MultiSet<K> {
    fn from(vec: Vec<K>) -> Self {
        let mut ret = MultiSet::new();
        for v in vec {
            ret.insert_one(v);
        }
        ret
    }
}

impl<K: Ord> MultiSet<K> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::default(),
        }
    }

    pub fn buf_mut(&mut self) -> &mut BTreeMap<K, usize> {
        &mut self.map
    }

    pub fn buf(&self) -> &BTreeMap<K, usize> {
        &self.map
    }

    /// keyを1個追加
    pub fn insert_one(&mut self, key: K) {
        self.map.entry(key).and_modify(|e| *e += 1).or_insert(1);
    }

    /// keyをc個追加
    pub fn insert_bunch(&mut self, key: K, c: usize) {
        self.map.entry(key).and_modify(|e| *e += c).or_insert(c);
    }

    /// keyを一つ削除する
    pub fn remove_one<Q>(&mut self, key: &Q) 
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(v) = self.map.get_mut(&key) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(&key);
            }
        }
    }

    /// keyをc個削除する
    pub fn remove_bunch<Q>(&mut self, key: &Q, c: usize) 
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(v) = self.map.get_mut(&key) {
            *v = v.saturating_sub(c);
            if *v == 0 {
                self.map.remove(&key);
            }
        }
    }

    /// keyをすべて削除する
    pub fn remove_all<Q>(&mut self, key: &Q) 
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.remove(&key);
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.contains_key(&key)
    }

    pub fn count<Q>(&self, key: &Q) -> usize 
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.get(&key).copied().unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn min_key(&self) -> Option<&K> {
        self.map.keys().next()
    }

    pub fn max_key(&self) -> Option<&K> {
        self.map.keys().next_back()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        let mut ms = MultiSet::new();
        let mut v = vec![];
        for _ in 0..1000 {
            let x = rng.gen_range(0..10);
            let cnt = rng.gen_range(1..=10);
            if rng.gen() {
                ms.insert_one(x);
                v.push(x);
            } else {
                ms.insert_bunch(x, cnt);
                v.extend(std::iter::repeat(x).take(cnt));
            }
            let x = rng.gen_range(0..10);
            let cnt = rng.gen_range(1..=5);
            if rng.gen() {
                ms.remove_one(&x);
                if let Some(pos) = v.iter().position(|&y| y == x) {
                    v.remove(pos);
                }
            } else {
                ms.remove_bunch(&x, cnt);
                for _ in 0..cnt {
                    if let Some(pos) = v.iter().position(|&y| y == x) {
                        v.remove(pos);
                    }
                }
            }
        }
        for x in v.iter() {
            assert_eq!(ms.count(x), v.iter().filter(|&&y| y == *x).count());
        }
        for x in v.iter() {
            ms.remove_one(x);
        }
        assert!(ms.is_empty());
    }
}