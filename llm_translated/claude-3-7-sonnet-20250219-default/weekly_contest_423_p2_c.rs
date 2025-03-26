// Problem: Weekly Contest 423 Problem 2
use std::cmp::{max, min};
use std::io::{self, BufRead};

/// Function to return the maximum of three integers
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };  // Find the maximum of a and b
    if d > c { d } else { c }           // Return the maximum of d and c
}

/// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    if nums_size <= 1 {
        return nums_size as i32;
    }
    
    let mut maxlen = 0;  // Variable to store the maximum length of subarrays
    let mut i = 1;       // Start from the second element
    let mut max1 = 1;    // Variable to store the length of the previous increasing subarray
    
    // Traverse the array
    while i < nums_size {
        let mut max2 = 1;  // Variable to store the length of the current increasing subarray
        // Find the length of the current increasing subarray
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2);  // Find the minimum length between the previous and current subarray
        maxlen = max_of_three(maxlen, temp, max2 / 2);  // Update the maximum length based on the calculated values
        max1 = max2;  // Update the length of the previous subarray
        i += 1;  // Move to the next element
    }
    maxlen  // Return the maximum length found
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the size of the array
    let nums_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Failed to parse number");
    
    // Input the array elements
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Call the function and display the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
}