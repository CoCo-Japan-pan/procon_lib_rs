//! [Mo's algorithm](https://ei1333.hateblo.jp/entry/2017/09/11/211011)  
//! 一般に二つのindex `(x, y)` をとるクエリ達に対して、`x`や`y`を+-1する変更を`O(α)`でできる場合  
//! `0 <= x <= nx, 0 <= y <= ny` とすると全体で`O(α\sqrt{nxnyQ})`で処理できる (Q = クエリの数)  
//! クエリ先読みが必要  
//! index`(x, y)`が区間クエリ`[x, y)`に対応する場合がよくある使い方だが、
//! 別に`x <= y`である必要はなく、一般に二つのindexをとるクエリ全般に使える

/// 現在のクエリの値、indexの+-1の変更、答えの配列を管理するtrait  
/// クエリ(0, 0)で初期化する
pub trait MoFuncs {
    /// (x, y) -> (x-1, y) の変更
    fn x_minus(&mut self, x: usize);
    /// (x, y) -> (x+1, y) の変更
    fn x_plus(&mut self, x: usize);
    /// (x, y) -> (x, y-1) の変更
    fn y_minus(&mut self, y: usize);
    /// (x, y) -> (x, y+1) の変更
    fn y_plus(&mut self, y: usize);
    /// 現在のクエリの値を、答えの配列のidx番目に記録する
    fn memo(&mut self, idx: usize);
}

#[derive(Debug)]
pub struct MoRunner<'a> {
    querys: &'a [(usize, usize)],
    order: Vec<usize>,
}

impl<'a> MoRunner<'a> {
    /// `querys` はオフラインで読み込んだクエリ(x, y) の配列  
    /// `0 <= x <= nx, 0 <= y <= ny` とする
    pub fn new(querys: &'a [(usize, usize)], nx: usize, ny: usize) -> Self {
        let order = calc_mo_friendly_order(querys, nx, ny);
        Self { querys, order }
    }

    /// クエリ(0, 0)から初めて、indexの+-1の変更を行いすべてのクエリを処理する  
    /// クエリの値を順に保持した配列を返す
    pub fn run<M: MoFuncs>(&self, mo_funcs: &mut M) {
        let mut x = 0;
        let mut y = 0;
        for id in &self.order {
            let (nx, ny) = self.querys[*id];
            while x > nx {
                mo_funcs.x_minus(x);
                x -= 1;
            }
            while y < ny {
                mo_funcs.y_plus(y);
                y += 1;
            }
            while x < nx {
                mo_funcs.x_plus(x);
                x += 1;
            }
            while y > ny {
                mo_funcs.y_minus(y);
                y -= 1;
            }
            mo_funcs.memo(*id);
        }
    }
}

/// クエリの左右端+-1変化が少なくなるように、クエリ番号[0,1,...Q)をソートした配列を返す
fn calc_mo_friendly_order(query_ranges: &[(usize, usize)], nx: usize, ny: usize) -> Vec<usize> {
    if query_ranges.is_empty() {
        return vec![];
    }
    let query_cnt = query_ranges.len();
    let mut order = (0..query_cnt).collect::<Vec<_>>();
    let block_size = ((nx as f64 * ny as f64 / query_cnt as f64).sqrt().ceil() as usize).max(1);
    // x/block_sizeでソート その中ではyでソート yについては昇順と降順を交互にする
    order.sort_unstable_by(|&a, &b| {
        let (x1, y1) = query_ranges[a];
        let (x2, y2) = query_ranges[b];
        let block1 = x1 / block_size;
        let block2 = x2 / block_size;
        block1.cmp(&block2).then(if (block1 & 1) == 0 {
            y1.cmp(&y2)
        } else {
            y2.cmp(&y1)
        })
    });
    order
}
