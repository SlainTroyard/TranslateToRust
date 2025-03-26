use std::io::{self, BufRead};
use std::cmp::{max, min};
use std::i64;

/// Returns the maximum of two i64 values
fn max_value(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

/// Calculates the maximum subarray sum with specific constraints
fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur = 0i64;
    let mut keep = vec![0i64; n - k + 1];

    // Calculate sums of all k-sized subarrays
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;

    // For each possible starting position within the first k elements
    for i in 0..min(k, n - k + 1) {
        let mut cur = 0i64;
        let mut max_val = keep[i];

        // Process subarrays that start at positions i, i+k, i+2k, ...
        for l in (i..n - k + 1).step_by(k) {
            cur += keep[l];

            if cur > max_val {
                max_val = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        ans = max_value(ans, max_val);
    }
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n (size of array)
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse n");
    
    // Read k
    let k: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse k");
    
    // Read the array elements
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();
    
    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);
    
    Ok(())
}