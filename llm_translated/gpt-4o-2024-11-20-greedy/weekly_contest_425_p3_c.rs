use std::io::{self, BufRead};
use std::cmp::min;

fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Create a 3D DP table initialized to `inf`
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

                // Case 1: No operation
                dp[i + 1][used_op1][used_op2] = min(dp[i + 1][used_op1][used_op2], curr_sum + num);

                // Case 2: Apply op1
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] = min(dp[i + 1][used_op1 + 1][used_op2], curr_sum + new_num);
                }

                // Case 3: Apply op2
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] = min(dp[i + 1][used_op1][used_op2 + 1], curr_sum + new_num);
                }

                // Case 4: Apply both op1 and op2
                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + temp_num);
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum in the last row of the DP table
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = min(min_sum, dp[n][used_op1][used_op2]);
        }
    }

    min_sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: n, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: usize = iter.next().unwrap().parse().unwrap();
    let op2: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line: nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);
}