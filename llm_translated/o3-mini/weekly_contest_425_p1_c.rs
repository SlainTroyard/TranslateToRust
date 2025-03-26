use std::io::{self, Read};

/// Solve the LeetCode Weekly Contest 425 Problem 1 in idiomatic Rust.
///
/// The function `minimum_sum_subarray` mirrors the original C logic.
/// It takes a slice of numbers and a range (l, r) for the subarray window length,
/// then computes the rolling sum for each window length in that range, and finally
/// returns the smallest positive sum found, or -1 if none exists.
fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    // `min_sum` will be used as None until we find a valid sum,
    // mimicking the C usage of checking for '\0' (which is 0) as a flag.
    let mut min_sum: Option<i32> = None;
    let n = nums.len();

    // Iterate over each possible window length from l to r (inclusive)
    for window in l..=r {
        // Convert the window size to usize for indexing.
        let window = window as usize;
        let mut sum = 0;

        // Use a sliding window technique to calculate the sum.
        for i in 0..n {
            sum += nums[i];
            // If we have added at least `window + 1` elements, subtract the element that falls off.
            if i >= window {
                sum -= nums[i - window];
            }
            // Check if we have a valid window (i >= window - 1) and the current sum is positive.
            if sum > 0 && i >= window.saturating_sub(1) {
                // Update `min_sum` if it's not set or if the current sum is smaller.
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }
    // Return -1 if no valid subarray sum was found; otherwise, return the found minimum sum.
    min_sum.unwrap_or(-1)
}

fn main() -> io::Result<()> {
    // Read the entire input from stdin as a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input into tokens by whitespace.
    let mut tokens = input.split_whitespace();

    // Parse the first token as numsSize.
    let nums_size: usize = tokens
        .next()
        .expect("Expected the size of the array")
        .parse()
        .expect("Error parsing the size of the array");

    // Read the next nums_size numbers as the array elements.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = tokens
            .next()
            .expect("Expected a number in the array")
            .parse()
            .expect("Error parsing an array element");
        nums.push(num);
    }

    // Read the next two tokens as l and r.
    let l: i32 = tokens
        .next()
        .expect("Expected l")
        .parse()
        .expect("Error parsing l");
    let r: i32 = tokens
        .next()
        .expect("Expected r")
        .parse()
        .expect("Error parsing r");

    // Call the function that finds the minimum sum subarray.
    let result = minimum_sum_subarray(&nums, l, r);

    // Print the result with a newline, following the original format.
    println!("{}", result);

    Ok(())
}