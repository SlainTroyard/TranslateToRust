use std::cmp::max;
use std::io;

fn max_value(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur = 0;
    let mut keep = vec![0i64; n - k + 1];

    // Precompute sums of subarrays of length `k`
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;

    // Compute the maximum subarray sum considering chunks of `k`
    for i in 0..k.min(n - k + 1) {
        cur = 0;
        let mut max_in_chunk = keep[i];

        for l in (i..n - k + 1).step_by(k) {
            cur += keep[l];

            if cur > max_in_chunk {
                max_in_chunk = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }

        ans = max_value(ans, max_in_chunk);
    }

    ans
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read `n` and `k`
    stdin.read_line(&mut input).unwrap();
    let n_k: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    // Read the array `nums`
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Solve the problem
    let result = max_subarray_sum(&nums, n, k);

    // Print the result
    println!("{}", result);
}