use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt = vec![0; 100_003];
    let mut sum = vec![0; 100_003];
    let mut ans = 0i64;

    for &mut num in nums.iter_mut() {
        let x = (num + 1) as usize;
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;

        cnt[x] = (cnt[x] + c) % MOD;
        sum[x] = (sum[x] + s) % MOD;
        ans = (ans + s) % MOD;
    }

    ans as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the number of elements matches the size
    assert_eq!(nums.len(), n);

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}