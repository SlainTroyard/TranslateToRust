use std::io::{self, BufRead};

// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32], nums_size: usize) -> Vec<i32> {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Construct transformed array
    let transformed_array = construct_transformed_array(&nums, nums_size);

    // Print transformed array
    for &num in &transformed_array {
        print!("{} ", num);
    }
    println!();

    Ok(())
}