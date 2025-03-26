use std::io;
use std::io::BufRead;

// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size]; // Initialize result vector with zeros

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = 0;
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
    // Read numsSize from stdin
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input for numsSize");

    // Read nums array from stdin
    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums element"))
        .collect();

    // Check if the input nums size matches the declared size
    if nums.len() != nums_size {
        panic!("Input nums size does not match the declared numsSize");
    }

    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);

    // Print the transformed array
    for i in 0..transformed_array.len() {
        print!("{} ", transformed_array[i]);
    }
    println!();
}