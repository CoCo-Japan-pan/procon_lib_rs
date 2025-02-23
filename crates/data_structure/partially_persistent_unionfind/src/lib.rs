//! 部分永続UnionFind  
//! 全バージョンにアクセス可能であり、最新バージョンのみ更新可能なUnionFind  
//! 経路圧縮はできないのでクエリはO(logN)  
//! <https://blog.tiramister.net/posts/persistent-unionfind/>

pub struct PartiallyPersistentUnionFind {
    size: usize,
    /// 現在時刻
    now: usize,
    /// 各ノードの親(根の場合は自身)
    par: Vec<usize>,
    /// 各ノードの親の更新時刻
    time: Vec<usize>,
    /// (時刻、集合のサイズ)の記録
    num: Vec<Vec<(usize, usize)>>,
}

impl PartiallyPersistentUnionFind {
    pub fn new(size: usize) -> Self {
        PartiallyPersistentUnionFind {
            size,
            now: 0,
            par: (0..size).collect(),
            time: vec![usize::MAX; size],
            num: vec![vec![(0, 1)]; size],
        }
    }

    /// 時刻tにおいてxの属する集合の根を返す
    pub fn leader(&self, x: usize, t: usize) -> usize {
        assert!(x < self.size);
        assert!(t <= self.now);
        if self.time[x] > t {
            x
        } else {
            self.leader(self.par[x], t)
        }
    }

    /// 時刻tにおいて、xとyが同じ集合に属するかどうかを返す
    pub fn same(&self, x: usize, y: usize, t: usize) -> bool {
        self.leader(x, t) == self.leader(y, t)
    }

    /// xとyが属する集合を併合し、時間を進める  
    /// 最新時刻を返す
    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.size);
        assert!(y < self.size);
        let (x, y) = (self.leader(x, self.now), self.leader(y, self.now));
        self.now += 1;
        if x == y {
            return self.now;
        }
        let x_size = self.num[x].last().unwrap().1;
        let y_size = self.num[y].last().unwrap().1;
        let (x, y) = if x_size < y_size { (y, x) } else { (x, y) };

        let new_size = x_size + y_size;
        self.num[x].push((self.now, new_size));

        // xにyをくっつける
        self.par[y] = x;
        self.time[y] = self.now;

        self.now
    }

    /// 時刻tにおいてxの属する集合のサイズを返す
    pub fn size(&self, x: usize, t: usize) -> usize {
        assert!(x < self.size);
        assert!(t <= self.now);
        let x = self.leader(x, t);

        let mut ok = 0;
        let mut ng = self.num[x].len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if self.num[x][mid].0 <= t {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        self.num[x][ok].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use unionfind::UnionFind;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        const SIZE: usize = 100;
        let mut puf = PartiallyPersistentUnionFind::new(SIZE);
        let mut ufs = vec![UnionFind::new(SIZE)];
        for _ in 0..SIZE * 2 {
            let a = rng.random_range(0..SIZE);
            let b = rng.random_range(0..SIZE);
            let mut last = ufs.last().unwrap().clone();
            last.merge(a, b);
            ufs.push(last);
            puf.merge(a, b);
        }
        for t in 0..=SIZE * 2 {
            for i in 0..SIZE {
                for j in 0..SIZE {
                    assert_eq!(ufs[t].same(i, j), puf.same(i, j, t));
                }
            }
            for i in 0..SIZE {
                assert_eq!(ufs[t].size(i), puf.size(i, t));
            }
        }
    }
}
