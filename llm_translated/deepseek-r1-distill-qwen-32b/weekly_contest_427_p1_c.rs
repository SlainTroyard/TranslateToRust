// Problem: Weekly Contest 427 Problem 1
use std::io;

/// Constructs the transformed array based on the given rules.
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = Vec::with_capacity(nums_size);
    
    for i in 0..nums_size {
        let steps = nums[i];
        if steps == 0 {
            result.push(0);
        } else {
            let mut target_index = (i as i32 + steps) % nums_size as i32;
            // Ensure the target index is non-negative
            if target_index < 0 {
                target_index += nums_size as i32;
            }
            let target_index = target_index as usize;
            result.push(nums[target_index]);
        }
    }
    
    result
}

fn main() {
    // Read all input at once
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Split input into tokens and parse
    let mut tokens = input.split_whitespace();
    let nums_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let nums: Vec<i32> = tokens.map(|t| t.parse().unwrap()).collect();
    
    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);
    
    // Print the result in the required format
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}