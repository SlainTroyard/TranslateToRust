use std::cmp::{min, max};
use std::io;

// Function to return the minimum of two integers
fn min_int(a: i32, b: i32) -> i32 {
    min(a, b)
}

// Function to return the maximum of three integers
fn max_int(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b }; // Find the maximum of a and b
    if d > c { d } else { c } // Return the maximum of d and c
}

// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    if nums_size == 0 {
        return 0;
    }
    let mut maxlen = 0; // Variable to store the maximum length of subarrays
    let mut i = 1; // Start from the second element
    let mut max1 = 1; // Variable to store the length of the previous increasing subarray

    // Traverse the array
    while i < nums_size {
        let mut max2 = 1; // Variable to store the length of the current increasing subarray
        // Find the length of the current increasing subarray
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min_int(max1, max2); // Find the minimum length between the previous and current subarray
        maxlen = max_int(maxlen, temp, max2 / 2); // Update the maximum length based on the calculated values
        max1 = max2; // Update the length of the previous subarray
        i += 1; // Move to the next element
    }
    maxlen
}

fn main() {
    let mut nums_size_str = String::new();

    // Input the size of the array
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input for numsSize");

    let mut nums = Vec::with_capacity(nums_size);

    let mut nums_input_str = String::new();
    io::stdin().read_line(&mut nums_input_str).expect("Failed to read line");
    let nums_str_vec: Vec<&str> = nums_input_str.trim().split_whitespace().collect();

    // Input the array elements
    for i in 0..nums_size {
        let num: i32 = nums_str_vec[i].parse().expect("Invalid input for nums element");
        nums.push(num);
    }

    // Call the function and display the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
}