fn min_array_sum(nums: &[i32], n: usize, k: i32, op1: usize, op2: usize) -> i32 {
    let INF = i32::MAX / 2;
    let n_plus_1 = n + 1;
    let op1_plus_1 = op1 + 1;
    let op2_plus_1 = op2 + 1;

    // Initialize the DP array with INF
    let mut dp = vec![vec![vec![INF; op2_plus_1]; op1_plus_1]; n_plus_1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= INF {
                    continue;
                }

                let num = nums[i];
                // Option 1: Do nothing
                if dp[i+1][used_op1][used_op2] > curr_sum + num {
                    dp[i+1][used_op1][used_op2] = curr_sum + num;
                }

                // Option 2: Apply operation 1 (halve)
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    if dp[i+1][used_op1 + 1][used_op2] > curr_sum + new_num {
                        dp[i+1][used_op1 + 1][used_op2] = curr_sum + new_num;
                    }
                }

                // Option 3: Apply operation 2 (subtract k)
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    if dp[i+1][used_op1][used_op2 + 1] > curr_sum + new_num {
                        dp[i+1][used_op1][used_op2 + 1] = curr_sum + new_num;
                    }
                }

                // Option 4: Apply both operations
                if used_op1 < op1 && used_op2 < op2 {
                    // Case 1: Halve first, then subtract if possible
                    let temp_num = (num + 1) / 2;
                    let new_num = if temp_num >= k {
                        temp_num - k
                    } else {
                        temp_num
                    };
                    if dp[i+1][used_op1 + 1][used_op2 + 1] > curr_sum + new_num {
                        dp[i+1][used_op1 + 1][used_op2 + 1] = curr_sum + new_num;
                    }

                    // Case 2: Subtract first, then halve if possible
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        if dp[i+1][used_op1 + 1][used_op2 + 1] > curr_sum + new_num {
                            dp[i+1][used_op1 + 1][used_op2 + 1] = curr_sum + new_num;
                        }
                    }
                }
            }
        }
    }

    // Find the minimum sum in the last layer
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
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let n = tokens[0].parse::<usize>().unwrap();
    let k = tokens[1].parse::<i32>().unwrap();
    let op1 = tokens[2].parse::<usize>().unwrap();
    let op2 = tokens[3].parse::<usize>().unwrap();

    let nums = tokens[4..4 + n]
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = min_array_sum(&nums, n, k, op1, op2);
    println!("{}", result);
}