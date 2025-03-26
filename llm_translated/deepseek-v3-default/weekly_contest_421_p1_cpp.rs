use std::io;
use std::cmp::{max, min};
use std::collections::VecDeque;

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    a * (b / gcd(a as i32, b as i32) as i64)
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];
        
        // Calculate suffix GCD and LCM
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        let mut ans = suf_gcd[0] as i64 * suf_lcm[0]; // Case when no element is removed
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        // Enumerate removing nums[i]
        for i in 0..n {
            ans = max(ans, gcd(pre_gcd, suf_gcd[i + 1]) as i64 * lcm(pre_lcm, suf_lcm[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let solution = Solution;
    println!("{}", solution.max_score(nums));
}