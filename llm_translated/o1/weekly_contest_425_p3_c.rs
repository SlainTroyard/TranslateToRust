/// Translated from the C code for "Weekly Contest 425 Problem 3"
/// Requirements met:
/// 1. Translates the entire file as a complete program
/// 2. Preserves the algorithm logic exactly
/// 3. Uses idiomatic Rust with proper handling of data
/// 4. Maintains the exact same stdin/stdout format as the original C code
///    (reads n, k, op1, op2, then reads n integers, then prints a single integer)
/// 5. Adds helpful comments explaining the code

use std::io::{self, BufRead};

/// This function replicates the C function `minArraySum`
///
/// nums: the array of integers
/// k: the threshold for subtract operation
/// op1: how many times the "halve" operation can be used
/// op2: how many times the "subtract k" operation can be used
fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    // Use a large "infinity" value to represent unreachable/uninitialized states
    let inf = i32::MAX / 2;

    // dp[i][used_op1][used_op2] = minimum sum achievable using
    // first i elements with used_op1 "halve" operations and
    // used_op2 "subtract k" operations used so far.
    let mut dp = vec![vec![vec![inf; op2 + 1]; op1 + 1]; n + 1];

    // Base case: no elements used, no operations used -> sum = 0
    dp[0][0][0] = 0;

    // Fill the DP table
    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= inf {
                    // Skip unreachable states
                    continue;
                }

                let num = nums[i];

                // No operation on current number
                let next_val = curr_sum + num;
                if next_val < dp[i + 1][used_op1][used_op2] {
                    dp[i + 1][used_op1][used_op2] = next_val;
                }

                // Use the halve operation (op1)
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    let next_val = curr_sum + new_num;
                    if next_val < dp[i + 1][used_op1 + 1][used_op2] {
                        dp[i + 1][used_op1 + 1][used_op2] = next_val;
                    }
                }

                // Use the subtract-k operation (op2) if num >= k
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    let next_val = curr_sum + new_num;
                    if next_val < dp[i + 1][used_op1][used_op2 + 1] {
                        dp[i + 1][used_op1][used_op2 + 1] = next_val;
                    }
                }

                // Use both operations in one step if both are still available
                if used_op1 < op1 && used_op2 < op2 {
                    // Halve first, then subtract k if possible
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        // If the halved number is still >= k
                        let new_num = temp_num - k;
                        let next_val = curr_sum + new_num;
                        if next_val < dp[i + 1][used_op1 + 1][used_op2 + 1] {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = next_val;
                        }
                    } else {
                        // Halve is done but there's no need/possibility to subtract k
                        let next_val = curr_sum + temp_num;
                        if next_val < dp[i + 1][used_op1 + 1][used_op2 + 1] {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = next_val;
                        }
                    }

                    // Subtract k first, then halve
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        let next_val = curr_sum + new_num;
                        if next_val < dp[i + 1][used_op1 + 1][used_op2 + 1] {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = next_val;
                        }
                    }
                }
            }
        }
    }

    // Find the minimum sum among all states using all n elements
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            if dp[n][used_op1][used_op2] < min_sum {
                min_sum = dp[n][used_op1][used_op2];
            }
        }
    }

    min_sum
}

fn main() -> io::Result<()> {
    // Read all tokens (numbers) from stdin
    // This allows parsing input that may come in multiple lines
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for word in line.split_whitespace() {
            tokens.push(word.to_string());
        }
    }

    // Parse n, k, op1, op2 in the exact order they appear in the input
    let mut iter = tokens.iter();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: usize = iter.next().unwrap().parse().unwrap();
    let op2: usize = iter.next().unwrap().parse().unwrap();

    // Parse the array of n integers
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = iter.next().unwrap().parse().unwrap();
        nums.push(value);
    }

    // Compute the result
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result (same as the C code: printf("%d\n", result))
    println!("{}", result);

    Ok(())
}