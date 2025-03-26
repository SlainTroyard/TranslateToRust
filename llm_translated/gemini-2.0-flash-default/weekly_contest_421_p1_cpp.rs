use num::integer::lcm;
use std::io;

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution {}

impl Solution {
    fn max_score(&self, nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd: Vec<i64> = vec![0; n + 1];
        let mut suf_lcm: Vec<i64> = vec![0; n + 1];
        suf_lcm[n] = 1;

        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i] as i64);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        let mut ans = suf_gcd[0] * suf_lcm[0]; // 不移除元素
        let mut pre_gcd: i64 = 0;
        let mut pre_lcm: i64 = 1;

        for i in 0..n {
            // 枚举移除 nums[i]
            ans = std::cmp::max(
                ans,
                gcd(pre_gcd, suf_gcd[i + 1]) * lcm(pre_lcm, suf_lcm[i + 1]),
            );
            pre_gcd = gcd(pre_gcd, nums[i] as i64);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_score(&nums));
}