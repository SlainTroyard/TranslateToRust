use std::io::{self, BufRead};
use std::cmp::max;

/// Calculate the greatest common divisor (gcd) using Euclidean algorithm
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculate the least common multiple (lcm) using the formula lcm(a,b) = (a*b)/gcd(a,b)
fn lcm(a: i64, b: i32) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * (b as i64) / (gcd(a as i32, b) as i64)
    }
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];
        
        // Compute suffix gcd and lcm
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i]);
        }

        // Initial answer (without removing any element)
        let mut ans = (suf_gcd[0] as i64) * suf_lcm[0];
        
        let mut pre_gcd = 0;
        let mut pre_lcm: i64 = 1;
        
        // Try removing each element and calculate the score
        for i in 0..n {
            ans = max(ans, (gcd(pre_gcd, suf_gcd[i + 1]) as i64) * lcm(pre_lcm, suf_lcm[i + 1] as i32));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }
        
        ans
    }
}

fn main() {
    // Read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Parse the nums array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure nums has the correct length
    assert_eq!(nums.len(), n);
    
    // Calculate and print the result
    let solution = Solution;
    println!("{}", Solution::max_score(nums));
}