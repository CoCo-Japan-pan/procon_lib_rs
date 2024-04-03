// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2842

use algebra::{Commutative, Monoid};
use proconio::{fastout, input, marker::Usize1};
use segtree_2d::SegTree2D;
use std::collections::VecDeque;

pub enum AddMonoid {}
impl Monoid for AddMonoid {
    type Target = u32;
    fn id_element() -> Self::Target {
        0
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a + *b
    }
}
impl Commutative for AddMonoid {}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        q: usize,
    }
    let mut queue = VecDeque::new();
    let mut nama_yake = SegTree2D::<AddMonoid>::new(h, w);
    let mut tabereru = SegTree2D::<AddMonoid>::new(h, w);
    for _ in 0..q {
        input! {
            cur_time: usize,
            ci: usize,
            h1: Usize1,
            w1: Usize1,
        }
        // 一度焼きあがったら、そのたい焼きはもう見る必要無し
        // つまり各クエリの時刻ごとに、焼きあがったもののみを記録していく
        while let Some(&(time, h, w)) = queue.front() {
            if time + t > cur_time {
                break;
            }
            queue.pop_front();
            nama_yake.set(h, w, 0);
            tabereru.set(h, w, 1);
        }
        match ci {
            0 => {
                // 焼く
                queue.push_back((cur_time, h1, w1));
                nama_yake.set(h1, w1, 1);
            }
            1 => {
                // 食べる
                if tabereru.get(h1, w1) == 1 {
                    tabereru.set(h1, w1, 0);
                }
            }
            2 => {
                // たい焼きの状態を確認
                input! {
                    h2: Usize1,
                    w2: Usize1,
                }
                let nama_yake_cnt = nama_yake.prod(h1..=h2, w1..=w2);
                let tabereru_cnt = tabereru.prod(h1..=h2, w1..=w2);
                println!("{} {}", tabereru_cnt, nama_yake_cnt);
            }
            _ => unreachable!(),
        }
    }
}
