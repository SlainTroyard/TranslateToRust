use std::io::{self, Write};

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Initialize the DP table with INF
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

                // Case 1: Do not use any operation
                dp[i + 1][used_op1 as usize][used_op2 as usize] =
                    dp[i + 1][used_op1 as usize][used_op2 as usize].min(curr_sum + num);

                // Case 2: Use operation 1
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize] =
                        dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize].min(curr_sum + new_num);
                }

                // Case 3: Use operation 2
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize] =
                        dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize].min(curr_sum + new_num);
                }

                // Case 4: Use both operations
                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize].min(curr_sum + new_num);
                    } else {
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize].min(curr_sum + temp_num);
                    }

                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] =
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize].min(curr_sum + new_num);
                    }
                }
            }
        }
    }

    // Find the minimum sum in the last layer of the DP table
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = min_sum.min(dp[n][used_op1 as usize][used_op2 as usize]);
        }
    }

    min_sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: i32 = iter.next().unwrap().parse().unwrap();
    let op2: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num_str in input.split_whitespace() {
        nums.push(num_str.parse().unwrap());
    }

    let result = min_array_sum(&nums, k, op1, op2);
    println!("{}", result);
}