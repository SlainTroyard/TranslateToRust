use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        // Initialize a dynamic programming array with 5 elements
        // f[j] represents the maximum score using j elements from array a
        let mut f = [0i64, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2];
        
        for y in b {
            // Process in reverse order to avoid using the same element multiple times
            for j in (0..4).rev() {
                // Update f[j+1] by considering the current state f[j] and adding a[j] * y
                f[j + 1] = cmp::max(f[j + 1], f[j] + (a[j] as i64) * (y as i64));
            }
        }
        
        // Return the maximum score using 4 elements
        f[4]
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read sizes
    let sizes = lines.next().unwrap()?;
    let mut sizes_iter = sizes.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let a_size = sizes_iter.next().unwrap();
    let b_size = sizes_iter.next().unwrap();
    
    // Read array a
    let a_line = lines.next().unwrap()?;
    let a = a_line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    // Read array b
    let b_line = lines.next().unwrap()?;
    let b = b_line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    // Compute and print the result
    let solution = Solution;
    println!("{}", Solution::max_score(a, b));
    
    Ok(())
}