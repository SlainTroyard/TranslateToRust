use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

struct Solution;

impl Solution {
    fn max_score(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];

        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1] as i64, nums[i] as i64) as i32;
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        for i in 0..n {
            ans = ans.max(gcd(pre_gcd, suf_gcd[i + 1] as i64) * lcm(pre_lcm, suf_lcm[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i] as i64);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let solution = Solution;
    println!("{}", solution.max_score(&nums));

    Ok(())
}