pub use geometry_basics::Point;

/// ソート済みの点列から凸包を求める `O(n)`  
/// ここでは`(lowerhull, upperhull)`の形で返す  
/// 各hullは辞書順最小の点<->辞書順最大の点の間へと反時計回りになっている  
/// つまり辞書順最小・最大の点は両方に含まれるので注意  
/// 同一直線上の点を含めるなら、`contain_mid_point`は`true`にする  
pub fn monotone_chain(points: &[Point], contain_mid_point: bool) -> (Vec<Point>, Vec<Point>) {
    for ls in points.windows(2) {
        assert!(ls[0] <= ls[1], "please sort the input for graham scan!!!");
    }
    let lower_hull = calc_hull(points.len(), points.iter(), contain_mid_point);
    let upper_hull = calc_hull(points.len(), points.iter().rev(), contain_mid_point);
    (lower_hull, upper_hull)
}

fn calc_hull<'a, T: Iterator<Item = &'a Point>>(
    len: usize,
    points: T,
    contain_mid_point: bool,
) -> Vec<Point> {
    let mut hull = Vec::with_capacity(len);
    for &p in points {
        while hull.len() > 1 {
            let second = hull[hull.len() - 2];
            let first = hull[hull.len() - 1];
            let from = second - first;
            let to = p - first;
            let cross = from.cross(to);
            if cross > 0 || (!contain_mid_point && cross == 0) {
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

/// ソート済みの点列から、最遠点対の距離の二乗を求める `O(n)`
pub fn calc_farthest_point_pair(points: &[Point]) -> i64 {
    let ch = {
        let (mut lower_hull, mut upper_hull) = monotone_chain(points, false);
        lower_hull.pop();
        upper_hull.pop();
        lower_hull.append(&mut upper_hull);
        lower_hull
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
