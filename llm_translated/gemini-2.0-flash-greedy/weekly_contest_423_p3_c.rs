use std::io;
use std::io::Read;

const MOD: i64 = 1_000_000_007;
const MAX_SIZE: usize = 100003;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt = [0i64; MAX_SIZE];
    let mut sum = [0i64; MAX_SIZE];
    let mut ans: i64 = 0;

    for i in 0..nums.len() {
        let x = (nums[i] + 1) as usize; // Offset by 1 as in original C code

        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;

        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let nums_str = lines.next().unwrap();
    let nums: Vec<i32> = nums_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut nums_mut = nums.clone();
    let result = sum_of_good_subsequences(&mut nums_mut);
    println!("{}", result);

    Ok(())
}