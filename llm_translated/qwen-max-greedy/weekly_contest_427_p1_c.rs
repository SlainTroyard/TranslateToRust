// Problem: Weekly Contest 427 Problem 1

use std::io;

/// Function to construct the transformed array
fn construct_transformed_array(nums: &Vec<i32>, nums_size: usize) -> Vec<i32> {
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i];
            let target_index = (i as isize + steps) as usize % nums_size;

            result[i] = nums[target_index];
        }
    }

    result
}

fn main() {
    // Read the size of the array from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Please type a number!");

    // Read the array elements from stdin
    let mut nums = vec![0; nums_size];
    for i in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Please type a number!");
    }

    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums, nums_size);

    // Print the transformed array
    for num in &transformed_array {
        print!("{} ", num);
    }
    println!();
}