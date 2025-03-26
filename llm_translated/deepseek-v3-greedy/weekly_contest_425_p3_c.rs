use std::io::{self, BufRead};
use std::cmp;

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Initialize DP table
    let mut dp = vec![vec![vec![inf; (op2 + 1) as usize]; (op1 + 1) as usize]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1 as usize][used_op2 as usize];
                if curr_sum >= inf {
                    continue;
                }

                let num = nums[i];
                dp[i + 1][used_op1 as usize][used_op2 as usize] =
                    cmp::min(dp[i + 1][used_op1 as usize][used_op2 as usize], curr_sum + num);

                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize] =
                        cmp::min(dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize], curr_sum + new_num);
                }

                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize] =
                        cmp::min(dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize], curr_sum + new_num);
                }

                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            cmp::min(dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize], curr_sum + new_num);
                    } else {
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            cmp::min(dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize], curr_sum + temp_num);
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            cmp::min(dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize], curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum in the last row of the DP table
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = cmp::min(min_sum, dp[n][used_op1 as usize][used_op2 as usize]);
        }
    }

    min_sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: n, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let op1: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let op2: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Read the nums array
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);
}