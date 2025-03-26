use std::io::{self, BufRead};
use std::cmp::{max, min};
use std::i64;

fn max_value(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur: i64 = 0;
    let mut keep = vec![0i64; n - k + 1];

    // Calculate the sum of all subarrays of size `k` and store them in `keep`
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;

    // Iterate over possible starting points for the subarrays
    for i in 0..min(k, n - k + 1) {
        cur = 0;
        let mut max_val = keep[i];

        // Calculate the maximum subarray sum for the current starting point
        for l in (i..n - k + 1).step_by(k) {
            cur += keep[l];

            if cur > max_val {
                max_val = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        ans = max_value(ans, max_val);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array `n`
    let n: usize = lines
        .next()
        .expect("Expected input for n")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the value of `k`
    let k: usize = lines
        .next()
        .expect("Expected input for k")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse k");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for nums")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), n, "The number of elements in the array must match n");

    // Calculate the result
    let result = max_subarray_sum(&nums, n, k);

    // Print the result
    println!("{}", result);
}