use std::collections::{BTreeSet, HashMap, HashSet};
use std::cmp::max;
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 512;

#[derive(Debug, Clone)]
struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

impl Query {
    fn new(lx: i32, rx: i32, ly: i32, ry: i32, area: i64) -> Self {
        Query { lx, rx, ly, ry, area }
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx && self.rx == other.rx
    }
}

impl Eq for Query {}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ((self.lx / BLOCK_SIZE as i32), self.rx).cmp(&((other.lx / BLOCK_SIZE as i32), other.rx))
    }
}

fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
    let mut ans = -1;
    let n = x_coord.len();

    // Coordinate compression
    let mut to_compressed = HashMap::new();
    let mut to_original = HashMap::new();
    let mut unique_coords: BTreeSet<i32> = x_coord.iter().cloned().collect();
    unique_coords.extend(y_coord.iter().cloned());
    let mut lst = 0;
    for &coord in &unique_coords {
        to_compressed.insert(coord, lst);
        to_original.insert(lst, coord);
        lst += 1;
    }

    let mut x_coord_compressed: Vec<i32> = x_coord.iter().map(|&x| *to_compressed.get(&x).unwrap()).collect();
    let mut y_coord_compressed: Vec<i32> = y_coord.iter().map(|&y| *to_compressed.get(&y).unwrap()).collect();

    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in 0..n {
        by_x.entry(x_coord_compressed[i]).or_insert_with(Vec::new).push(y_coord_compressed[i]);
        by_y.entry(y_coord_compressed[i]).or_insert_with(Vec::new).push(x_coord_compressed[i]);
    }

    for (_, v) in by_x.iter_mut() {
        v.sort();
    }

    for (_, v) in by_y.iter_mut() {
        v.sort();
    }

    let mut queries = Vec::new();

    for i in 0..n {
        let ax = x_coord_compressed[i];
        let ay = y_coord_compressed[i];

        let rx = match by_y[&ay].iter().find(|&&x| x > ax) {
            Some(&x) => x,
            None => continue,
        };
        let ry = ay;

        let ty = match by_x[&ax].iter().find(|&&y| y > ay) {
            Some(&y) => y,
            None => continue,
        };
        let tx = ax;

        let tr_y = match by_x[&rx].iter().find(|&&y| y > ry) {
            Some(&y) => y,
            None => continue,
        };

        let tr_x = match by_y[&ty].iter().find(|&&x| x > tx) {
            Some(&x) => x,
            None => continue,
        };

        if tr_y != ty || tr_x != rx {
            continue;
        }

        let dx = ax + 1;
        let dy = ay + 1;
        let ux = rx - 1;
        let uy = ty - 1;

        let area = (to_original[&ty] - to_original[&ay]) as i64 * (to_original[&rx] - to_original[&ax]) as i64;

        if dx <= ux && dy <= uy {
            queries.push(Query::new(dx, ux, dy, uy, area));
        } else {
            ans = max(ans, area);
        }
    }

    queries.sort();

    let mut cur_l = 0;
    let mut cur_r = -1;
    let mut ms = HashSet::new();

    for q in queries {
        while cur_l > q.lx {
            cur_l -= 1;
            for &y in &by_x[&cur_l] {
                ms.insert(y);
            }
        }
        while cur_r < q.rx {
            cur_r += 1;
            for &y in &by_x[&cur_r] {
                ms.insert(y);
            }
        }
        while cur_l < q.lx {
            for &y in &by_x[&cur_l] {
                ms.remove(&y);
            }
            cur_l += 1;
        }
        while cur_r > q.rx {
            for &y in &by_x[&cur_r] {
                ms.remove(&y);
            }
            cur_r -= 1;
        }

        if ms.iter().any(|&y| y >= q.ly && y <= q.ry) {
            continue;
        }

        ans = max(ans, q.area);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace();
        x_coord.push(coords.next().unwrap().parse().unwrap());
        y_coord.push(coords.next().unwrap().parse().unwrap());
    }

    let max_area = max_rectangle_area(x_coord, y_coord);

    println!("{}", max_area);
}