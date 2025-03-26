use std::io;

fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let INF = i32::MAX / 2;

    // Initialize DP table with dimensions (n+1) x (op1+1) x (op2+1)
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

                // Option 1: Do not apply any operation
                if dp[i + 1][used_op1][used_op2] > curr_sum + num {
                    dp[i + 1][used_op1][used_op2] = curr_sum + num;
                }

                // Option 2: Apply op1 (if possible)
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    let new_used_op1 = used_op1 + 1;
                    if dp[i + 1][new_used_op1][used_op2] > curr_sum + new_num {
                        dp[i + 1][new_used_op1][used_op2] = curr_sum + new_num;
                    }
                }

                // Option 3: Apply op2 (if possible)
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    let new_used_op2 = used_op2 + 1;
                    if dp[i + 1][used_op1][new_used_op2] > curr_sum + new_num {
                        dp[i + 1][used_op1][new_used_op2] = curr_sum + new_num;
                    }
                }

                // Option 4: Apply both op1 and op2 (both are possible)
                if used_op1 < op1 && used_op2 < op2 {
                    // Case 1: Apply op1 first then op2 (if applicable)
                    let temp_num = (num + 1) / 2;
                    let new_used_op1 = used_op1 + 1;
                    let new_used_op2 = used_op2 + 1;
                    let new_num1 = if temp_num >= k {
                        temp_num - k
                    } else {
                        temp_num
                    };
                    if dp[i + 1][new_used_op1][new_used_op2] > curr_sum + new_num1 {
                        dp[i + 1][new_used_op1][new_used_op2] = curr_sum + new_num1;
                    }

                    // Case 2: Apply op2 first then op1 (if applicable)
                    if num >= k {
                        let temp_num2 = num - k;
                        let new_num2 = (temp_num2 + 1) / 2;
                        if dp[i + 1][new_used_op1][new_used_op2] > curr_sum + new_num2 {
                            dp[i + 1][new_used_op1][new_used_op2] = curr_sum + new_num2;
                        }
                    }
                }
            }
        }
    }

    // Find the minimum sum among all possible used_op1 and used_op2
    let mut min_sum = INF;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            if dp[n][used_op1][used_op2] < min_sum {
                min_sum = dp[n][used_op1][used_op2];
            }
        }
    }

    min_sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();
    let op1: usize = tokens.next().unwrap().parse().unwrap();
    let op2: usize = tokens.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = tokens.map(|s| s.parse().unwrap()).collect();
    assert_eq!(nums.len(), n);

    let result = min_array_sum(&nums, k, op1, op2);
    println!("{}", result);
}