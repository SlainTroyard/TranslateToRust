use std::error::Error;
use std::io::{self, BufRead};

/// Returns the minimum of two integers.
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// Returns the maximum of three integers.
fn max(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };
    if d > c { d } else { c }
}

/// Computes the longest length as described by LeetCode Weekly Contest 423 Problem 2.
///
/// This function preserves the same algorithm logic as the original C code:
/// It traverses the array to find increasing subarrays,
/// and updates the maximum length based on the minimum length between the previous
/// and current increasing sequences, and half the length of the current sequence.
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut maxlen = 0; // Maximum length found so far
    let mut i = 1;      // Start from the second element
    let mut max1 = 1;   // The length of the previous increasing subarray

    // Iterate through the array
    while i < nums_size {
        let mut max2 = 1; // Length of the current increasing subarray
        // Find the length of the current increasing subarray
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2); // Minimum of previous and current subarray lengths
        // Update maxlen using the maximum of (previously computed maxlen, temp, and (max2 / 2))
        maxlen = max(maxlen, temp, max2 / 2); 
        max1 = max2; // Update previous subarray length with current one
        i += 1;      // Move to the next element
    }
    maxlen
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use stdin to read the input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array from the first line.
    // The first line may contain multiple numbers.
    let first_line = lines
        .next()
        .ok_or("No input found for array size")??;
    let mut tokens = first_line.split_whitespace();

    // Parse the first token as the number of elements.
    let nums_size: usize = if let Some(token) = tokens.next() {
        token.parse()?
    } else {
        return Err("Failed to parse array size".into());
    };

    // Create a vector with the capacity to hold all numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);

    // If there are extra tokens in the first line, process them as array elements.
    for token in tokens {
        if nums.len() < nums_size {
            nums.push(token.parse()?);
        }
    }

    // Read further lines if not all numbers have been collected.
    while nums.len() < nums_size {
        let line = lines
            .next()
            .ok_or("Not enough input numbers")??;
        for token in line.split_whitespace() {
            if nums.len() < nums_size {
                nums.push(token.parse()?);
            }
        }
    }

    // Calculate the result.
    let result = max_increasing_subarrays(&nums);
    
    // Output the result, ensuring to match the exact format (with a newline).
    println!("{}", result);

    Ok(())
}