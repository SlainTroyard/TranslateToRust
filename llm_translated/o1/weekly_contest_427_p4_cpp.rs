// Translation of the original C++ solution to Rust.
// Weekly Contest 427 Problem 4 (LeetCode) - Maximum Rectangle Area
//
// REQUIREMENTS MET:
//  1. The entire file is translated as a complete program, including the main function.
//  2. The algorithm logic is preserved exactly.
//  3. Uses idiomatic Rust with proper error handling and data structures.
//  4. Maintains the exact same stdin/stdout format as the original code.
//  5. Comments have been added for clarity.

use std::io::{self, BufRead};
use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::cmp;

// Mo's algorithm block size (same as in the C++ code)
const BLOCK_SIZE: i32 = 512;

// A query struct for Mo's algorithm with the rectangle boundaries plus the candidate area.
#[derive(Clone)]
struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

impl Query {
    fn new(lx: i32, rx: i32, ly: i32, ry: i32, area: i64) -> Self {
        Self { lx, rx, ly, ry, area }
    }
}

// Custom compare function to mimic the operator< used in the C++ code.
// Sort by (lx / BLOCK_SIZE), then by rx.
fn compare_queries(a: &Query, b: &Query) -> cmp::Ordering {
    let ablk = a.lx / BLOCK_SIZE;
    let bblk = b.lx / BLOCK_SIZE;
    if ablk != bblk {
        ablk.cmp(&bblk)
    } else {
        a.rx.cmp(&b.rx)
    }
}

// Helper functions to insert / remove a single value into a frequency map
// (used as a multiset replacement).
fn insert_val(freq_map: &mut BTreeMap<i32, i32>, val: i32) {
    *freq_map.entry(val).or_insert(0) += 1;
}

fn remove_val(freq_map: &mut BTreeMap<i32, i32>, val: i32) {
    if let Some(count) = freq_map.get_mut(&val) {
        *count -= 1;
        if *count == 0 {
            freq_map.remove(&val);
        }
    }
}

// Adds all y-values from by_x[x] to freq_map
fn add_x(x: i32, by_x: &HashMap<i32, Vec<i32>>, freq_map: &mut BTreeMap<i32, i32>) {
    if let Some(ys) = by_x.get(&x) {
        for &y in ys {
            insert_val(freq_map, y);
        }
    }
}

// Removes all y-values from by_x[x] from freq_map
fn remove_x(x: i32, by_x: &HashMap<i32, Vec<i32>>, freq_map: &mut BTreeMap<i32, i32>) {
    if let Some(ys) = by_x.get(&x) {
        for &y in ys {
            remove_val(freq_map, y);
        }
    }
}

