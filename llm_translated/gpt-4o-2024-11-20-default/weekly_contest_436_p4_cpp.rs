use std::io::{self, BufRead};
use std::cmp;

// Solution struct simulating the encapsulation in the C++ code
struct Solution;

impl Solution {
    // Method mimicking the maxScore function in C++
    fn max_score(points: Vec<i32>, m: i64) -> i64 {
        // Closure to simulate the check function in the C++ code
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for (i, &point) in points.iter().enumerate() {
                let k = (low - 1) / point as i64 + 1 - pre;
                let k = if i == n - 1 && k <= 0 { break; } else { cmp::max(k, 1) };
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        // Binary search logic for determining max score
        let mut left = 0;
        let mut right = (m + 1) / 2 * *points.iter().min().unwrap() as i64 + 1;
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

fn main() {
    // Input/output handling with the same format as the C++ code
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for `n` (size of points) and `m`
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<i64> = first_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    let n = nums[0] as usize;
    let m = nums[1];

    // Read the second line for the `points` array
    let second_line = lines.next().unwrap().unwrap();
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    assert_eq!(points.len(), n, "points array size does not match n");

    // Compute result using the Solution struct
    let solution = Solution;
    let result = solution.max_score(points, m);

    // Print the result
    println!("{}", result);
}