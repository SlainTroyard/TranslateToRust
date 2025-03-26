/// Problem: Weekly Contest 436 Problem 4
/// 
/// This Rust program replicates the logic of the original C++ solution exactly,
/// including the same input/output format. It reads:
/// 1) A line containing two integers: n (size of the array) and m (some integer parameter).
/// 2) A line containing n integers (the array "points").
/// Then it prints the result of the algorithm.
/// 
/// The logic is preserved from the original C++ code but follows Rust idioms
/// (e.g., using closures, safer integer types, and standard Rust I/O).

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Returns the maximum score based on the given points array and parameter m.
    /// 
    /// The core logic is a binary search on the "low" value, with a check function
    /// that verifies feasibility under the constraints defined in the original C++
    /// solution. This is a direct translation of the original approach.
    fn max_score(&self, points: &[i32], m: i64) -> i64 {
        // Closure that checks if a given "low" value is feasible
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;  // Tracks the remaining count we can use (analogous to rem in C++)
            let mut pre = 0;  // Tracks how much we used in the previous step

            for (i, &p) in points.iter().enumerate() {
                // Calculating "k" similarly to how the C++ code does:
                // k = ((low - 1) / points[i]) + 1 - pre
                let mut k = ((low - 1) / p as i64) + 1 - pre as i64;

                // If we're at the last element and k <= 0, break out as in C++.
                if i == n - 1 && k <= 0 {
                    break;
                }

                // k must be at least 1
                if k < 1 {
                    k = 1;
                }

                // Update the "rem" (the leftover capacity) by subtracting (k*2 - 1)
                rem -= k * 2 - 1;

                // If we've exceeded our limit, it's not feasible
                if rem < 0 {
                    return false;
                }

                // Update pre to (k - 1), same as C++
                pre = k as i32 - 1;
            }
            true
        };

        // Binary search boundaries
        let mut left = 0i64;
        // Compute "right" similarly to the C++ code: ( (m+1)/2 ) * min(points) + 1
        let min_val = *points.iter().min().expect("points array should not be empty") as i64;
        let mut right = ((m + 1) / 2) * min_val + 1;

        // Standard binary search loop
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

fn main() -> io::Result<()> {
    // Read a line containing n and m
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let n: usize = parts[0].parse().expect("Failed to parse n");
    let m: i64 = parts[1].parse().expect("Failed to parse m");

    // Read a line containing n integers for "points"
    let mut points_line = String::new();
    stdin.lock().read_line(&mut points_line)?;
    let points_strs: Vec<&str> = points_line.split_whitespace().collect();

    // Parse points into a vector of i32
    let mut points = Vec::with_capacity(n);
    for &num_str in points_strs.iter().take(n) {
        points.push(num_str.parse::<i32>().expect("Failed to parse point"));
    }

    // Solve using the translated solution
    let solution = Solution;
    let answer = solution.max_score(&points, m);

    // Print the result to stdout
    println!("{}", answer);

    Ok(())
}