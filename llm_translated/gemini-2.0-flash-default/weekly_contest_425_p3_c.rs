use std::io;
use std::io::Read;
use std::cmp::min;

const INF: i32 = i32::MAX / 2;

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let op1_usize = op1 as usize;
    let op2_usize = op2 as usize;

    // Create a 3D vector (dp) initialized with INF
    let mut dp = vec![vec![vec![INF; (op2_usize + 1)]; (op1_usize + 1)]; (n + 1)];

    // Initialize the base case
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1_usize {
            for used_op2 in 0..=op2_usize {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= INF {
                    continue;
                }

                let num = nums[i];
                if dp[i + 1][used_op1][used_op2] > curr_sum + num {
                    dp[i + 1][used_op1][used_op2] = curr_sum + num;
                }

                if used_op1 < op1_usize {
                    let new_num = (num + 1) / 2;
                    if dp[i + 1][used_op1 + 1][used_op2] > curr_sum + new_num {
                        dp[i + 1][used_op1 + 1][used_op2] = curr_sum + new_num;
                    }
                }

                if used_op2 < op2_usize && num >= k {
                    let new_num = num - k;
                    if dp[i + 1][used_op1][used_op2 + 1] > curr_sum + new_num {
                        dp[i + 1][used_op1][used_op2 + 1] = curr_sum + new_num;
                    }
                }

                if used_op1 < op1_usize && used_op2 < op2_usize {
                    let mut temp_num = (num + 1) / 2;
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
                        temp_num = num - k;
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
    for used_op1 in 0..=op1_usize {
        for used_op2 in 0..=op2_usize {
            min_sum = min(min_sum, dp[n][used_op1][used_op2]);
        }
    }

    min_sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: i32 = first_line_iter.next().unwrap().parse()?;
    let k: i32 = first_line_iter.next().unwrap().parse()?;
    let op1: i32 = first_line_iter.next().unwrap().parse()?;
    let op2: i32 = first_line_iter.next().unwrap().parse()?;

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = min_array_sum(&nums, k, op1, op2);

    println!("{}", result);

    Ok(())
}