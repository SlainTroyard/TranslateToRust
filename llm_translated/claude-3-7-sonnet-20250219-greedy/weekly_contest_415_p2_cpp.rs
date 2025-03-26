use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        // Initialize an array to store dynamic programming states
        // f[j] represents the maximum score when using j elements from array a
        let mut f = [0i64, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2];
        
        for y in b {
            // Process each element in array b
            // We need to iterate backwards to avoid using the same element from a multiple times
            for j in (0..4).rev() {
                // Update the state: either keep the current value or use the current element
                f[j + 1] = cmp::max(f[j + 1], f[j] + (a[j] as i64) * (y as i64));
            }
        }
        
        // Return the maximum score when using all 4 elements from array a
        f[4]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read array sizes
    let sizes = lines.next().unwrap().unwrap();
    let mut iter = sizes.split_whitespace();
    let a_size: usize = iter.next().unwrap().parse().unwrap();
    let b_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read array a
    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = a_line.split_whitespace()
        .take(a_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read array b
    let b_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = b_line.split_whitespace()
        .take(b_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let solution = Solution;
    println!("{}", Solution::max_score(a, b));
}