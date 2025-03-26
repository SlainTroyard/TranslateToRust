use std::io::{self, BufRead};
use std::cmp::max;

// Helper function to calculate GCD (Greatest Common Divisor)
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Helper function to calculate LCM (Least Common Multiple)
fn lcm(a: i64, b: i32) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / (gcd(a as i32, b) as i64) * (b as i64)
    }
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1i64; n + 1];
        
        // Calculate suffix GCD and LCM
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i]);
        }

        // Initial answer: not removing any element
        let mut ans = (suf_gcd[0] as i64) * suf_lcm[0];
        
        let mut pre_gcd = 0;
        let mut pre_lcm = 1i64;
        
        // Try removing each element and find the maximum score
        for i in 0..n {
            ans = max(ans, (gcd(pre_gcd, suf_gcd[i + 1]) as i64) * lcm(pre_lcm, suf_gcd[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.max_score(nums));
}