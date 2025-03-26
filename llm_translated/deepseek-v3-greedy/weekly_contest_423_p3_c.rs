use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i64 {
    let mut cnt = vec![0i64; 100_003];
    let mut sum = vec![0i64; 100_003];
    let mut ans = 0i64;

    for &mut num in nums.iter_mut() {
        let x = num + 1;
        let c = (cnt[(x - 1) as usize] + 1 + cnt[(x + 1) as usize]) % MOD;
        cnt[x as usize] = (cnt[x as usize] + c) % MOD;
        let s = (c * (x - 1) as i64 + sum[(x - 1) as usize] + sum[(x + 1) as usize]) % MOD;
        sum[x as usize] = (sum[x as usize] + s) % MOD;
        ans = (ans + s) % MOD;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let mut nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}