use std::io;

fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;

    // Initialize the DP table
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

    // Find the minimum sum
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            if min_sum > dp[n][used_op1][used_op2] {
                min_sum = dp[n][used_op1][used_op2];
            }
        }
    }

    min_sum
}

fn main() {
    // Read input: number of elements, k, op1, op2
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");
    let op1: usize = iter.next().unwrap().parse().expect("Failed to parse op1");
    let op2: usize = iter.next().unwrap().parse().expect("Failed to parse op2");

    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Failed to parse num");
        nums.push(num);
    }

    // Call the min_array_sum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);
}