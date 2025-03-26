use std::io;
use std::io::Read;

const MOD: i64 = 1_000_000_007;
const MAXN: usize = 100003;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt: [i64; MAXN] = [0; MAXN];
    let mut sum: [i64; MAXN] = [0; MAXN];
    let mut ans: i64 = 0;

    for i in 0..nums.len() {
        nums[i] += 1;
        let x = nums[i] as usize;

        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;

        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();

    let nums_str = lines.next().unwrap();
    let nums: Vec<i32> = nums_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut nums_mut = nums.clone(); // Create a mutable copy
    let result = sum_of_good_subsequences(&mut nums_mut);
    println!("{}", result);
}