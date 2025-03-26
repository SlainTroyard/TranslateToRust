use std::num::integer::gcd;
use std::num::integer::lcm;
use std::io;

struct Solution {}

impl Solution {
    fn max_score(&self, nums: Vec<i64>) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];
        
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(nums[i], suf_gcd[i + 1]);
            suf_lcm[i] = lcm(nums[i], suf_lcm[i + 1]);
        }
        
        let mut ans = suf_gcd[0] * suf_lcm[0];
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;
        
        for i in 0..n {
            let current_gcd = gcd(pre_gcd, suf_gcd[i + 1]);
            let current_lcm = lcm(pre_lcm, suf_lcm[i + 1]);
            let current_score = current_gcd * current_lcm;
            if current_score > ans {
                ans = current_score;
            }
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }
        
        ans
    }
}

fn read_i64() -> Result<i64, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn read_vec_i64() -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    Ok(nums)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_i64()?;
    let nums = read_vec_i64()?;
    if nums.len() != n as usize {
        eprintln!("Expected {} numbers, got {}", n, nums.len());
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")));
    }
    let solution = Solution {};
    let ans = solution.max_score(nums);
    println!("{}", ans);
    Ok(())
}