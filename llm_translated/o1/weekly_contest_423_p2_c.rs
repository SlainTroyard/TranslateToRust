use std::io::{self, BufRead};
use std::error::Error;

/// Returns the minimum of two integers.
fn c_min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// Returns the maximum of three integers.
fn c_max(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };
    if d > c { d } else { c }
}

/// Finds the length of the longest "increasing subarrays" as specified.
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut maxlen = 0;    // Store the maximum length of subarrays
    let mut i = 1;         // Start from the second element
    let mut max1 = 1;      // Length of the previous increasing subarray

    // Traverse the array
    while i < nums_size {
        let mut max2 = 1;  // Length of the current increasing subarray
        // Find how long the current increasing subarray is
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = c_min(max1, max2);  // Minimum of the previous and current subarray lengths
        // Update the maximum length: compare current max with temp, and also max2/2
        maxlen = c_max(maxlen, temp, max2 / 2);
        max1 = max2;  // Update the length of the previous subarray
        i += 1;       // Move to the next element
    }
    maxlen
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the array size
    let line = lines.next().ok_or("No input for numsSize")??;
    let nums_size: usize = line.trim().parse()?;

    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        let line = lines.next().ok_or("Not enough numbers provided")??;
        for val in line.split_whitespace() {
            if nums.len() < nums_size {
                nums.push(val.parse()?);
            } else {
                break;
            }
        }
    }

    // Compute the result
    let result = max_increasing_subarrays(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}