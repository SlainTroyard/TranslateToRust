use std::io::{self, BufRead};

fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Initialize 3D vector for dynamic programming
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
                dp[i + 1][used_op1][used_op2] = dp[i + 1][used_op1][used_op2].min(curr_sum + num);

                // Operation 1: Divide by 2 and round up
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] = dp[i + 1][used_op1 + 1][used_op2].min(curr_sum + new_num);
                }

                // Operation 2: Subtract k
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] = dp[i + 1][used_op1][used_op2 + 1].min(curr_sum + new_num);
                }

                // Both operations
                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + temp_num);
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = min_sum.min(dp[n][used_op1][used_op2]);
        }
    }

    min_sum
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input: number of elements, k, op1, op2
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: usize = iter.next().unwrap().parse().unwrap();
    let op2: usize = iter.next().unwrap().parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);

    Ok(())
}