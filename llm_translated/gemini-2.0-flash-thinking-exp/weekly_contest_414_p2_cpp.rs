// Problem: Weekly Contest 414 Problem 2
use std::io;
use std::vec::Vec;

struct Solution {}

impl Solution {
    pub fn max_possible_score(&self, start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort(); // Sort the input vector 'start' in ascending order

        // Define a closure 'check' that takes a 'score' and returns a boolean
        let check = |score: i32| -> bool {
            let mut x: i64 = i64::min_value(); // Initialize 'x' to the smallest possible i64
            for &s in start.iter() {
                x = std::cmp::max(x + score as i64, s as i64); // Update 'x' to the maximum of 'x + score' and 's'
                if x > (s as i64) + (d as i64) {
                    return false; // If 'x' exceeds 's + d', return false
                }
            }
            true // If the loop completes without returning false, return true
        };

        let mut left: i32 = 0;
        // Calculate the initial right bound for binary search.
        // This is based on the maximum possible score which can be roughly estimated.
        let right: i32 = (start.last().unwrap_or(&0) + d - start.first().unwrap_or(&0)) / (if start.len() > 1 { (start.len() - 1) as i32 } else { 1 }) + 2;

        let mut right_bound = right; // Use a separate variable to avoid borrowing issues in loop

        while left + 1 < right_bound {
            let mid: i32 = left + (right_bound - left) / 2; // Calculate the middle value
            if check(mid) {
                left = mid; // If 'check(mid)' is true, update 'left' to 'mid'
            } else {
                right_bound = mid; // If 'check(mid)' is false, update 'right' to 'mid'
            }
        }
        left // Return the final 'left' value, which is the maximum possible score
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut iter = input_line.trim().split_whitespace();

    let start_size: usize = iter.next().unwrap().parse().expect("Invalid input for startSize");
    let d: i32 = iter.next().unwrap().parse().expect("Invalid input for d");

    let mut start: Vec<i32> = Vec::with_capacity(start_size);
    for _i in 0..start_size {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read line");
        let s: i32 = input_line.trim().parse().expect("Invalid input for start element");
        start.push(s);
    }

    let sol = Solution {};
    println!("{}", sol.max_possible_score(&mut start, d)); // Call max_possible_score and print the result
}