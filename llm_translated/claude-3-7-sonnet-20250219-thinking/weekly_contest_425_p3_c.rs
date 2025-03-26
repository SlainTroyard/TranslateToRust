use std::io::{self, BufRead};
use std::cmp;

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2;
    
    // Create 3D DP array
    let mut dp = vec![vec![vec![inf; (op2 + 1) as usize]; (op1 + 1) as usize]; n + 1];
    dp[0][0][0] = 0;
    
    for i in 0..n {
        for used_op1 in 0..=op1 as usize {
            for used_op2 in 0..=op2 as usize {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= inf {
                    continue;
                }
                
                let num = nums[i];
                
                // Don't use any operation
                dp[i + 1][used_op1][used_op2] = cmp::min(dp[i + 1][used_op1][used_op2], curr_sum + num);
                
                // Use operation 1: (num+1)/2
                if used_op1 < op1 as usize {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] = cmp::min(dp[i + 1][used_op1 + 1][used_op2], curr_sum + new_num);
                }
                
                // Use operation 2: num - k (if num >= k)
                if used_op2 < op2 as usize && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] = cmp::min(dp[i + 1][used_op1][used_op2 + 1], curr_sum + new_num);
                }
                
                // Use both operations in different orders
                if used_op1 < op1 as usize && used_op2 < op2 as usize {
                    // Operation 1 then Operation 2
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + temp_num);
                    }
                    
                    // Operation 2 then Operation 1
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] = cmp::min(dp[i + 1][used_op1 + 1][used_op2 + 1], curr_sum + new_num);
                    }
                }
            }
        }
    }
    
    // Find minimum sum
    let mut min_sum = inf;
    for used_op1 in 0..=op1 as usize {
        for used_op2 in 0..=op2 as usize {
            min_sum = cmp::min(min_sum, dp[n][used_op1][used_op2]);
        }
    }
    
    min_sum
}

fn main() {
    // Read input: number of elements, k, op1, op2
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0] as usize;
    let k = parts[1];
    let op1 = parts[2];
    let op2 = parts[3];
    
    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums");
    for num_str in input.trim().split_whitespace().take(n) {
        nums.push(num_str.parse::<i32>().unwrap());
    }
    
    // Call the minArraySum function
    let result = min_array_sum(&nums, k, op1, op2);

    // Output the result
    println!("{}", result);
}