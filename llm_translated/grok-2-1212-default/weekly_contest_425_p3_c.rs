use std::io::{self, BufRead};

const INF: i32 = i32::MAX / 2;

fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let mut dp = vec![vec![vec![INF; op2 + 1]; op1 + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= INF {
                    continue;
                }

                let num = nums[i];
                if dp[i + 1][used_op1][used_op2] > curr_sum + num {
                    dp[i + 1][used_op1][used_op2] = curr_sum + num;
                }

                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    if dp[i + 1][used_op1 + 1][used_op2] > curr_sum + new_num {
                        dp[i + 1][used_op1 + 1][used_op2] = curr_sum + new_num;
                    }
                }

                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    if dp[i + 1][used_op1][used_op2 + 1] > curr_sum + new_num {
                        dp[i + 1][used_op1][used_op2 + 1] = curr_sum + new_num;
                    }
                }

                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        if dp[i + 1][used_op1 + 1][used_op2 + 1] > curr_sum + new_num {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = curr_sum + new_num;
                        }
                    } else {
                        if dp[i + 1][used_op1 + 1][used_op2 + 1] > curr_sum + temp_num {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = curr_sum + temp_num;
                        }
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        if dp[i + 1][used_op1 + 1][used_op2 + 1] > curr_sum + new_num {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = curr_sum + new_num;
                        }
                    }
                }
            }
        }
    }

    let mut min_sum = INF;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            if min_sum > dp[n][used_op1][used_op2] {
                min_sum = dp[n][used_op1][used_op2];
            }
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
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);

    Ok(())
}