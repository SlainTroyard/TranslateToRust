// Problem: Weekly Contest 415 Problem 1
use std::io::{self, Read};

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2];
    let mut count = 0;
    
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }
    
    // Original always returns array of size 2
    result
}

fn main() {
    // Read num_size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num_size: usize = input.trim().parse().expect("Invalid input");
    num_size += 2; // Add 2 to num_size as in the original code
    
    // Read the numbers
    let mut nums = vec![0; num_size];
    for i in 0..num_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read number");
        nums[i] = input.trim().parse().expect("Invalid number");
    }
    
    // Get the sneaky numbers
    let return_size = 2; // Original code always sets returnSize to 2
    let result = get_sneaky_numbers(&nums);
    
    // Print results
    for i in 0..return_size {
        print!("{} ", result[i]);
    }
}