use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_partitions(nums: &Vec<i32>) -> i32 {
        // Calculate the sum of all elements in the vector
        let s: i32 = nums.iter().sum();
        
        // If the sum is odd, return 0 (can't partition evenly)
        // Otherwise, return the size of the array minus 1
        if s % 2 != 0 {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse the number of elements");
    
    // Read the array elements
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse an integer"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), n, "Number of elements doesn't match the specified count");
    
    // Create a solution instance and call the method
    let sol = Solution;
    println!("{}", sol.count_partitions(&nums));
    
    Ok(())
}