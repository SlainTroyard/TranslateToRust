use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufRead, Write};

// Block size used for Mo's algorithm, same as C++ code.
const BLOCK_SIZE: i32 = 512;

// Structure to represent a query (rectangle interior to check emptiness).
#[derive(Debug)]
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

// Implement ordering for Query according to (lx / BLOCK_SIZE, rx).
impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx && self.rx == other.rx && self.ly == other.ly && self.ry == other.ry && self.area == other.area
    }
}

impl Eq for Query {}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_key = (self.lx / BLOCK_SIZE, self.rx);
        let other_key = (other.lx / BLOCK_SIZE, other.rx);
        self_key.cmp(&other_key)
    }
}

// A simple multiset implemented using BTreeMap, where keys are i32 values and value is the count.
#[derive(Debug)]
struct MultiSet {
    map: BTreeMap<i32, usize>,
}

impl MultiSet {
    fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    // Insert an element into the multiset.
    fn insert(&mut self, key: i32) {
        *self.map.entry(key).or_insert(0) += 1;
    }

    // Remove one instance of the element from the multiset.
    fn remove(&mut self, key: i32) {
        if let Some(count) = self.map.get_mut(&key) {
            *count -= 1;
            if *count == 0 {
                self.map.remove(&key);
            }
        }
    }

    // Check if there's any element in the range [low, high].
    fn range_exists(&self, low: i32, high: i32) -> bool {
        // BTreeMap's range is inclusive of low and high.
        self.map.range(low..=high).next().is_some()
    }
}

// Helper function: given a sorted slice, returns the first element strictly greater than val (upper_bound).
fn upper_bound(slice: &[i32], val: i32) -> Option<i32> {
    // Since the slice is sorted in ascending order, we can iterate and find the first element > val.
    // (Could use binary_search_by but for clarity we use iteration.)
    for &x in slice {
        if x > val {
            return Some(x);
        }
    }
    None
}

