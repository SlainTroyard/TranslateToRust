use std::cmp;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(points: Vec<i64>, m: i64) -> i64 {
        // Closure for checking if a given `low` is feasible
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;

            for i in 0..n {
                let mut k = ((low - 1) / points[i]) + 1 - pre;
                if i == n - 1 && k <= 0 {
                    break;
                }
                k = cmp::max(k, 1);
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        // Binary search range
        let mut left = 0i64;
        let mut right = (m + 1) / 2 * points.iter().min().unwrap() + 1;

        // Binary search for the maximum feasible `low`
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
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line containing `n` and `m`
    let first_line = lines.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: i64 = split.next().unwrap().parse().unwrap();

    // Parse the second line containing the array `points`
    let second_line = lines.next().unwrap().unwrap();
    let points: Vec<i64> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure input validity (optional, but helpful for debugging)
    assert_eq!(points.len(), n);

    // Solve the problem
    let solution = Solution;
    let result = solution.max_score(points, m);

    // Print the result to stdout
    println!("{}", result);
}