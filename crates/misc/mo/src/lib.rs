//! 区間[L, R)について、元の結果を用いて、LやRを+-1した区間の結果を`O(α)`でできる場合  
//! 全体を`O(αN√Q)`で処理できる (N = 区間全体の長さ, Q = クエリの数)  
//! クエリ先読みが必要  

pub trait MoFuncs {
    fn add_left(&mut self, idx: usize);
    fn add_right(&mut self, idx: usize);
    fn remove_left(&mut self, idx: usize);
    fn remove_right(&mut self, idx: usize);
    fn memo(&mut self, idx: usize);
}

#[derive(Debug)]
pub struct MoRunner {
    query_ranges: Vec<(usize, usize)>,
    order: Vec<usize>,
}

impl MoRunner {
    pub fn new(range_size: usize, query_ranges: Vec<(usize, usize)>) -> Self {
        let order = calc_mo_friendly_order(range_size, &query_ranges);
        Self {
            query_ranges,
            order,
        }
    }

    pub fn run<M: MoFuncs>(&self, mo_funcs: &mut M) {
        let mut left = 0;
        let mut right = 0;
        for id in &self.order {
            let (l, r) = self.query_ranges[*id];
            while left > l {
                left -= 1;
                mo_funcs.add_left(left);
            }
            while right < r {
                mo_funcs.add_right(right);
                right += 1;
            }
            while left < l {
                mo_funcs.remove_left(left);
                left += 1;
            }
            while right > r {
                right -= 1;
                mo_funcs.remove_right(right);
            }
            mo_funcs.memo(*id);
        }
    }
}

/// クエリの左右端+-1変化が少なくなるように、クエリ番号[0,1,...Q)をソートした配列を返す
fn calc_mo_friendly_order(range_size: usize, query_ranges: &Vec<(usize, usize)>) -> Vec<usize> {
    if query_ranges.is_empty() {
        return vec![];
    }
    let query_cnt = query_ranges.len();
    let mut order = (0..query_cnt).collect::<Vec<_>>();
    let block_size = (range_size / ((query_cnt as f64).sqrt().ceil() as usize)).max(1);
    // left/block_sizeでソート その中では右端でソート 右端については昇順と降順を交互にする
    order.sort_by(|&a, &b| {
        let (l1, r1) = query_ranges[a];
        let (l2, r2) = query_ranges[b];
        let block1 = l1 / block_size;
        let block2 = l2 / block_size;
        block1.cmp(&block2).then(if (block1 & 1) == 0 {
            r1.cmp(&r2)
        } else {
            r2.cmp(&r1)
        })
    });
    order
}