// Main solution function.
fn max_rectangle_area(x_coord: &mut Vec<i32>, y_coord: &mut Vec<i32>) -> i64 {
    let mut ans: i64 = -1;
    let n = x_coord.len();

    // Coordinate compression:
    // Collect all unique coordinates from both x and y.
    let mut coords_set = std::collections::BTreeSet::new();
    for &x in x_coord.iter() {
        coords_set.insert(x);
    }
    for &y in y_coord.iter() {
        coords_set.insert(y);
    }

    // Create mapping from original coordinate to compressed coordinate,
    // and also store the original value for each compressed coordinate.
    let mut to_compressed: HashMap<i32, i32> = HashMap::new();
    let mut to_original: HashMap<i32, i32> = HashMap::new();
    let mut lst = 0;
    for &value in coords_set.iter() {
        to_compressed.insert(value, lst);
        to_original.insert(lst, value);
        lst += 1;
    }

    // Update the input coordinates with compressed values.
    for x in x_coord.iter_mut() {
        *x = *to_compressed.get(x).unwrap();
    }
    for y in y_coord.iter_mut() {
        *y = *to_compressed.get(y).unwrap();
    }

    // Build mappings: byX maps x-coordinate to vector of y's at that x,
    // and byY maps y-coordinate to vector of x's at that y.
    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..n {
        by_x.entry(x_coord[i]).or_default().push(y_coord[i]);
        by_y.entry(y_coord[i]).or_default().push(x_coord[i]);
    }
    // Sort each vector in by_x and by_y.
    for vec in by_x.values_mut() {
        vec.sort();
    }
    for vec in by_y.values_mut() {
        vec.sort();
    }

    // Collect queries for Mo's algorithm.
    let mut queries: Vec<Query> = Vec::new();

    // For each point, try to form a rectangle.
    for i in 0..n {
        let ax = x_coord[i];
        let ay = y_coord[i];

        // Find bottom-right point: first x in by_y[ay] that is strictly greater than ax.
        if let Some(by_y_vec) = by_y.get(&ay) {
            if let Some(&rx) = by_y_vec.iter().find(|&&v| v > ax) {
                let ry = ay; // bottom-right y is the same as current ay

                // Find top-left point: first y in by_x[ax] that is strictly greater than ay.
                if let Some(by_x_vec) = by_x.get(&ax) {
                    if let Some(&ty) = by_x_vec.iter().find(|&&v| v > ay) {
                        let tx = ax; // top-left x is same as current ax

                        // Check the existence of the top-right point.
                        if let Some(vec_rx) = by_x.get(&rx) {
                            if let Some(&candidate_ty) = upper_bound(vec_rx, ry).as_ref() {
                                if candidate_ty != ty {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                        if let Some(vec_ty) = by_y.get(&ty) {
                            if let Some(&candidate_rx) = upper_bound(vec_ty, tx).as_ref() {
                                if candidate_rx != rx {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }

                        // Compute interior boundaries (excluding the border)
                        let dx = ax + 1;
                        let dy = ay + 1;
                        let ux = rx - 1;
                        let uy = ty - 1;
                        // Calculate the rectangle area using original coordinates.
                        let original_ty = *to_original.get(&ty).unwrap();
                        let original_ay = *to_original.get(&ay).unwrap();
                        let original_rx = *to_original.get(&rx).unwrap();
                        let original_ax = *to_original.get(&ax).unwrap();
                        let area = 1i64 * (original_ty - original_ay) as i64 * (original_rx - original_ax) as i64;

                        if dx <= ux && dy <= uy {
                            // If interior is non-empty, add a query to check if any point lies in the interior.
                            queries.push(Query::new(dx, ux, dy, uy, area));
                        } else {
                            // Otherwise, update answer directly.
                            ans = ans.max(area);
                        }
                    }
                }
            }
        }
    }

    // Sort queries according to Mo's algorithm order.
    queries.sort();

    // Mo's algorithm: we will process queries by manipulating a sliding window over x-coordinate indices.
    // cur_l and cur_r define the current x-coordinate range (inclusive) we have in our multiset.
    let mut cur_l: i32 = 0;
    let mut cur_r: i32 = -1;
    let mut ms = MultiSet::new();

    // Process each query.
    for q in queries {
        // Expand the window to the left if needed.
        while cur_l > q.lx {
            cur_l -= 1;
            // Add all points that occur at x-coordinate cur_l.
            if let Some(vec_y) = by_x.get(&cur_l) {
                for &val in vec_y.iter() {
                    ms.insert(val);
                }
            }
        }
        // Expand the window to the right if needed.
        while cur_r < q.rx {
            cur_r += 1;
            if let Some(vec_y) = by_x.get(&cur_r) {
                for &val in vec_y.iter() {
                    ms.insert(val);
                }
            }
        }
        // Shrink the window from the left if needed.
        while cur_l < q.lx {
            if let Some(vec_y) = by_x.get(&cur_l) {
                for &val in vec_y.iter() {
                    ms.remove(val);
                }
            }
            cur_l += 1;
        }
        // Shrink the window from the right if needed.
        while cur_r > q.rx {
            if let Some(vec_y) = by_x.get(&cur_r) {
                for &val in vec_y.iter() {
                    ms.remove(val);
                }
            }
            cur_r -= 1;
        }
        // Check if there is any point in the interior rectangle [q.ly, q.ry].
        if ms.range_exists(q.ly, q.ry) {
            // If yes, then this rectangle is invalid.
            continue;
        }
        ans = ans.max(q.area);
    }

    ans
}

fn main() -> io::Result<()> {
    // Use stdin and stdout locks for efficient I/O
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read first line: the number of points.
    reader.read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Failed to parse n");
    input.clear();

    // Prepare vectors for x and y coordinates.
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    // Read n lines of points. The points may be provided on separate lines or multiple per line.
    // We will split the input by whitespace until we have read all 2*n numbers.
    let mut numbers: Vec<i32> = Vec::with_capacity(2 * n);
    while numbers.len() < 2 * n {
        input.clear();
        // Read a line, if no more lines, break.
        if reader.read_line(&mut input)? == 0 {
            break;
        }
        for token in input.split_whitespace() {
            numbers.push(token.parse::<i32>().expect("Failed to parse coordinate"));
        }
    }
    // Populate x_coord and y_coord.
    for i in 0..n {
        x_coord.push(numbers[2 * i]);
        y_coord.push(numbers[2 * i + 1]);
    }

    // Calculate the maximum rectangle area using our solution.
    let max_area = max_rectangle_area(&mut x_coord, &mut y_coord);

    // Output the result.
    writeln!(stdout, "{}", max_area)?;
    Ok(())
}