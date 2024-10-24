pub use geometry_basics::Point;

/// ソート済みの点列から凸包を求める `O(n)`  
/// 辞書順最小の点から、反時計周りに並べて返す(辞書順最小の点は最初と最後の二回登場するので注意!!!)  
/// つまり`lower_hull(左->右) -> upper_hull(右->左)`の順に連結している  
/// 同一直線上の点を含めるなら、`contain_mid_point`は`true`にする  
pub fn monotone_chain(points: &[Point], contain_mid_point: bool) -> Vec<Point> {
    for ls in points.windows(2) {
        assert!(ls[0] <= ls[1], "please sort the input for graham scan!!!");
    }
    let mut hull = Vec::with_capacity(points.len() * 2);
    for &p in points.iter() {
        while hull.len() > 1 {
            if is_clock_wise(
                hull[hull.len() - 2],
                hull[hull.len() - 1],
                p,
                !contain_mid_point,
            ) {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(p);
    }
    let upper_hull_len = hull.len();
    for &p in points.iter().rev().skip(1) {
        while hull.len() > upper_hull_len {
            if is_clock_wise(
                hull[hull.len() - 2],
                hull[hull.len() - 1],
                p,
                !contain_mid_point,
            ) {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(p);
    }
    hull.shrink_to(0);
    hull
}

#[inline]
fn is_clock_wise(second: Point, first: Point, new_point: Point, exclude_zero: bool) -> bool {
    let from = second - first;
    let to = new_point - first;
    let cross = from.cross(to);
    cross > 0 || (exclude_zero && cross == 0)
}

/// ソート済みの点列から、最遠点対の距離の二乗を求める `O(n)`
pub fn calc_farthest_point_pair(points: &[Point]) -> i64 {
    let ch = {
        let mut ret = monotone_chain(points, false);
        ret.pop();
        ret
    };
    // rotating calipers
    let len = ch.len();
    if len == 2 {
        return (ch[0] - ch[1]).square_dist();
    }
    let mut i = ch.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
    let mut j = ch.iter().enumerate().max_by_key(|(_, p)| **p).unwrap().0;
    let mut dist = 0;
    let si = i;
    let sj = j;
    while i != sj || j != si {
        dist = dist.max((ch[i] - ch[j]).square_dist());
        let i_i1 = ch[(i + 1) % len] - ch[i];
        let j_j1 = ch[(j + 1) % len] - ch[j];
        if i_i1.cross(j_j1) < 0 {
            i = (i + 1) % len;
        } else {
            j = (j + 1) % len;
        }
    }
    dist
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_farthest_pairs() {
        for _ in 0..10 {
            let mut rng = thread_rng();
            let n = 1000;
            let mut points = vec![];
            for _ in 0..n {
                points.push(Point::new(
                    rng.gen_range(-1000_000_000..=1000_000_000),
                    rng.gen_range(-1000_000_000..=1000_000_000),
                ));
            }
            points.sort_unstable();
            let ans = calc_farthest_point_pair(&points);
            let mut max_dist = 0;
            for i in 0..n {
                for j in i + 1..n {
                    max_dist = max_dist.max((points[i] - points[j]).square_dist());
                }
            }
            assert_eq!(ans, max_dist);
        }
    }
}