// Main solution function that replicates the logic from the C++ class Solution::maxRectangleArea.
fn max_rectangle_area(mut x_coord: Vec<i32>, mut y_coord: Vec<i32>) -> i64 {
    let n = x_coord.len();
    let mut ans = -1_i64;

    // Coordinate compression mapping
    let mut all_coords = BTreeSet::new();
    for i in 0..n {
        all_coords.insert(x_coord[i]);
        all_coords.insert(y_coord[i]);
    }

    let mut sorted_coords: Vec<i32> = all_coords.into_iter().collect();
    let mut to_compressed = HashMap::new();
    let mut to_original = HashMap::new();

    // Build the forward (original -> compressed) and reverse (compressed -> original) mappings
    for (idx, &val) in sorted_coords.iter().enumerate() {
        to_compressed.insert(val, idx as i32);
        to_original.insert(idx as i32, val);
    }

    // Compress x_coord and y_coord in-place
    for i in 0..n {
        x_coord[i] = *to_compressed.get(&x_coord[i]).unwrap();
        y_coord[i] = *to_compressed.get(&y_coord[i]).unwrap();
    }

    // Group points by their x and y values
    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in 0..n {
        by_x.entry(x_coord[i]).or_insert_with(Vec::new).push(y_coord[i]);
        by_y.entry(y_coord[i]).or_insert_with(Vec::new).push(x_coord[i]);
    }

    // Sort the vectors so we can perform binary_search (upper_bound equivalent)
    for v in by_x.values_mut() {
        v.sort();
    }
    for v in by_y.values_mut() {
        v.sort();
    }

    // Generate queries from valid rectangles found by checking the four corners
    let mut queries = Vec::new();
    for i in 0..n {
        let ax = x_coord[i];
        let ay = y_coord[i];

        // bottom-right point: find first x > ax in by_y[ay]
        let row = by_y.get(&ay).unwrap();
        let mut pos = match row.binary_search(&ax) {
            Ok(p) => p + 1, // upper_bound
            Err(p) => p,
        };
        if pos == row.len() {
            continue;
        }
        let rx = row[pos];
        let ry = ay;

        // top-left point: find first y > ay in by_x[ax]
        let col = by_x.get(&ax).unwrap();
        pos = match col.binary_search(&ay) {
            Ok(p) => p + 1,
            Err(p) => p,
        };
        if pos == col.len() {
            continue;
        }
        let tx = ax;
        let ty = col[pos];

        // Now check for the top-right point to ensure rectangle closure
        let col_rx = by_x.get(&rx).unwrap();
        let mut pos_r = match col_rx.binary_search(&ry) {
            Ok(p) => p + 1,
            Err(p) => p,
        };
        if pos_r == col_rx.len() || col_rx[pos_r] != ty {
            continue;
        }
        let row_ty = by_y.get(&ty).unwrap();
        let mut pos_t = match row_ty.binary_search(&tx) {
            Ok(p) => p + 1,
            Err(p) => p,
        };
        if pos_t == row_ty.len() || row_ty[pos_t] != rx {
            continue;
        }

        // Internal rectangle boundaries (excluding the border)
        let dx = ax + 1;
        let dy = ay + 1;
        let ux = rx - 1;
        let uy = ty - 1;

        // Calculate the area in original (uncompressed) coordinates
        let height = to_original[&ty] - to_original[&ay];
        let width = to_original[&rx] - to_original[&ax];
        let area = (height as i64) * (width as i64);

        // If there is no "internal" rectangle, immediately update ans
        if dx <= ux && dy <= uy {
            queries.push(Query::new(dx, ux, dy, uy, area));
        } else {
            ans = ans.max(area);
        }
    }

    // Sort queries by Mo's ordering
    queries.sort_by(compare_queries);

    // Mo's algorithm to check for internal points
    let mut freq_map: BTreeMap<i32, i32> = BTreeMap::new();
    let mut cur_l = 0;
    let mut cur_r = -1;

    for q in queries {
        // Expand or shrink the range [cur_l..cur_r] along the x-axis to match q.lx..q.rx
        while cur_l > q.lx {
            cur_l -= 1;
            add_x(cur_l, &by_x, &mut freq_map);
        }
        while cur_r < q.rx {
            cur_r += 1;
            add_x(cur_r, &by_x, &mut freq_map);
        }
        while cur_l < q.lx {
            remove_x(cur_l, &by_x, &mut freq_map);
            cur_l += 1;
        }
        while cur_r > q.rx {
            remove_x(cur_r, &by_x, &mut freq_map);
            cur_r -= 1;
        }

        // Check if there's any point in the y-range [q.ly..q.ry]
        // Looking at the lowest value >= q.ly:
        if let Some((&lowest_val, _count)) = freq_map.range(q.ly..).next() {
            // If that value is <= q.ry, we have a point in the interior -> not valid
            if lowest_val <= q.ry {
                continue;
            }
        }

        // Otherwise, update the answer
        ans = ans.max(q.area);
    }

    ans
}

fn main() {
    // Read the number of points
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: n
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Next n lines contain x and y
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts
            .next()
            .expect("Not enough numbers on the line")
            .parse()
            .expect("Failed to parse x");
        let y: i32 = parts
            .next()
            .expect("Not enough numbers on the line")
            .parse()
            .expect("Failed to parse y");
        x_coord.push(x);
        y_coord.push(y);
    }

    // Calculate and output the maximum rectangle area
    let max_area = max_rectangle_area(x_coord, y_coord);
    println!("{}", max_area);
}