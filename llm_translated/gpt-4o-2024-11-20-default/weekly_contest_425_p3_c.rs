use std::cmp::min;
use std::io::{self, BufRead};

fn min_array_sum(nums: Vec<i32>, nums_size: usize, k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums_size;
    let inf = i32::MAX / 2;

    // Initialize 3D DP array
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

                // Case 1: Don't use any operation
                dp[i + 1][used_op1][used_op2] =
                    min(dp[i + 1][used_op1][used_op2], curr_sum + num);

                // Case 2: Use op1 (halve the number)
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] =
                        min(dp[i + 1][used_op1 + 1][used_op2], curr_sum + new_num);
                }

                // Case 3: Use op2 (subtract k)
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] =
                        min(dp[i + 1][used_op1][used_op2 + 1], curr_sum + new_num);
                }

                // Case 4: Use both op1 and op2
                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;

                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + temp_num);
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum in the last row of dp
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

    // Read input: number of elements, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n = parts.next().unwrap().parse::<usize>().unwrap();
    let k = parts.next().unwrap().parse::<i32>().unwrap();
    let op1 = parts.next().unwrap().parse::<usize>().unwrap();
    let op2 = parts.next().unwrap().parse::<usize>().unwrap();

    // Read the nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Call the min_array_sum function
    let result = min_array_sum(nums, n, k, op1, op2);

    // Output the result
    println!("{}", result);
}