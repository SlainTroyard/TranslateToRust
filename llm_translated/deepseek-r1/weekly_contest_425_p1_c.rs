use std::io::{self, Read};

// Reads all input as a vector of i32 by splitting whitespace
fn read_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    input.split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect()
}

// Finds the minimum sum of a subarray with length between l and r (inclusive)
fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    let mut min_sum = None;

    // Iterate over all possible window lengths from l to r
    for window_length in l..=r {
        let window_len = window_length as usize;
        let mut sum = 0;

        // Sliding window approach for the current window length
        for i in 0..nums.len() {
            sum += nums[i];

            // Subtract the element that's out of the window
            if i >= window_len {
                sum -= nums[i - window_len];
            }

            // Check if the current window is valid (i >= window_len - 1)
            if i >= window_len - 1 && sum > 0 {
                // Update the minimum sum if current sum is smaller
                min_sum = Some(match min_sum {
                    Some(current_min) => current_min.min(sum),
                    None => sum,
                });
            }
        }
    }

    // Return -1 if no valid subarray found, otherwise the minimum sum
    min_sum.unwrap_or(-1)
}

fn main() {
    // Read and parse all input
    let input = read_input();
    let mut iter = input.into_iter();

    // Parse nums array
    let nums_size = iter.next().expect("Missing nums_size");
    let nums: Vec<i32> = iter.by_ref().take(nums_size as usize).collect();

    // Parse l and r
    let l = iter.next().expect("Missing l");
    let r = iter.next().expect("Missing r");

    // Compute and print the result
    let result = minimum_sum_subarray(&nums, l, r);
    println!("{}", result);
}