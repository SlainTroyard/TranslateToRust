use std::io;
use std::cmp;

struct Solution;

impl Solution {
    fn max_score(a: &Vec<i32>, b: &Vec<i32>) -> i64 {
        // Initialize dp array with minimum values except for the first element
        let mut f = [0i64, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2];
        
        // For each value in array b
        for &y in b {
            // Update dp array from right to left to avoid using the same element multiple times
            for j in (0..=3).rev() {
                f[j + 1] = cmp::max(f[j + 1], f[j] + (a[j] as i64) * (y as i64));
            }
        }
        
        f[4]
    }
}

fn main() -> io::Result<()> {
    // Read array sizes
    let mut sizes_input = String::new();
    io::stdin().read_line(&mut sizes_input)?;
    
    let sizes: Vec<usize> = sizes_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let a_size = sizes[0];
    let b_size = sizes[1];
    
    // Read array a
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input)?;
    let a: Vec<i32> = a_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read array b
    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input)?;
    let b: Vec<i32> = b_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", Solution::max_score(&a, &b));
    
    Ok(())
}