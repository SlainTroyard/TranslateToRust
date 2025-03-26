use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        
        // Iterate through the array except the last element
        for i in 0..(nums.len() - 1) {
            // Update the maximum value seen so far
            mx = cmp::max(mx, nums[i]);
            // Add the current maximum to the answer
            ans += mx as i64;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the nums array
    let nums_size: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse nums size");
    
    // Read the nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), nums_size, "Input size mismatch");
    
    // Create solution and call the method
    let sol = Solution;
    println!("{}", Solution::find_maximum_score(&nums));
    
    Ok(())
}