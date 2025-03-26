use std::error::Error;
use std::io::{self, Read};

/// Computes the minimum possible sum after applying available operations
/// on every element of `nums` as described in the problem.
/// 
/// # Arguments
/// 
/// * `nums` - Slice of numbers.
/// * `k` - Subtraction constant for op2.
/// * `op1` - Number of times you can perform the halving operation (op1).
/// * `op2` - Number of times you can perform the subtraction operation (op2).
fn min_array_sum(nums: &[i32], k: i32, op1: usize, op2: usize) -> i32 {
    let n = nums.len();
    // Use a large constant (half of i32 max) as INF.
    let inf = i32::MAX / 2;

    // Create a 3D vector dp with dimensions (n+1) x (op1+1) x (op2+1)
    // Initialize all entries with 'inf'. dp[i][j][l] represents the minimal sum 
    // achievable after processing i elements using j times op1 and l times op2.
    let mut dp = vec![vec![vec![inf; op2 + 1]; op1 + 1]; n + 1];
    dp[0][0][0] = 0;

    // Process each element in nums.
    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1][used_op2];
                if curr_sum >= inf {
                    continue;
                }
                let num = nums[i];

                // Option 0: Do not use any operation on num.
                dp[i + 1][used_op1][used_op2] = dp[i + 1][used_op1][used_op2].min(curr_sum + num);

                // Option 1: Use op1 only (halving operation).
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][used_op1 + 1][used_op2] =
                        dp[i + 1][used_op1 + 1][used_op2].min(curr_sum + new_num);
                }

                // Option 2: Use op2 only (subtraction operation).
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1][used_op2 + 1] =
                        dp[i + 1][used_op1][used_op2 + 1].min(curr_sum + new_num);
                }

                // Option 3: Use both op1 and op2.
                if used_op1 < op1 && used_op2 < op2 {
                    // First branch: apply op1 first, then op2.
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + new_num);
                    } else {
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + temp_num);
                    }

                    // Second branch: apply op2 first, then op1.
                    if num >= k {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][used_op1 + 1][used_op2 + 1] =
                            dp[i + 1][used_op1 + 1][used_op2 + 1].min(curr_sum + new_num);
                    }
                }
            }
        }
    }

    // After processing all numbers, find the minimum sum among all valid operation counts.
    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = min_sum.min(dp[n][used_op1][used_op2]);
        }
    }
    min_sum
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire standard input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and parse tokens.
    let mut tokens = input.split_whitespace();

    // Read the first four integers: n, k, op1, op2.
    // They are provided in the beginning of the input.
    let n: usize = tokens
        .next()
        .ok_or("Missing n")?
        .parse()
        .map_err(|_| "Invalid n")?;
    let k: i32 = tokens
        .next()
        .ok_or("Missing k")?
        .parse()
        .map_err(|_| "Invalid k")?;
    let op1: usize = tokens
        .next()
        .ok_or("Missing op1")?
        .parse()
        .map_err(|_| "Invalid op1")?;
    let op2: usize = tokens
        .next()
        .ok_or("Missing op2")?
        .parse()
        .map_err(|_| "Invalid op2")?;

    // Read the array of numbers.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens
            .next()
            .ok_or("Missing number in nums array")?
            .parse()
            .map_err(|_| "Invalid number in nums array")?;
        nums.push(num);
    }

    // Compute the minimal possible sum using the provided operations.
    let result = min_array_sum(&nums, k, op1, op2);

    // Print the result exactly the same way as the original code.
    println!("{}", result);
    Ok(())
}