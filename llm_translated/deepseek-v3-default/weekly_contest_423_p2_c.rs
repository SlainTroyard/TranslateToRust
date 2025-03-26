use std::cmp::{max, min};
use std::io::{self, Write};

// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut max_len = 0;  // Variable to store the maximum length of subarrays
    let mut i = 1;        // Start from the second element
    let mut max1 = 1;     // Variable to store the length of the previous increasing subarray

    // Traverse the array
    while i < nums_size {
        let mut max2 = 1;  // Variable to store the length of the current increasing subarray
        // Find the length of the current increasing subarray
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2);  // Find the minimum length between the previous and current subarray
        max_len = max(max_len, max(temp, max2 / 2));  // Update the maximum length based on the calculated values
        max1 = max2;  // Update the length of the previous subarray
        i += 1;  // Move to the next element
    }
    max_len  // Return the maximum length found
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Please type a number!");

    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Call the function and display the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
}