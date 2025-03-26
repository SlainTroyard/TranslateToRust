// Problem: Weekly Contest 427 Problem 1

use std::io;

// Function to construct the transformed array
fn construct_transformed_array(nums: &Vec<i32>, nums_size: usize) -> Vec<i32> {
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i] as isize;
            let target_index = (i as isize + steps) % nums_size as isize;

            let target_index = if target_index < 0 {
                target_index + nums_size as isize // Handle negative wrapping
            } else {
                target_index
            };

            result[i] = nums[target_index as usize];
        }
    }

    result
}

fn main() {
    // Read the size of the array from stdin
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Please type a number!");

    // Read the array elements from stdin
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums, nums_size);

    // Print the transformed array
    for &num in &transformed_array {
        print!("{} ", num);
    }
    println!();
}