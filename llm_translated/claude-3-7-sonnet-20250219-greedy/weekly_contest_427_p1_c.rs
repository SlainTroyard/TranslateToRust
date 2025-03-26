// Problem: Weekly Contest 427 Problem 1
use std::io::{self, BufRead};

// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i];
            // Calculate target index with proper modulo handling for negative numbers
            let mut target_index = (i as i32 + steps) % nums_size as i32;
            
            if target_index < 0 {
                target_index += nums_size as i32; // Handle negative wrapping
            }
            
            result[i] = nums[target_index as usize];
        }
    }

    result
}

// Main function
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let nums_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse nums_size");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), nums_size, "Input array size doesn't match specified size");
    
    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);
    
    // Print the result
    for (i, &val) in transformed_array.iter().enumerate() {
        print!("{}", val);
        if i < transformed_array.len() - 1 {
            print!(" ");
        }
    }
    println!();
}