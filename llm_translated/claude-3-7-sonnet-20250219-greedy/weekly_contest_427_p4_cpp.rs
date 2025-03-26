use std::collections::{BTreeSet, BTreeMap, BinaryHeap};
use std::cmp::Reverse;
use std::io::{self, BufRead};

// tweak for performance
const BLOCK_SIZE: i32 = 512;

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

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_block = self.lx / BLOCK_SIZE;
        let other_block = other.lx / BLOCK_SIZE;
        
        (self_block, self.rx).cmp(&(other_block, other.rx))
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx && self.rx == other.rx && self.ly == other.ly && self.ry == other.ry
    }
}

impl Eq for Query {}

struct Solution;

impl Solution {
    fn max_rectangle_area(x_coord: &mut Vec<i32>, y_coord: &mut Vec<i32>) -> i64 {
        let mut ans: i64 = -1;
        let n = x_coord.len();
        
        // Coordinate compression
        let mut to_compressed: BTreeMap<i32, i32> = BTreeMap::new();
        let mut to_original: BTreeMap<i32, i32> = BTreeMap::new();
        
        {
            let mut p = BTreeSet::new();
            for &x in x_coord.iter() {
                p.insert(x);
            }
            for &y in y_coord.iter() {
                p.insert(y);
            }
            
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
        
        let mut by_x: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        let mut by_y: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        
        for i in 0..n {
            by_x.entry(x_coord[i]).or_insert_with(Vec::new).push(y_coord[i]);
            by_y.entry(y_coord[i]).or_insert_with(Vec::new).push(x_coord[i]);
        }
        
        for (_, v) in by_x.iter_mut() {
            v.sort();
        }
        for (_, v) in by_y.iter_mut() {
            v.sort();
        }
        
        let mut queries = Vec::new();
        
        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];
            
            let y_points = by_y.get(&ay).unwrap();
            let pos = y_points.binary_search(&ax).unwrap_or_else(|p| p);
            if pos >= y_points.len() {
                continue;
            }
            let rx = y_points[pos];
            if rx <= ax {
                continue;
            }
            let ry = ay; // this is bottom right point
            
            let x_points = by_x.get(&ax).unwrap();
            let pos = x_points.binary_search(&ay).unwrap_or_else(|p| p);
            if pos >= x_points.len() {
                continue;
            }
            let ty = x_points[pos];
            if ty <= ay {
                continue;
            }
            let tx = ax; // this is top left point
            
            // check existence of top right point
            let rx_points = by_x.get(&rx).unwrap();
            let pos_r = rx_points.binary_search(&ry).unwrap_or_else(|p| p);
            if pos_r >= rx_points.len() || rx_points[pos_r] != ty {
                continue;
            }
            
            let ty_points = by_y.get(&ty).unwrap();
            let pos_t = ty_points.binary_search(&tx).unwrap_or_else(|p| p);
            if pos_t >= ty_points.len() || ty_points[pos_t] != rx {
                continue;
            }
            
            // dx/dy = down x/y,
            // ux/uy = up x/y
            // we +1 and -1 so we only get the inside (excluding border)
            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;
            let area = (to_original[&ty] - to_original[&ay]) as i64 * (to_original[&rx] - to_original[&ax]) as i64;
            
            if dx <= ux && dy <= uy {
                // check if any point is in [dx, dy] [ux, uy] rectangle
                // if yes, then this isn't valid
                queries.push(Query::new(dx, ux, dy, uy, area));
            } else {
                ans = ans.max(area);
            }
        }
        
        // Sort queries for Mo's algorithm
        queries.sort();
        
        // Mo's algorithm at play
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms = BTreeSet::new();
        
        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                if let Some(points) = by_x.get(&cur_l) {
                    for &y in points {
                        ms.insert(y);
                    }
                }
            }
            while cur_r < q.rx {
                cur_r += 1;
                if let Some(points) = by_x.get(&cur_r) {
                    for &y in points {
                        ms.insert(y);
                    }
                }
            }
            while cur_l < q.lx {
                if let Some(points) = by_x.get(&cur_l) {
                    for &y in points {
                        ms.remove(&y);
                    }
                }
                cur_l += 1;
            }
            while cur_r > q.rx {
                if let Some(points) = by_x.get(&cur_r) {
                    for &y in points {
                        ms.remove(&y);
                    }
                }
                cur_r -= 1;
            }
            
            if let Some(&y) = ms.range(q.ly..).next() {
                if y <= q.ry {
                    continue; // there is a point that lies inside
                }
            }
            ans = ans.max(q.area);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);
    
    // Input the coordinates of the points
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        x_coord.push(coords[0]);
        y_coord.push(coords[1]);
    }
    
    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(&mut x_coord, &mut y_coord);
    
    // Output the result
    println!("{}", max_area);
}