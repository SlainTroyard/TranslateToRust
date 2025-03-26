// Weekly Contest 421 Problem 1 in Rust
// Translated from the provided C++ code

use std::io::{self, Read};

// Helper function to compute the greatest common divisor (GCD)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

// Helper function to compute the least common multiple (LCM)
// Watch out for potential overflow in real usage
fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

// A struct mirroring the C++ "Solution" class
struct Solution;

impl Solution {
    // Translated from C++: long long maxScore(vector<int>& nums)
    fn maxScore(&self, nums: &[i64]) -> i64 {
        let n = nums.len();

        // Suffix arrays for GCD and LCM
        let mut suf_gcd = vec![0i64; n + 1];
        let mut suf_lcm = vec![1i64; n + 1];
        suf_gcd[n] = 0;
        suf_lcm[n] = 1;

        // Build suffix GCD and LCM
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i]);
        }

        // 不移除元素 (no elements removed)
        let mut ans = suf_gcd[0] * suf_lcm[0];

        // 枚举移除 nums[i] (consider removing nums[i])
        let mut pre_gcd = 0i64;
        let mut pre_lcm = 1i64;
        for i in 0..n {
            ans = ans.max(gcd(pre_gcd, suf_gcd[i + 1]) * lcm(pre_lcm, suf_lcm[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input from stdin (like "cin >> n" then "cin >> nums[i]")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // First integer: n
    let n: usize = tokens
        .next()
        .ok_or("Expected an integer for n")?
        .parse()?;

    // Next n integers: nums
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let val: i64 = tokens
            .next()
            .ok_or("Expected an integer in nums")?
            .parse()?;
        nums.push(val);
    }

    // Create a solution instance and compute the answer
    let solution = Solution;
    let result = solution.maxScore(&nums);

    // Print the result to stdout (like "cout << ... << endl")
    println!("{}", result);
    Ok(())
}