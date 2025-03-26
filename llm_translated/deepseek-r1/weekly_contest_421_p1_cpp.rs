use num_integer::Integer;
use std::io::{self, Read};
use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn max_score(&self, nums: Vec<i32>) -> i64 {
        let n = nums.len();
        // Initialize suffix arrays for gcd and lcm
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![0i64; n + 1];
        suf_lcm[n] = 1;

        // Compute suffix gcd and lcm from the end to start
        for i in (0..n).rev() {
            suf_gcd[i] = i32::gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = i64::lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        // Initial answer without removing any elements
        let mut ans = (suf_gcd[0] as i64) * suf_lcm[0];
        let mut pre_gcd = 0;    // Prefix gcd initialized to 0 (identity for gcd)
        let mut pre_lcm = 1i64; // Prefix lcm initialized to 1 (identity for lcm)

        // Iterate through each possible element to remove
        for i in 0..n {
            // Calculate combined gcd and lcm when removing current element
            let current_gcd = i32::gcd(pre_gcd, suf_gcd[i + 1]);
            let current_lcm = i64::lcm(pre_lcm, suf_lcm[i + 1]);
            ans = ans.max(current_gcd as i64 * current_lcm);

            // Update prefix gcd and lcm to include current element
            pre_gcd = i32::gcd(pre_gcd, nums[i]);
            pre_lcm = i64::lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Read input size and numbers
    let n = tokens
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Invalid input format");
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let solution = Solution;
    println!("{}", solution.max_score(nums));
}