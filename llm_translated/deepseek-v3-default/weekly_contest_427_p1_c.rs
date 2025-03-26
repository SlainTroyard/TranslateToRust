use std::io::{self, Write};

// Function to construct the transformed array
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

// Main function
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let transformed_array = construct_transformed_array(&nums);

    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}