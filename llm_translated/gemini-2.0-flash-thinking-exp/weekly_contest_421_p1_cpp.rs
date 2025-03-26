use std::io;
use std::vec::Vec;
use std::cmp;
use num::integer;

fn max_score(nums: &Vec<i32>) -> i64 {
    let n = nums.len();
    let mut suf_gcd = vec![0; n + 1];
    let mut suf_lcm = vec![0_i64; n + 1];
    suf_lcm[n] = 1;

    // Calculate suffix GCD and LCM
    for i in (0..n).rev() {
        suf_gcd[i] = integer::gcd(suf_gcd[i + 1], nums[i]);
        suf_lcm[i] = integer::lcm(suf_lcm[i + 1], nums[i] as i64);
    }

    let mut ans = (suf_gcd[0] as i64) * suf_lcm[0]; // Score without removing any element
    let mut pre_gcd = 0;
    let mut pre_lcm = 1_i64;

    // Iterate through the array and consider removing each element
    for i in 0..n {
        ans = cmp::max(ans, (integer::gcd(pre_gcd, suf_gcd[i + 1]) as i64) * integer::lcm(pre_lcm, suf_lcm[i + 1]));
        pre_gcd = integer::gcd(pre_gcd, nums[i]);
        pre_lcm = integer::lcm(pre_lcm, nums[i] as i64);
    }
    return ans;
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nums_str = input_line.trim().split_whitespace();
    for s in nums_str {
        nums.push(s.parse().unwrap());
    }

    println!("{}", max_score(&nums));
}