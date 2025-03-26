use std::io::{self, BufRead};

/// Finds the maximum score based on the given array of numbers.
/// 
/// This function implements the algorithm from LeetCode Weekly Contest 414 Problem 3.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let nums_size = nums.len();
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let mut r: usize = 1;
    
    while r < nums_size {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    
    // Add the final segment's contribution
    ans + ((r - l - 1) as i64 * nums[l] as i64)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let nums_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), nums_size, "Input size mismatch");
    
    // Calculate and print the result
    println!("{}", find_maximum_score(&nums));
    
    Ok(())
}