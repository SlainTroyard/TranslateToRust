use std::io::{self, BufRead};

pub struct Solution;

impl Solution {
    pub fn max_distance(&self, side: i64, points: &mut Vec<[i64; 2]>, k: usize) -> i64 {
        let n = points.len();

        // Define the perimeter order using a closure.
        // Depending on which edge of the square the point is, compute its order.
        let ord = |x: i64, y: i64| -> i64 {
            if y == 0 {
                x
            } else if x == side {
                side + y
            } else if y == side {
                side * 3 - x
            } else {
                side * 4 - y
            }
        };

        // Sort the points using the computed order.
        points.sort_by(|a, b| ord(a[0], a[1]).cmp(&ord(b[0], b[1])));

        // Closure to calculate Manhattan distance between two points using their indices.
        // Note: We use indices mod n when needed, because later we simulate a doubled array.
        let dis = |i: usize, j: usize| -> i64 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        // Check if a cycle with at least `lim` as the minimum Manhattan distance
        // between consecutive points (including between the first and last) can be formed.
        let check = |lim: i64| -> bool {
            // First, try a simple greedy approach.
            // Start with the first point (index 0).
            let mut vec_idx: Vec<usize> = Vec::with_capacity(k);
            vec_idx.push(0);
            for i in 1..n {
                if vec_idx.len() < k && dis(i, *vec_idx.last().unwrap()) >= lim {
                    vec_idx.push(i);
                }
            }
            if vec_idx.len() < k {
                return false;
            }
            if dis(vec_idx[0], vec_idx[vec_idx.len() - 1]) >= lim {
                return true;
            }

            // Try adjusting the starting index.
            // We simulate a cycle by "duplicating" the points array using indices modulo n.
            let mut vec_idx = vec![0usize; k]; // Pre-allocate a vector with k elements.
            // The outer loop iterates over possible starting positions.
            for i in 1..n {
                vec_idx[0] = i;
                let mut valid = true;
                for j in 1..k {
                    // Initialize the candidate for the j-th point.
                    vec_idx[j] = vec_idx[j - 1] + 1;
                    // Increase candidate until the distance from the previous selected point is at least lim.
                    while vec_idx[j] < 2 * n && dis(vec_idx[j] % n, vec_idx[j - 1] % n) < lim {
                        vec_idx[j] += 1;
                    }
                    if vec_idx[j] >= 2 * n {
                        valid = false;
                        break;
                    }
                }
                // Check that the total cycle (the edge from the last back to the first)
                // also satisfies the condition and that the selected points lie in a valid range.
                if valid && vec_idx[k - 1] < i + n && dis(i, vec_idx[k - 1] % n) >= lim {
                    return true;
                }
            }
            false
        };

        // Binary search for the maximum distance between consecutive points on the cycle.
        let mut head = 1;
        let mut tail = side;
        while head < tail {
            let mid = (head + tail + 1) / 2;
            if check(mid) {
                head = mid;
            } else {
                tail = mid - 1;
            }
        }
        head
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a buffered reader to handle input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut tokens: Vec<String> = Vec::new();

    // Read input tokens from stdin.
    // The input format expects:
    // side n K
    // followed by n lines, each containing two integers for the point coordinates.
    while let Some(line) = lines.next() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // Parse the first three tokens: side, n, and K.
    let mut it = tokens.into_iter();
    let side: i64 = it.next().ok_or("Missing side")?.parse()?;
    let n: usize = it.next().ok_or("Missing n")?.parse()?;
    let k: usize = it.next().ok_or("Missing K")?.parse()?;

    // Parse the next n pairs of integers as points.
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().ok_or("Missing point x")?.parse()?;
        let y: i64 = it.next().ok_or("Missing point y")?.parse()?;
        points.push([x, y]);
    }

    // Instantiate the solution and compute the answer.
    let sol = Solution;
    let ans = sol.max_distance(side, &mut points, k);

    // Print the answer exactly as the original C++ code would.
    println!("{}", ans);

    Ok(())
}