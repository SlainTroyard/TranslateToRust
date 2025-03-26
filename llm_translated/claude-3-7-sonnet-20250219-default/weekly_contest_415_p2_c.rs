use std::io::{self, BufRead};
use std::cmp;

/// Maximum score function as per the problem statement
/// Takes two slices of integers and returns the maximum possible score
fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    
    // Create a 2D array for dynamic programming
    // f[i][j] represents the maximum score after considering i elements from array b
    // and j elements from array a
    let mut f = vec![vec![i64::MIN; 5]; n + 1];
    
    // Base case: no elements selected
    f[0][0] = 0;
    
    // Fill the dp table
    for i in 1..=n {
        for j in 0..=4 {
            // Option 1: Skip the current element in array b
            f[i][j] = f[i-1][j];
            
            // Option 2: Include the current element in array b
            // and use the j-th element from array a (if possible)
            if j > 0 {
                f[i][j] = cmp::max(
                    f[i][j], 
                    f[i-1][j-1] + (a[j-1] as i64) * (b[i-1] as i64)
                );
            }
        }
    }
    
    // Return the maximum score after considering all elements from b
    // and exactly 4 elements from a
    f[n][4]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read array sizes
    let sizes = lines.next().unwrap().unwrap();
    let sizes: Vec<i32> = sizes.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let a_size = sizes[0] as usize;
    let b_size = sizes[1] as usize;
    
    // Read array a
    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = a_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read array b
    let b_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = b_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", max_score(&a, &b));
}