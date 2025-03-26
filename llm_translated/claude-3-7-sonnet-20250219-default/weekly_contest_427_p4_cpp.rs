use std::cmp::{max, Ordering};
use std::collections::{BTreeSet, BTreeMap, HashMap};
use std::io::{self, BufRead};

// tweak for performance
const BLOCK_SIZE: i32 = 512;

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
    fn cmp(&self, other: &Self) -> Ordering {
        let self_block = self.lx / BLOCK_SIZE;
        let other_block = other.lx / BLOCK_SIZE;
        
        match self_block.cmp(&other_block) {
            Ordering::Equal => self.rx.cmp(&other.rx),
            other => other,
        }
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx && self.rx == other.rx
    }
}

impl Eq for Query {}

struct Solution;

impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        let mut ans: i64 = -1;
        let n = x_coord.len();
        
        // Coordinate compression
        let mut to_compressed: HashMap<i32, i32> = HashMap::new();
        let mut to_original: HashMap<i32, i32> = HashMap::new();
        
        {
            let mut p = BTreeSet::new();
            for &x in &x_coord {
                p.insert(x);
            }
            for &y in &y_coord {
                p.insert(y);
            }
            
            let mut lst = 0;
            for &a in &p {
                to_compressed.insert(a, lst);
                to_original.insert(lst, a);
                lst += 1;
            }
        }
        
        let mut x_coord_compressed = x_coord.clone();
        let mut y_coord_compressed = y_coord.clone();
        
        for i in 0..n {
            x_coord_compressed[i] = *to_compressed.get(&x_coord[i]).unwrap();
            y_coord_compressed[i] = *to_compressed.get(&y_coord[i]).unwrap();
        }
        
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
            
            // Find bottom right point
            let y_neighbors = by_y.get(&ay).unwrap();
            let pos_rx = y_neighbors.binary_search(&ax).map_or_else(|p| p, |p| p + 1);
            if pos_rx >= y_neighbors.len() {
                continue;
            }
            let rx = y_neighbors[pos_rx];
            let ry = ay;
            
            // Find top left point
            let x_neighbors = by_x.get(&ax).unwrap();
            let pos_ty = x_neighbors.binary_search(&ay).map_or_else(|p| p, |p| p + 1);
            if pos_ty >= x_neighbors.len() {
                continue;
            }
            let tx = ax;
            let ty = x_neighbors[pos_ty];
            
            // Check existence of top right point
            let rx_neighbors = by_x.get(&rx).unwrap();
            let pos_rx_ty = rx_neighbors.binary_search(&ry).map_or_else(|p| p, |p| p + 1);
            if pos_rx_ty >= rx_neighbors.len() || rx_neighbors[pos_rx_ty] != ty {
                continue;
            }
            
            let ty_neighbors = by_y.get(&ty).unwrap();
            let pos_ty_rx = ty_neighbors.binary_search(&tx).map_or_else(|p| p, |p| p + 1);
            if pos_ty_rx >= ty_neighbors.len() || ty_neighbors[pos_ty_rx] != rx {
                continue;
            }
            
            // dx/dy = down x/y,
            // ux/uy = up x/y
            // we +1 and -1 so we only get the inside (excluding border)
            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;
            
            let area = (to_original[&ty] - to_original[&ay]) as i64 * 
                       (to_original[&rx] - to_original[&ax]) as i64;
            
            if dx <= ux && dy <= uy {
                // Check if any point is in [dx, dy] [ux, uy] rectangle
                // If yes, then this isn't valid
                queries.push(Query::new(dx, ux, dy, uy, area));
            } else {
                ans = max(ans, area);
            }
        }
        
        queries.sort();
        
        // Mo's algorithm
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms = BTreeMap::<i32, i32>::new(); // Multiset simulation with BTreeMap
        
        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                // Add all y-coordinates of points with x=cur_l to the multiset
                if let Some(points) = by_x.get(&cur_l) {
                    for &y in points {
                        *ms.entry(y).or_insert(0) += 1;
                    }
                }
            }
            
            while cur_r < q.rx {
                cur_r += 1;
                // Add all y-coordinates of points with x=cur_r to the multiset
                if let Some(points) = by_x.get(&cur_r) {
                    for &y in points {
                        *ms.entry(y).or_insert(0) += 1;
                    }
                }
            }
            
            while cur_l < q.lx {
                // Remove all y-coordinates of points with x=cur_l from the multiset
                if let Some(points) = by_x.get(&cur_l) {
                    for &y in points {
                        let count = ms.get_mut(&y).unwrap();
                        *count -= 1;
                        if *count == 0 {
                            ms.remove(&y);
                        }
                    }
                }
                cur_l += 1;
            }
            
            while cur_r > q.rx {
                // Remove all y-coordinates of points with x=cur_r from the multiset
                if let Some(points) = by_x.get(&cur_r) {
                    for &y in points {
                        let count = ms.get_mut(&y).unwrap();
                        *count -= 1;
                        if *count == 0 {
                            ms.remove(&y);
                        }
                    }
                }
                cur_r -= 1;
            }
            
            // Check if there's a point inside the rectangle
            if let Some((&y, _)) = ms.range(q.ly..).next() {
                if y <= q.ry {
                    continue; // There is a point that lies inside
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
    
    // Input the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);
    
    // Input the coordinates of the points
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        x_coord.push(coords[0]);
        y_coord.push(coords[1]);
    }
    
    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(x_coord, y_coord);
    
    // Output the result
    println!("{}", max_area);
}