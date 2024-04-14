// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use fenwick_tree::FenwickTree;
use mo::calc_mo_friendly_order;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        l_r: [(usize, usize); q],
    }
    let a = {
        let mut a_cpy = a.clone();
        a_cpy.sort();
        a_cpy.dedup();
        let mut ret = vec![0; a.len()];
        for (r, a) in ret.iter_mut().zip(a) {
            *r = a_cpy.binary_search(&a).unwrap();
        }
        ret
    };
    let mut ft = FenwickTree::new(a.len(), 0_i64);
    let order = calc_mo_friendly_order(n, &l_r);
    let mut ans = vec![0; q];
    let mut left = 0;
    let mut right = 0;
    let mut cur_inv = 0;
    for id in order {
        let (l, r) = l_r[id];
        while left > l {
            left -= 1;

            // Add left
            let num = a[left];
            cur_inv += ft.sum(..num);
            ft.add(num, 1);
        }
        while right < r {
            // Add right
            let num = a[right];
            cur_inv += ft.sum(num + 1..);
            ft.add(num, 1);

            right += 1;
        }
        while left < l {
            // Remove left
            let num = a[left];
            cur_inv -= ft.sum(..num);
            ft.add(num, -1);

            left += 1;
        }
        while right > r {
            right -= 1;

            // Remove right
            let num = a[right];
            cur_inv -= ft.sum(num + 1..);
            ft.add(num, -1);
        }
        ans[id] = cur_inv;
    }
    for a in ans {
        println!("{}", a);
    }
}
