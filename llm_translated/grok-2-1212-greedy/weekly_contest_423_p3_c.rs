use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i64 {
    let mut cnt = vec![0; 100003];
    let mut sum = vec![0; 100003];
    let mut ans = 0;

    for i in 0..nums.len() {
        let x = nums[i] + 1;
        let c = cnt[(x - 1) as usize] + 1 + cnt[(x + 1) as usize];
        cnt[x as usize] = (cnt[x as usize] + c) % MOD;
        let s = c * (x - 1) + sum[(x - 1) as usize] + sum[(x + 1) as usize];
        sum[x as usize] = (sum[x as usize] + s) % MOD;
        ans = (ans + s) % MOD;
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the elements of the array
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);

    Ok(())
}