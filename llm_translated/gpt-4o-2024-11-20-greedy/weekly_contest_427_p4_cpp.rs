use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};
use std::cmp::max;

// Constants
const BLOCK_SIZE: usize = 512;

#[derive(Debug, Clone)]
struct Query {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
    area: i64,
}

impl Query {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize, area: i64) -> Self {
        Self { lx, rx, ly, ry, area }
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        (self.lx / BLOCK_SIZE, self.rx) == (other.lx / BLOCK_SIZE, other.rx)
    }
}

impl Eq for Query {}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.lx / BLOCK_SIZE, self.rx).cmp(&(other.lx / BLOCK_SIZE, other.rx)))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.lx / BLOCK_SIZE, self.rx).cmp(&(other.lx / BLOCK_SIZE, other.rx))
    }
}

struct Solution;

impl Solution {
    fn max_rectangle_area(x_coord: &mut Vec<usize>, y_coord: &mut Vec<usize>) -> i64 {
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let mut to_compressed = HashMap::new();
        let mut to_original = HashMap::new();
        {
            let mut p: BTreeSet<usize> = BTreeSet::new();
            p.extend(x_coord.iter().cloned());
            p.extend(y_coord.iter().cloned());

            let mut lst = 0;
            for &a in &p {
                to_compressed.insert(a, lst);
                to_original.insert(lst, a);
                lst += 1;
            }

            for x in x_coord.iter_mut() {
                *x = *to_compressed.get(x).unwrap();
            }
            for y in y_coord.iter_mut() {
                *y = *to_compressed.get(y).unwrap();
            }
        }

        let mut by_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut by_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..n {
            by_x.entry(x_coord[i]).or_default().push(y_coord[i]);
            by_y.entry(y_coord[i]).or_default().push(x_coord[i]);
        }

        for (_, v) in by_x.iter_mut() {
            v.sort_unstable();
        }
        for (_, v) in by_y.iter_mut() {
            v.sort_unstable();
        }

        let mut queries = Vec::new();

        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];

            if let Some(it) = by_y[&ay].iter().find(|&&x| x > ax) {
                let rx = *it;
                let ry = ay;

                if let Some(it) = by_x[&ax].iter().find(|&&y| y > ay) {
                    let tx = ax;
                    let ty = *it;

                    if let Some(it_r) = by_x[&rx].iter().find(|&&y| y > ry) {
                        if let Some(it_t) = by_y[&ty].iter().find(|&&x| x > tx) {
                            if *it_r == ty && *it_t == rx {
                                let dx = ax + 1;
                                let dy = ay + 1;
                                let ux = rx - 1;
                                let uy = ty - 1;

                                let area = (to_original[&ty] - to_original[&ay]) as i64
                                    * (to_original[&rx] - to_original[&ax]) as i64;

                                if dx <= ux && dy <= uy {
                                    queries.push(Query::new(dx, ux, dy, uy, area));
                                } else {
                                    ans = max(ans, area);
                                }
                            }
                        }
                    }
                }
            }
        }

        queries.sort();

        // Mo's algorithm
        let mut cur_l = 0;
        let mut cur_r = 0;
        let mut ms = BTreeSet::new();

        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                ms.extend(&by_x[&cur_l]);
            }
            while cur_r < q.rx {
                cur_r += 1;
                ms.extend(&by_x[&cur_r]);
            }
            while cur_l < q.lx {
                for &b in &by_x[&cur_l] {
                    ms.remove(&b);
                }
                cur_l += 1;
            }
            while cur_r > q.rx {
                for &b in &by_x[&cur_r] {
                    ms.remove(&b);
                }
                cur_r -= 1;
            }

            if let Some(&it) = ms.range(q.ly..).next() {
                if it <= q.ry {
                    continue;
                }
            }
            ans = max(ans, q.area);
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    // Read the coordinates
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        x_coord.push(x);
        y_coord.push(y);
    }

    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(&mut x_coord, &mut y_coord);

    // Output the result
    println!("{}", max_area);
}