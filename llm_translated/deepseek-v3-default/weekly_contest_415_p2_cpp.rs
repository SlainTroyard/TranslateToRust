use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the sizes of vectors a and b
    let sizes = lines.next().unwrap().unwrap();
    let mut sizes_iter = sizes.split_whitespace();
    let a_size: usize = sizes_iter.next().unwrap().parse().unwrap();
    let b_size: usize = sizes_iter.next().unwrap().parse().unwrap();

    // Read vector a
    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = a_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read vector b
    let b_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = b_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create a solution instance and compute the max score
    let solution = Solution;
    let result = solution.max_score(&a, &b);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn max_score(&self, a: &[i32], b: &[i32]) -> i64 {
        // Initialize the DP array with minimum values
        let mut f = [i64::MIN / 2; 5];
        f[0] = 0;

        // Iterate over each element in b
        for &y in b {
            // Update the DP array in reverse order to avoid overwriting
            for j in (0..4).rev() {
                f[j + 1] = max(f[j + 1], f[j] + a[j] as i64 * y as i64);
            }
        }

        // Return the last element of the DP array
        f[4]
    }
}