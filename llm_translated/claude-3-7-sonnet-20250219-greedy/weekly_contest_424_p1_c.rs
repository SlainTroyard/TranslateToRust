// Problem: Weekly Contest 424 Problem 1
use std::io::{self, BufRead};

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;
    
    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }
    
    ret
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse array size");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure the array has the expected size
    assert_eq!(nums.len(), n, "Array size mismatch");
    
    // Calculate and print the result
    let result = count_valid_selections(&nums);
    println!("{}", result);
    
    Ok(())
}