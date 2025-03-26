use std::io::{self, BufRead};
use std::cmp;

/// Calculates the minimum possible sum after applying operations to the array
///
/// Parameters:
/// - nums: Array of integers
/// - k: Value to subtract in operation 2
/// - op1: Maximum number of times operation 1 can be applied
/// - op2: Maximum number of times operation 2 can be applied
fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Create 3D dp array
    let mut dp = vec![vec![vec![inf; op2 + 1]; op1 + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= inf {
                    continue;
                }

                let num = nums[i];
                
                // No operation
                dp[i + 1][used_op1][used_op2] = cmp::min(
                    dp[i + 1][used_op1][used_op2],
                    curr_sum + num
                );

                // Apply operation 1 (divide by 2 and round up)
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] = cmp::min(
                        dp[i + 1][used_op1 + 1][used_op2],
                        curr_sum + new_num
                    );
                }

                // Apply operation 2 (subtract k if possible)
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] = cmp::min(
                        dp[i + 1][used_op1][used_op2 + 1],
                        curr_sum + new_num
                    );
                }

                // Apply both operations in different orders
                if used_op1 < op1 && used_op2 < op2 {
                    // Op1 then Op2
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(
                            dp[i + 1][used_op1 + 1][used_op2 + 1],
                            curr_sum + new_num
                        );
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(
                            dp[i + 1][used_op1 + 1][used_op2 + 1],
                            curr_sum + temp_num
                        );
                    }

                    // Op2 then Op1
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(
                            dp[i + 1][used_op1 + 1][used_op2 + 1],
                            curr_sum + new_num
                        );
                    }
                }
            }
        }
    }

    // Find minimum sum across all possible operation combinations
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = cmp::min(min_sum, dp[n][used_op1][used_op2]);
        }
    }

    min_sum
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line to get n, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let n = params[0] as usize;
    let k = params[1];
    let op1 = params[2] as usize;
    let op2 = params[3] as usize;
    
    // Parse second line to get nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);
    
    // Output the result
    println!("{}", result);
}