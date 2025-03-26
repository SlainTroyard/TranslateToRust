use std::io::{self, BufRead};

fn max_value(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur: i64 = 0;
    let mut keep = vec![0; n - k + 1];

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;

    for i in 0..k.min(n - k + 1) {
        cur = 0;
        let mut max = keep[i];

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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array and the value of k
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);
}