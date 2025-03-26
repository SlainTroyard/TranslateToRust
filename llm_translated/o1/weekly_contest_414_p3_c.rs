// Problem: Weekly Contest 414 Problem 3
//
// This Rust program replicates the logic and I/O format of the
// provided C solution exactly, reading an integer "numsSize" first,
// then reading "numsSize" integers for the array. It computes the
// "findMaximumScore" in the same way as the original C code and
// prints the result to stdout.

use std::io::{self, Read};

/// Computes the maximum score as described in the original C code:
/// 1. Initialize ans = 0, l = 0
/// 2. Iterate r from 1 to numsSize - 1:
///    - If nums[l] < nums[r], then:
///      ans += (r - l) * nums[l], and l = r
/// 3. Finally, add (numsSize - l - 1) * nums[l] to ans.
/// Returns this final result as i64.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let nums_size = nums.len() as i64;
    if nums_size == 0 {
        return 0; // Edge case if empty
    }

    let mut ans: i64 = 0;
    let mut l: i64 = 0;
    // r goes from 1 to nums_size - 1
    for r in 1..nums_size {
        if nums[l as usize] < nums[r as usize] {
            ans += (r - l) * (nums[l as usize] as i64);
            l = r;
        }
    }

    // Add the remaining part after the loop
    ans + (nums_size - l - 1) * (nums[l as usize] as i64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input into a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Tokenize by whitespace to handle multiple lines or values per line
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>());

    // Read numsSize
    let nums_size = match tokens.next() {
        Some(Ok(n)) => n as usize,
        _ => return Err("Invalid input for numsSize".into()),
    };

    // Read nums array of length numsSize
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let val = match tokens.next() {
            Some(Ok(n)) => n,
            _ => return Err("Invalid input for nums".into()),
        };
        nums.push(val);
    }

    // Compute the answer
    let ans = find_maximum_score(&nums);

    // Print the result on its own line, matching the C code's behavior
    println!("{}", ans);

    Ok(())
}