use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 512;

#[derive(Eq, PartialEq)]
struct Query {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
    area: i64,
}

impl Query {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize, area: i64) -> Self {
        Query { lx, rx, ly, ry, area }
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.lx / BLOCK_SIZE, self.rx).cmp(&(other.lx / BLOCK_SIZE, other.rx))
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let mut unique_coords: HashSet<i32> = HashSet::new();
        for &x in x_coord {
            unique_coords.insert(x);
        }
        for &y in y_coord {
            unique_coords.insert(y);
        }
        let mut unique_coords: Vec<i32> = unique_coords.into_iter().collect();
        unique_coords.sort();

        let mut to_compressed: HashMap<i32, usize> = HashMap::new();
        let mut to_original: HashMap<usize, i32> = HashMap::new();
        for (i, &coord) in unique_coords.iter().enumerate() {
            to_compressed.insert(coord, i);
            to_original.insert(i, coord);
        }

        let x_coord: Vec<usize> = x_coord.iter().map(|&x| to_compressed[&x]).collect();
        let y_coord: Vec<usize> = y_coord.iter().map(|&y| to_compressed[&y]).collect();

        let mut by_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut by_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..n {
            by_x.entry(x_coord[i]).or_default().push(y_coord[i]);
            by_y.entry(y_coord[i]).or_default().push(x_coord[i]);
        }

        for v in by_x.values_mut() {
            v.sort();
        }
        for v in by_y.values_mut() {
            v.sort();
        }

        let mut queries: Vec<Query> = Vec::new();
        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];

            if let Some(&rx) = by_y[&ay].iter().find(|&&x| x > ax) {
                let ry = ay;

                if let Some(&ty) = by_x[&ax].iter().find(|&&y| y > ay) {
                    let tx = ax;

                    if let Some(&ty_r) = by_x[&rx].iter().find(|&&y| y > ry) {
                        if let Some(&rx_t) = by_y[&ty].iter().find(|&&x| x > tx) {
                            if ty_r == ty && rx_t == rx {
                                let dx = ax + 1;
                                let dy = ay + 1;
                                let ux = rx - 1;
                                let uy = ty - 1;
                                let area = (to_original[&ty] - to_original[&ay]) as i64
                                    * (to_original[&rx] - to_original[&ax]) as i64;

                                if dx <= ux && dy <= uy {
                                    queries.push(Query::new(dx, ux, dy, uy, area));
                                } else {
                                    ans = ans.max(area);
                                }
                            }
                        }
                    }
                }
            }
        }

        queries.sort();

        let mut cur_l = 0;
        let mut cur_r = 0;
        let mut ms: BTreeSet<usize> = BTreeSet::new();

        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                if let Some(v) = by_x.get(&cur_l) {
                    for &y in v {
                        ms.insert(y);
                    }
                }
            }
            while cur_r <= q.rx {
                if let Some(v) = by_x.get(&cur_r) {
                    for &y in v {
                        ms.insert(y);
                    }
                }
                cur_r += 1;
            }
            while cur_l < q.lx {
                if let Some(v) = by_x.get(&cur_l) {
                    for &y in v {
                        ms.remove(&y);
                    }
                }
                cur_l += 1;
            }
            while cur_r > q.rx + 1 {
                cur_r -= 1;
                if let Some(v) = by_x.get(&(cur_r - 1)) {
                    for &y in v {
                        ms.remove(&y);
                    }
                }
            }

            if let Some(&y) = ms.range(q.ly..).next() {
                if y <= q.ry {
                    continue;
                }
            }
            ans = ans.max(q.area);
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the number of points
    let n: usize = lines.next().unwrap()?.parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    // Input the coordinates of the points
    for _ in 0..n {
        let mut coords = lines.next().unwrap()?.split_whitespace();
        x_coord.push(coords.next().unwrap().parse().unwrap());
        y_coord.push(coords.next().unwrap().parse().unwrap());
    }

    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    println!("{}", max_area);

    Ok(())
}