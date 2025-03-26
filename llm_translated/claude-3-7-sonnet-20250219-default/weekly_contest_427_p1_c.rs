use std::io::{self, BufRead};

/// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = Vec::with_capacity(nums_size);

    for i in 0..nums_size {
        if nums[i] == 0 {
            result.push(nums[i]);
        } else {
            let steps = nums[i];
            // Convert to isize to avoid overflow and handle negative values properly
            let mut target_index = (i as isize + steps as isize) % nums_size as isize;
            
            // Handle negative wrapping
            if target_index < 0 {
                target_index += nums_size as isize;
            }
            
            result.push(nums[target_index as usize]);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let nums_size: usize = lines.next()
                                .expect("Failed to read line")
                                .expect("Failed to read line")
                                .trim()
                                .parse()
                                .expect("Failed to parse number");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next()
                              .expect("Failed to read line")
                              .expect("Failed to read line")
                              .split_whitespace()
                              .map(|s| s.parse().expect("Failed to parse number"))
                              .collect();
    
    // Verify we have the correct number of elements
    assert_eq!(nums.len(), nums_size, "Input array size doesn't match specified size");
    
    // Construct the transformed array
    let transformed_array = construct_transformed_array(&nums);
    
    // Print the result with the same format as the original code
    for (i, &num) in transformed_array.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}