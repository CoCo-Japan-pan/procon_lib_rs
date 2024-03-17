//! [abc345e](https://atcoder.jp/contests/abc345/tasks/abc345_e) でTop2のみ保持するものが欲しくなったのでライブラリ化

#[derive(Debug, Clone, Copy)]
enum Inner<K: Copy, V: Copy> {
    Empty,
    One(K, V),
    Two([(K, V); 2]),
}

/// Top2(大きい順)までのMapを持つ ただし同じKeyは一つまで
#[derive(Debug, Clone, Copy)]
pub struct Top2Map<K: Eq + Copy, V: Ord + Copy> {
    map: Inner<K, V>,
}

impl<K: Eq + Copy, V: Ord + Copy> Default for Top2Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Copy, V: Ord + Copy> Top2Map<K, V> {
    pub fn new() -> Self {
        Self { map: Inner::Empty }
    }
    #[allow(clippy::collapsible_else_if)]
    pub fn insert(&mut self, key: K, value: V) {
        match self.map {
            Inner::Empty => {
                self.map = Inner::One(key, value);
            }
            Inner::One(k, v) => {
                if key == k {
                    self.map = Inner::One(key, v.max(value));
                } else {
                    if value > v {
                        self.map = Inner::Two([(key, value), (k, v)]);
                    } else {
                        self.map = Inner::Two([(k, v), (key, value)]);
                    }
                }
            }
            Inner::Two([(k1, v1), (k2, v2)]) => {
                if key == k1 {
                    self.map = Inner::Two([(key, v1.max(value)), (k2, v2)]);
                } else if key == k2 {
                    let new_k2_val = v2.max(value);
                    if new_k2_val > v1 {
                        self.map = Inner::Two([(k2, new_k2_val), (k1, v1)]);
                    } else {
                        self.map = Inner::Two([(k1, v1), (k2, new_k2_val)]);
                    }
                } else {
                    if value > v1 {
                        self.map = Inner::Two([(key, value), (k1, v1)]);
                    } else if value > v2 {
                        self.map = Inner::Two([(k1, v1), (key, value)]);
                    }
                }
            }
        }
    }
    pub fn first(&self) -> Option<(K, V)> {
        match self.map {
            Inner::Empty => None,
            Inner::One(k, v) => Some((k, v)),
            Inner::Two([(k, v), _]) => Some((k, v)),
        }
    }
    pub fn second(&self) -> Option<(K, V)> {
        match self.map {
            Inner::Empty => None,
            Inner::One(_, _) => None,
            Inner::Two([_, (k, v)]) => Some((k, v)),
        }
    }
}
