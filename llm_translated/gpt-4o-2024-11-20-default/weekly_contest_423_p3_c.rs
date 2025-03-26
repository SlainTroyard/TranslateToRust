use std::io::{self, Write};
use std::cmp::min;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut Vec<i32>, nums_size: usize) -> i64 {
    let mut cnt = vec![0i64; 100_003];
    let mut sum = vec![0i64; 100_003];
    let mut ans = 0i64;

    for i in 0..nums_size {
        let x = {
            nums[i] += 1; // Equivalent to x = ++nums[i]
            nums[i] as usize
        };

        let c = (cnt[x.saturating_sub(1)] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;

        let s = (c * ((x - 1) as i64) + sum[x.saturating_sub(1)] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    // Read the elements of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    if nums.len() != n {
        eprintln!("Unexpected input length!");
        std::process::exit(1);
    }

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums, n);
    println!("{}", result);
}