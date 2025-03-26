use std::io;
use std::cmp;

fn max_value(a: i64, b: i64) -> i64 {
    cmp::max(a, b)
}

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur: i64 = 0;
    let mut keep: Vec<i64> = Vec::with_capacity(n - k + 1);

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep.push(cur);
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans: i64 = i64::MIN;
    let mut max;

    for i in 0..cmp::min(k, n - k + 1) {
        cur = 0;
        max = keep[i];

        for l in (i..n - k + 1).step_by(k) {
            cur += keep[l];

            if cur > max {
                max = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        ans = max_value(ans, max);
    }
    ans
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: usize = k_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let result: i64 = max_subarray_sum(&nums, n, k);
    println!("{}", result);
}