use std::io;

const MOD: i64 = 1000000007;

fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
    let mut cnt = vec![0; 100003];
    let mut sum = vec![0; 100003];
    let mut ans = 0;

    for &num in nums {
        let x = (num + 1) as usize;
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;
        let s = (c * (x - 1) as i64 + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;
        ans = (ans + s) % MOD;
    }

    ans as i32
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    let result = sum_of_good_subsequences(&nums);
    println!("{}", result);
}