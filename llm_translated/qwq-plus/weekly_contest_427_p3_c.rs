use std::io;

/// Computes the maximum subarray sum with specific constraints.
///
/// # Arguments
/// * `nums` - Slice of integers representing the input array.
/// * `k` - Window size for initial subarray sums.
///
/// # Returns
/// The maximum subarray sum as per the problem's constraints.
pub fn max_subarray_sum(nums: &[i32], k: usize) -> i64 {
    let n = nums.len();
    if n < k {
        // If the array length is less than k, return the minimum possible value
        return i64::MIN;
    }

    let mut keep = Vec::new();
    let mut cur: i64 = 0;
    for (i, &num) in nums.iter().enumerate() {
        cur += num as i64;
        if i >= k - 1 {
            // Store the sum of the current window of size k
            keep.push(cur);
            // Subtract the first element of the current window to slide the window forward
            let first_element_index = i - (k - 1);
            cur -= nums[first_element_index] as i64;
        }
    }

    let mut ans = i64::MIN;
    let min_k = std::cmp::min(k, keep.len());
    for i in 0..min_k {
        // Initialize current_sum and current_max for the starting index i
        let mut current_sum = 0;
        let mut current_max = keep[i]; // At least the first element's value
        // Iterate through elements spaced by k starting from i
        for l in (i..keep.len()).step_by(k) {
            current_sum += keep[l];
            if current_sum > current_max {
                current_max = current_sum;
            }
            if current_sum < 0 {
                current_sum = 0; // Reset if negative to avoid negative contributions
            }
        }
        // Update the global maximum
        if current_max > ans {
            ans = current_max;
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    // Read all tokens from stdin
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        tokens.extend(line.split_whitespace());
    }

    // Parse the first two tokens as n and k
    let n = tokens[0].parse::<i32>().unwrap();
    let k = tokens[1].parse::<i32>().unwrap();

    // Parse the next 'n' tokens as the array elements
    let nums: Vec<i32> = tokens[2..]
        .iter()
        .take(n as usize)
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the result
    let result = max_subarray_sum(&nums, k as usize);
    println!("{}", result);
}