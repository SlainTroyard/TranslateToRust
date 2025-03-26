use std::io::{self, BufRead};
use std::cmp::{max, min};
use std::i64::MIN as I64_MIN;

/// Returns the maximum of two i64 values
fn max_value(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

/// Calculates the maximum subarray sum according to the problem requirements
fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur = 0i64;
    let mut keep = vec![0i64; n - k + 1];

    // Calculate sums of subarrays of length k
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = I64_MIN;

    // Check different starting positions and find maximum kadane sum
    for i in 0..min(k, n - k + 1) {
        let mut cur = 0i64;
        let mut max_val = keep[i];

        // Process subarrays that start at positions i, i+k, i+2k, ...
        let mut l = i;
        while l < n - k + 1 {
            cur += keep[l];

            if cur > max_val {
                max_val = cur;
            }
            if cur < 0 {
                cur = 0;
            }
            
            l += k;
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
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();
    
    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);
    
    Ok(())
}