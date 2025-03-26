use std::collections::{BTreeSet, HashMap, HashSet};
use std::cmp::max;
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 512;

#[derive(Debug)]
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

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx / BLOCK_SIZE == other.lx / BLOCK_SIZE && self.rx == other.rx
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
        (self.lx / BLOCK_SIZE, self.rx).cmp(&(other.lx / BLOCK_SIZE, other.rx))
    }
}

fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
    let mut ans = -1;
    let n = x_coord.len();

    // Coordinate compression
    let mut to_compressed = HashMap::new();
    let mut to_original = HashMap::new();
    {
        let mut points = BTreeSet::new();
        points.extend(x_coord.iter().cloned());
        points.extend(y_coord.iter().cloned());
        let mut lst = 0;
        for &a in &points {
            to_compressed.insert(a, lst);
            to_original.insert(lst, a);
            lst += 1;
        }
    }

    let mut x_coord = x_coord.into_iter().map(|x| *to_compressed.get(&x).unwrap()).collect::<Vec<_>>();
    let mut y_coord = y_coord.into_iter().map(|y| *to_compressed.get(&y).unwrap()).collect::<Vec<_>>();

    let mut by_x: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut by_y: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..n {
        by_x.entry(x_coord[i]).or_insert_with(Vec::new).push(y_coord[i]);
        by_y.entry(y_coord[i]).or_insert_with(Vec::new).push(x_coord[i]);
    }

    for v in by_x.values_mut() {
        v.sort();
    }
    for v in by_y.values_mut() {
        v.sort();
    }

    let mut queries = Vec::new();
    for i in 0..n {
        let ax = x_coord[i];
        let ay = y_coord[i];

        let it = by_y[&ay].iter().find(|&&x| x > ax);
        if it.is_none() {
            continue;
        }
        let rx = *it.unwrap();
        let ry = ay;

        let it = by_x[&ax].iter().find(|&&y| y > ay);
        if it.is_none() {
            continue;
        }
        let tx = ax;
        let ty = *it.unwrap();

        let it_r = by_x[&rx].iter().find(|&&y| y > ry);
        let it_t = by_y[&ty].iter().find(|&&x| x > tx);
        if it_r.is_none() || it_t.is_none() {
            continue;
        }
        if *it_r.unwrap() != ty || *it_t.unwrap() != rx {
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
    let mut cur_r = 0;
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
        let mut iter = line.split_whitespace();
        x_coord.push(iter.next().unwrap().parse().unwrap());
        y_coord.push(iter.next().unwrap().parse().unwrap());
    }

    let max_area = max_rectangle_area(x_coord, y_coord);
    println!("{}", max_area);
}