use std::error::Error;
use std::io::{self, Read};

/// Constructs the transformed array based on the given rules:
///   - If nums[i] is 0, then result[i] = 0
///   - Otherwise, we calculate targetIndex = (i + nums[i]) % numsSize
///     (with handling for negative wrapping), and set result[i] = nums[targetIndex]
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            // If the current number is 0, place 0 in the result at position i
            result[i] = nums[i];
        } else {
            // Otherwise, calculate the target index with handling for negative values
            let steps = nums[i];
            let mut target_index = (i as i32 + steps) % nums_size as i32;
            if target_index < 0 {
                // Handle negative wrapping
                target_index += nums_size as i32;
            }
            result[i] = nums[target_index as usize];
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all input from stdin into a single string buffer
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Split the buffer on whitespace and parse the tokens
    let mut tokens = buffer.split_whitespace();

    // Read numsSize
    let nums_size: usize = tokens
        .next()
        .ok_or("Failed to read numsSize")?
        .parse::<usize>()?;

    // Read the array of integers
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let val: i32 = tokens
            .next()
            .ok_or("Failed to read next integer for nums")?
            .parse()?;
        nums.push(val);
    }

    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);

    // Print the transformed array with the exact same format as in the C code
    for val in &transformed_array {
        print!("{} ", val);
    }
    println!();

    Ok(())
}