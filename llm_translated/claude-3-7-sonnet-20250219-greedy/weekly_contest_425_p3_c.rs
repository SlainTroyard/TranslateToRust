use std::io::{self, BufRead};
use std::cmp::min;

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Create a 3D DP array
    let mut dp = vec![vec![vec![inf; (op2 + 1) as usize]; (op1 + 1) as usize]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 as usize {
            for used_op2 in 0..=op2 as usize {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= inf {
                    continue;
                }

                let num = nums[i];
                
                // Option 1: Don't apply any operation
                dp[i + 1][used_op1][used_op2] = min(dp[i + 1][used_op1][used_op2], curr_sum + num);

                // Option 2: Apply operation 1 (divide by 2 and round up)
                if used_op1 < op1 as usize {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] = min(dp[i + 1][used_op1 + 1][used_op2], curr_sum + new_num);
                }

                // Option 3: Apply operation 2 (subtract k if num >= k)
                if used_op2 < op2 as usize && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] = min(dp[i + 1][used_op1][used_op2 + 1], curr_sum + new_num);
                }

                // Option 4: Apply both operations in different orders
                if used_op1 < op1 as usize && used_op2 < op2 as usize {
                    // First op1, then op2
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + temp_num);
                    }

                    // First op2, then op1
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum
    let mut min_sum = inf;
    for used_op1 in 0..=op1 as usize {
        for used_op2 in 0..=op2 as usize {
            min_sum = min(min_sum, dp[n][used_op1][used_op2]);
        }
    }

    min_sum
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line: n, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = params[0];
    let k = params[1];
    let op1 = params[2];
    let op2 = params[3];
    
    // Parse second line: nums array
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the function
    let result = min_array_sum(&nums, k, op1, op2);
    
    // Output the result
    println!("{}", result);
}