pub use geometry_basics::Point;

/// ソート済みの点列から凸包を求める `O(n)`  
/// 辞書順最小の点から、反時計周りに並べて返す(辞書順最小の点は最初と最後の二回登場するので注意!!!)  
/// つまり`lower_hull(左->右) -> upper_hull(右->左)`の順に連結している  
/// 同一直線上の点を含めるなら、`contain_mid_edge`はtrueにする  
pub fn monotone_chain(points: &[Point], contain_mid_edge: bool) -> Vec<Point> {
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
                !contain_mid_edge,
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
                !contain_mid_edge,
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
