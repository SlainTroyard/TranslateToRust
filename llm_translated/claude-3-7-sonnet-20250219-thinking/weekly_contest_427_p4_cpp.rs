use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::cmp::{self, Ordering};

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
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Query {}

// Custom MultiSet implementation to match C++ multiset behavior
struct MultiSet {
    data: BTreeMap<i32, usize>,
}

impl MultiSet {
    fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    fn insert(&mut self, value: i32) {
        *self.data.entry(value).or_insert(0) += 1;
    }

    fn insert_all(&mut self, values: &[i32]) {
        for &val in values {
            self.insert(val);
        }
    }

    fn erase_one(&mut self, value: i32) {
        if let Some(count) = self.data.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.data.remove(&value);
            }
        }
    }

    fn lower_bound(&self, value: i32) -> Option<i32> {
        self.data.range(value..).next().map(|(&k, _)| k)
    }
}

struct Solution;

impl Solution {
    pub fn max_rectangle_area(mut x_coord: Vec<i32>, mut y_coord: Vec<i32>) -> i64 {
        let mut ans: i64 = -1;
        let n = x_coord.len();
        
        // Coordinate compression
        let mut to_compressed: HashMap<i32, i32> = HashMap::new();
        let mut to_original: HashMap<i32, i32> = HashMap::new();
        
        {
            let mut p = HashSet::new();
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
            
            for x in &mut x_coord {
                *x = *to_compressed.get(x).unwrap();
            }
            for y in &mut y_coord {
                *y = *to_compressed.get(y).unwrap();
            }
        }
        
        let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();
        
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
            
            // Find the next x-coordinate on the same y-level (to the right)
            let y_points = by_y.get(&ay).unwrap();
            let it_y = y_points.iter().position(|&x| x > ax);
            if it_y.is_none() {
                continue;
            }
            let rx = y_points[it_y.unwrap()];
            let ry = ay; // this is bottom right point
            
            // Find the next y-coordinate on the same x-level (above)
            let x_points = by_x.get(&ax).unwrap();
            let it_x = x_points.iter().position(|&y| y > ay);
            if it_x.is_none() {
                continue;
            }
            let tx = ax;
            let ty = x_points[it_x.unwrap()]; // this is top left point
            
            // check existence of top right point
            let rx_points = by_x.get(&rx).unwrap();
            let it_r = rx_points.iter().position(|&y| y > ry);
            
            let ty_points = by_y.get(&ty).unwrap();
            let it_t = ty_points.iter().position(|&x| x > tx);
            
            if it_r.is_none() || it_t.is_none() {
                continue;
            }
            
            if rx_points[it_r.unwrap()] != ty || ty_points[it_t.unwrap()] != rx {
                continue;
            }
            
            // dx/dy = down x/y,
            // ux/uy = up x/y
            // we +1 and -1 so we only get the inside (excluding border)
            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;
            
            let area = (*to_original.get(&ty).unwrap() - *to_original.get(&ay).unwrap()) as i64 *
                      (*to_original.get(&rx).unwrap() - *to_original.get(&ax).unwrap()) as i64;
            
            if dx <= ux && dy <= uy {
                // check if any point is in [dx, dy] [ux, uy] rectangle
                // if yes, then this isn't valid
                queries.push(Query::new(dx, ux, dy, uy, area));
            } else {
                ans = cmp::max(ans, area);
            }
        }
        
        queries.sort();
        
        // Mo's algorithm at play
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms = MultiSet::new();
        
        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                if let Some(points) = by_x.get(&cur_l) {
                    ms.insert_all(points);
                }
            }
            
            while cur_r < q.rx {
                cur_r += 1;
                if let Some(points) = by_x.get(&cur_r) {
                    ms.insert_all(points);
                }
            }
            
            while cur_l < q.lx {
                if let Some(points) = by_x.get(&cur_l) {
                    for &b in points {
                        ms.erase_one(b);
                    }
                }
                cur_l += 1;
            }
            
            while cur_r > q.rx {
                if let Some(points) = by_x.get(&cur_r) {
                    for &b in points {
                        ms.erase_one(b);
                    }
                }
                cur_r -= 1;
            }
            
            if let Some(y) = ms.lower_bound(q.ly) {
                if y <= q.ry {
                    continue; // there is a point that lies inside
                }
            }
            
            ans = cmp::max(ans, q.area);
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the number of points
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);
    
    // Input the coordinates of the points
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        x_coord.push(coords[0]);
        y_coord.push(coords[1]);
    }
    
    // Calculate the maximum rectangle area
    let max_area = Solution::max_rectangle_area(x_coord, y_coord);
    
    // Output the result
    println!("{}", max_area);
    
    Ok(())
}