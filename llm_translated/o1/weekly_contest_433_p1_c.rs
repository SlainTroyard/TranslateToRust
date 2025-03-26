// Problem: Weekly Contest 433 Problem 1
// Translate from C to Rust, preserving the exact algorithm and I/O format

use std::io::{self, BufRead};
use std::error::Error;

/// Computes the subarray sum using the same logic as the original C code.
fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    // This vector holds partial sums, analogous to 'sums' in the C code
    let mut sums = vec![0; n + 1];
    let mut ans = 0;

    for i in 0..n {
        // sums[i+1] = sums[i] + nums[i]
        sums[i + 1] = sums[i] + nums[i];

        // Replicate max(0, i - nums[i]) from the C macro
        let diff_index = i as i32 - nums[i];
        let m = if diff_index > 0 { diff_index } else { 0 } as usize;

        // ans += sums[i+1] - sums[m]
        ans += sums[i + 1] - sums[m];
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the integer n
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let n: usize = buffer.trim().parse()?;

    // Read n integers. Like the C code with scanf("%d"), this loop
    // keeps reading until we've collected exactly n integers.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        buffer.clear();
        if io::stdin().read_line(&mut buffer)? == 0 {
            break;  // Stop if no more input
        }
        for chunk in buffer.split_whitespace() {
            if nums.len() == n {
                break;
            }
            nums.push(chunk.parse()?);
        }
    }

    // Compute the result using subarray_sum
    let result = subarray_sum(&nums);

    // Print the result in the same format as the C code's printf("%d\n", ...)
    println!("{}", result);

    Ok(())
}