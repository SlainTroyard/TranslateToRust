use std::io::{self, BufRead};

fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let op1 = op1 as usize;
    let op2 = op2 as usize;
    const INF: i32 = i32::MAX / 2;

    // Initialize 3D DP array with INF and set starting state
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

                // No operation
                if dp[i + 1][used_op1][used_op2] > curr_sum + num {
                    dp[i + 1][used_op1][used_op2] = curr_sum + num;
                }

                // Apply op1 if possible
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    if dp[i + 1][used_op1 + 1][used_op2] > curr_sum + new_num {
                        dp[i + 1][used_op1 + 1][used_op2] = curr_sum + new_num;
                    }
                }

                // Apply op2 if possible (num >= k)
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    if dp[i + 1][used_op1][used_op2 + 1] > curr_sum + new_num {
                        dp[i + 1][used_op1][used_op2 + 1] = curr_sum + new_num;
                    }
                }

                // Apply both op1 and op2 in different orders if possible
                if used_op1 < op1 && used_op2 < op2 {
                    // Apply op1 then op2
                    let temp_num = (num + 1) / 2;
                    let new_num = if temp_num >= k {
                        temp_num - k
                    } else {
                        temp_num
                    };
                    let next_sum = curr_sum + new_num;
                    if dp[i + 1][used_op1 + 1][used_op2 + 1] > next_sum {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = next_sum;
                    }

                    // Apply op2 then op1 if possible (num >= k)
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        let next_sum = curr_sum + new_num;
                        if dp[i + 1][used_op1 + 1][used_op2 + 1] > next_sum {
                            dp[i + 1][used_op1 + 1][used_op2 + 1] = next_sum;
                        }
                    }
                }
            }
        }
    }

    // Find the minimal sum in the final state
    dp[n]
        .iter()
        .flat_map(|row| row.iter())
        .fold(INF, |acc, &x| acc.min(x))
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap();
    let op1 = tokens.next().unwrap();
    let op2 = tokens.next().unwrap();

    let nums: Vec<i32> = tokens.take(n).collect();

    let result = min_array_sum(nums, k, op1, op2);
    println!("{}", result);
}