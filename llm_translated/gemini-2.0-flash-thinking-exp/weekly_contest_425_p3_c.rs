use std::io;
use std::cmp::min;

fn min_array_sum(nums: &[i32], k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let inf = i32::MAX / 2; // Representing infinity

    // Initialize the 3D DP table using Vec in Rust
    let mut dp = vec![vec![vec![inf; (op2 + 1) as usize]; (op1 + 1) as usize]; (n + 1) as usize];
    dp[0][0][0] = 0;

    for i in 0..n {
        for used_op1 in 0..=op1 {
            for used_op2 in 0..=op2 {
                let curr_sum = dp[i][used_op1 as usize][used_op2 as usize];
                if curr_sum >= inf {
                    continue; // Skip if the current state is unreachable
                }

                let num = nums[i];

                // Option 1: No operation on current number
                dp[i + 1][used_op1 as usize][used_op2 as usize] = min(
                    dp[i + 1][used_op1 as usize][used_op2 as usize],
                    curr_sum + num,
                );

                // Option 2: Apply operation 1 (divide by 2) if possible
                if used_op1 < op1 {
                    let new_num = (num + 1) / 2;
                    dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize] = min(
                        dp[i + 1][(used_op1 + 1) as usize][used_op2 as usize],
                        curr_sum + new_num,
                    );
                }

                // Option 3: Apply operation 2 (subtract k) if possible
                if used_op2 < op2 && num >= k {
                    let new_num = num - k;
                    dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize] = min(
                        dp[i + 1][used_op1 as usize][(used_op2 + 1) as usize],
                        curr_sum + new_num,
                    );
                }

                // Option 4: Apply both operation 1 and operation 2 if possible
                if used_op1 < op1 && used_op2 < op2 {
                    let temp_num = (num + 1) / 2;
                    if temp_num >= k {
                        let new_num = temp_num - k;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] = min(
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize],
                            curr_sum + new_num,
                        );
                    } else {
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] = min(
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize],
                            curr_sum + temp_num,
                        );
                    }

                    if (num >= k) {
                        let temp_num = num - k;
                        let new_num = (temp_num + 1) / 2;
                        dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize] = min(
                            dp[i + 1][(used_op1 + 1) as usize][(used_op2 + 1) as usize],
                            curr_sum + new_num,
                        );
                    }
                }
            }
        }
    }

    let mut min_sum = inf;
    for used_op1 in 0..=op1 {
        for used_op2 in 0..=op2 {
            min_sum = min(min_sum, dp[n][used_op1 as usize][used_op2 as usize]);
        }
    }

    min_sum
}

fn main() -> io::Result<()> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let mut parts = input_line.trim().split_whitespace();

    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let op1: i32 = parts.next().unwrap().parse().unwrap();
    let op2: i32 = parts.next().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str)?;
        let num: i32 = num_str.trim().parse().unwrap();
        nums.push(num);
    }

    let result = min_array_sum(&nums, k, op1, op2);
    println!("{}", result);

    Ok(())
}