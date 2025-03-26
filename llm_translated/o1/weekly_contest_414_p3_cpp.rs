/// Translated Rust solution for "Weekly Contest 414 Problem 3"
/// This program reads an integer n (numsSize), then n integers for the array nums,
/// and computes the result. The input is read from stdin and output is printed to stdout.
///
/// This translation preserves the same logic and I/O format as the original C++ code.

use std::io::{self, BufRead};

/// Equivalent to the C++ Solution class method.
/// Iterates through nums (up to the second last element), tracking the maximum so far,
/// accumulating it into ans.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut mx = 0;
    for i in 0..nums.len().saturating_sub(1) {
        mx = mx.max(nums[i]);
        ans += mx as i64;
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the nums array (numsSize)
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let nums_size: usize = buffer.trim().parse()?;

    // Read nums_size integers (they can be on multiple lines or the same line)
    let mut nums = Vec::with_capacity(nums_size);
    let mut read_count = 0;
    while read_count < nums_size {
        buffer.clear();
        if io::stdin().read_line(&mut buffer)? == 0 {
            break; // no more input
        }
        for token in buffer.split_whitespace() {
            if let Ok(val) = token.parse::<i32>() {
                nums.push(val);
                read_count += 1;
                if read_count == nums_size {
                    break;
                }
            }
        }
    }

    // Compute the result using the translated function
    let answer = find_maximum_score(&nums);

    // Print the result (same format as the original C++ code)
    println!("{}", answer);

    Ok(())
}