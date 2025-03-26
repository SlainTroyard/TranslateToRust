use std::cmp::{max, min};
use std::io;

// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
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
        let temp = min(max1, max2); // Find the minimum length between the previous and current subarray
        maxlen = max(maxlen, max(temp, max2 / 2)); // Update the maximum length based on the calculated values
        max1 = max2; // Update the length of the previous subarray
        i += 1; // Move to the next element
    }
    maxlen // Return the maximum length found
}

fn main() {
    let mut nums_size_str = String::new();
    io::stdin()
        .read_line(&mut nums_size_str)
        .expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(nums_size);
    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");

    for num_str in nums_str.trim().split_whitespace() {
        let num: i32 = num_str.parse().expect("Invalid number");
        nums.push(num);
    }

    // Call the function and display the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
}