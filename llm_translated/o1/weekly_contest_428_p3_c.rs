// Translated from C to Rust for LeetCode Weekly Contest 428 Problem 3.
// This Rust program reads an integer n from stdin, followed by n integers,
// then computes and prints the result of the "beautifulSplits" logic.
use std::io::{self, BufRead};
use std::cmp::min;

// This function replicates the exact logic from the C code's beautifulSplits function
fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    // First pass: build prefix table (KMP-style) and compute partial results
    if nums_size > 0 {
        cnt0[0] = 0;
    }
    for i in 1..nums_size {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 == 1 && ((i + 1) / 2) % ((i + 1) - kmpcnt) == 0 {
            res += (nums_size - i - 1) as i32;
        }
    }

    // Second pass: iterate over all possible splits, reuse KMP logic
    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmpcnt = 0;
        // Build another prefix table starting from index i
        if 2 * i < nums_size && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            end = min(end, 3 * i);
        }

        for j in (i + 1)..end {
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            cnt[j - i] = kmpcnt;

            // Check even-length substring condition based on KMP
            let length = j - i + 1;
            if length % 2 == 0 && ((length / 2) % (length - kmpcnt)) == 0 {
                let div = i - 1 + length / 2;
                // If the special condition is met, break
                if length == i * 2 && i % ((div + 1) - cnt0[div]) == 0 {
                    break;
                }
                // Otherwise, increment the result
                res += 1;
            }
        }
    }

    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all tokens (the first is n, followed by n integers)
    let stdin = io::stdin();
    let mut tokens: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for num_str in line.split_whitespace() {
            if let Ok(num) = num_str.parse::<i32>() {
                tokens.push(num);
            }
        }
    }

    // The first token is the size n
    let n = tokens[0] as usize;

    // The next n tokens form the array
    let mut nums = vec![0; n];
    for i in 0..n {
        nums[i] = tokens[i + 1];
    }

    // Calculate the result and print
    let result = beautiful_splits(&nums);
    println!("{}", result);

    Ok(())
}