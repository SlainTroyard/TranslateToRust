use std::io::{self, BufRead};

/// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i];
            let mut target_index = (i as i32 + steps) % nums_size as i32;

            if target_index < 0 {
                target_index += nums_size as i32; // Handle negative wrapping
            }

            result[i] = nums[target_index as usize];
        }
    }

    result
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: size of the array
    let nums_size: usize = lines
        .next()
        .expect("Expected input for numsSize")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Second line: array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for nums array")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), nums_size, "Input array size does not match numsSize");

    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);

    // Print the transformed array
    for value in transformed_array {
        print!("{} ", value);
    }
    println!();
}