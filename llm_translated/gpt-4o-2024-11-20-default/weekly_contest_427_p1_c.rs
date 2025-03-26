use std::io::{self, BufRead};

/// Constructs the transformed array based on the given logic.
/// - nums: A reference to the input vector of integers.
/// - Returns: A vector containing the transformed array.
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size]; // Initialize result array with default values

    for (i, &num) in nums.iter().enumerate() {
        if num == 0 {
            result[i] = num; // Directly copy zero values
        } else {
            let steps = num;
            // Calculate the target index, ensuring proper handling of wrapping
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
    
    // Read the size of the array
    let nums_size: usize = lines
        .next()
        .expect("Expected a line of input")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse numsSize as usize");
    
    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected a line of input")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse element as i32"))
        .collect();

    // Ensure the input size matches the specified nums_size
    assert_eq!(
        nums.len(),
        nums_size,
        "Input array size does not match the specified nums_size"
    );

    // Perform the transformation
    let transformed_array = construct_transformed_array(&nums);

    // Output the transformed array
    for (i, value) in transformed_array.iter().enumerate() {
        // Print each element, separated by a space
        print!("{}", value);
        if i != transformed_array.len() - 1 {
            print!(" ");
        }
    }
    println!(); // Print a newline at the end
}