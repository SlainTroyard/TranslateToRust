use std::io;

const MOD: i64 = 1_000_000_007;
const MAX: usize = 100003;

pub fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt = vec![0_i64; MAX];
    let mut sum = vec![0_i64; MAX];
    let mut ans = 0_i64;

    for num in nums.iter_mut() {
        *num += 1;
        let x = *num as usize;

        // Compute c and s, ensuring indices are within bounds
        // The problem constraints ensure x is within valid indices
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;

        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans as i32
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let n: usize = tokens[0].parse().unwrap();
    let nums: Vec<i32> = tokens[1..=n]
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    let mut nums = nums;
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}