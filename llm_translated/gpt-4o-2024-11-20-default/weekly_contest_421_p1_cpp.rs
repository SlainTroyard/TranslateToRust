use std::io;
use std::cmp::max;
use std::collections::VecDeque;

/// Helper function to compute the greatest common divisor (GCD) of two numbers
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Helper function to compute the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i32) -> i64 {
    let gcd_value = gcd(a as i32, b);
    (a / gcd_value as i64) * b as i64
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        // Prepare suffix arrays for GCD and LCM
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];
        
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i]);
        }
        
        // Compute the initial answer without removing any elements
        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];

        // Prefix GCD and LCM to keep track during iterations
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        // Iterate through the elements, considering the removal of nums[i]
        for i in 0..n {
            ans = max(
                ans,
                gcd(pre_gcd, suf_gcd[i + 1]) as i64 * lcm(pre_lcm, suf_lcm[i + 1]),
            );
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the number of elements
    let n: usize = input.trim().parse().expect("Failed to parse n");
    let mut nums = vec![0; n];

    // Read the next line for array values
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums");

    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    nums.copy_from_slice(&values);

    // Call the solution and print the result
    let solution = Solution;
    let result = solution.max_score(nums);

    // Output the result
    println!("{}", result);
}