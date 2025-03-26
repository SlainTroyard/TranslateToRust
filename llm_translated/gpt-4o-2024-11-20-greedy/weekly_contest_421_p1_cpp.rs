use std::io::{self, BufRead};
use std::cmp::max;
use std::collections::VecDeque;

// Helper function to calculate the greatest common divisor (GCD)
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Helper function to calculate the least common multiple (LCM)
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a as i32, b as i32) as i64) * b
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];

        // Calculate suffix GCD and LCM arrays
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        // Initialize the answer with the case where no elements are removed
        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        // Iterate over the array to consider removing each element
        for i in 0..n {
            ans = max(
                ans,
                gcd(pre_gcd, suf_gcd[i + 1]) as i64 * lcm(pre_lcm, suf_lcm[i + 1]),
            );
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse the size of the array");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n, "Input size does not match the expected size");

    // Solve the problem and print the result
    let solution = Solution;
    println!("{}", solution.max_score(nums));
}