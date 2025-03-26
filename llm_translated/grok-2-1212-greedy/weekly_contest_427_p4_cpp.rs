use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 512;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Query {
    lx: usize,
    rx: usize,
    ly: i32,
    ry: i32,
    area: i64,
}

impl Query {
    fn new(lx: usize, rx: usize, ly: i32, ry: i32, area: i64) -> Self {
        Query { lx, rx, ly, ry, area }
    }
}

struct Solution;

impl Solution {
    fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let mut to_compressed = HashMap::new();
        let mut to_original = HashMap::new();
        {
            let mut p = BTreeSet::new();
            for &x in x_coord {
                p.insert(x);
            }
            for &y in y_coord {
                p.insert(y);
            }
            let mut lst = 0;
            for &a in &p {
                to_compressed.insert(a, lst);
                to_original.insert(lst, a);
                lst += 1;
            }
        }

        let mut x_coord: Vec<usize> = x_coord.iter().map(|&x| *to_compressed.get(&x).unwrap()).collect();
        let mut y_coord: Vec<usize> = y_coord.iter().map(|&y| *to_compressed.get(&y).unwrap()).collect();

        let mut by_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut by_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..n {
            by_x.entry(x_coord[i]).or_default().push(y_coord[i]);
            by_y.entry(y_coord[i]).or_default().push(x_coord[i]);
        }

        for v in by_x.values_mut() {
            v.sort_unstable();
        }
        for v in by_y.values_mut() {
            v.sort_unstable();
        }

        let mut queries = Vec::new();
        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];

            let rx = match by_y[&ay].binary_search(&(ax + 1)) {
                Ok(_) => continue,
                Err(idx) if idx == by_y[&ay].len() => continue,
                Err(idx) => by_y[&ay][idx],
            };

            let ty = match by_x[&ax].binary_search(&(ay + 1)) {
                Ok(_) => continue,
                Err(idx) if idx == by_x[&ax].len() => continue,
                Err(idx) => by_x[&ax][idx],
            };

            // Check existence of top right point
            let ty_exists = match by_x[&rx].binary_search(&ty) {
                Ok(_) => true,
                Err(_) => false,
            };
            let rx_exists = match by_y[&ty].binary_search(&rx) {
                Ok(_) => true,
                Err(_) => false,
            };

            if !ty_exists || !rx_exists {
                continue;
            }

            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;
            let area = (to_original[&ty] - to_original[&ay]) as i64 * (to_original[&rx] - to_original[&ax]) as i64;

            if dx <= ux && dy <= uy {
                queries.push(Query::new(dx, ux, dy as i32, uy as i32, area));
            } else {
                ans = ans.max(area);
            }
        }

        queries.sort_by_key(|q| (q.lx / BLOCK_SIZE, q.rx));

        // Mo's algorithm
        let mut cur_l = 0;
        let mut cur_r = 0;
        let mut ms = BTreeSet::new();

        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                for &b in &by_x[&cur_l] {
                    ms.insert(b as i32);
                }
            }
            while cur_r <= q.rx {
                for &b in &by_x[&cur_r] {
                    ms.insert(b as i32);
                }
                cur_r += 1;
            }
            while cur_l < q.lx {
                for &b in &by_x[&cur_l] {
                    ms.remove(&(b as i32));
                }
                cur_l += 1;
            }
            while cur_r > q.rx + 1 {
                cur_r -= 1;
                for &b in &by_x[&(cur_r - 1)] {
                    ms.remove(&(b as i32));
                }
            }

            if let Some(&val) = ms.range(q.ly..).next() {
                if val <= q.ry {
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
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace();
        x_coord.push(nums.next().unwrap().parse().unwrap());
        y_coord.push(nums.next().unwrap().parse().unwrap());
    }

    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    println!("{}", max_area);

    Ok(())
}