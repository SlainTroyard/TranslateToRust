use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        // Sort the input vector
        start.sort();

        // Closure to check if a given score is valid
        let check = |score: i32| -> bool {
            let mut x: i64 = i64::MIN; // Initialize x to the smallest possible value
            for &s in start.iter() {
                x = max(x + score as i64, s as i64); // Update x based on the current score
                if x > s as i64 + d as i64 {
                    return false; // If x exceeds the allowed range, return false
                }
            }
            true // If all checks pass, return true
        };

        // Binary search to find the maximum possible score
        let mut left = 0;
        let mut right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid; // If the score is valid, move the left boundary
            } else {
                right = mid; // Otherwise, move the right boundary
            }
        }
        left // Return the maximum possible score
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let start_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let d: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Parse the second line for the start vector
    let second_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(start.len(), start_size);

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.max_possible_score(&mut start, d);

    // Print the result to stdout
    println!("{}", result);
}