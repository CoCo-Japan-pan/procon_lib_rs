use algebra::{Action, Commutative};
use proconio::{fastout, input};
use rect_add_point_get::RectActPointGet;

#[derive(Clone, Copy, Debug)]
enum Query {
    Add((u32, u32, u32, u32, u64)),
    Get(u32, u32),
}

#[derive(Clone, Copy, Debug)]
struct AddMap(u64);
impl Action for AddMap {
    type Target = u64;
    fn id_map() -> Self {
        Self(0)
    }
    fn composition(&mut self, rhs: &Self) {
        self.0 += rhs.0;
    }
    fn mapping(&self, target: &mut Self::Target) {
        *target += self.0;
    }
}
impl Commutative for AddMap {}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        l_d_r_u_w: [(u32, u32, u32, u32, u64); n],
    }
    let querys = {
        let mut querys = Vec::with_capacity(q);
        for _ in 0..q {
            input! {
                t: u32,
            }
            match t {
                0 => {
                    input! {
                        l_d_r_u_w: (u32, u32, u32, u32, u64),
                    }
                    querys.push(Query::Add(l_d_r_u_w));
                }
                1 => {
                    input! {
                        x: u32,
                        y: u32,
                    }
                    querys.push(Query::Get(x, y));
                }
                _ => unreachable!(),
            }
        }
        querys
    };
    let get_points: Vec<(u32, u32)> = querys
        .iter()
        .filter_map(|q| match q {
            Query::Get(x, y) => Some((*x, *y)),
            _ => None,
        })
        .collect();
    let mut kdtree = RectActPointGet::<AddMap, _>::new(get_points);
    for (l, d, r, u, w) in l_d_r_u_w {
        kdtree.add_range(&(l..r), &(d..u), &AddMap(w));
    }
    // eprintln!("{:?}", kdtree);
    for q in querys {
        match q {
            Query::Add((l, d, r, u, w)) => {
                kdtree.add_range(&(l..r), &(d..u), &AddMap(w));
            }
            Query::Get(x, y) => {
                let ans = kdtree.get_composition(x, y);
                println!("{}", ans.0);
            }
        }
    }
}
