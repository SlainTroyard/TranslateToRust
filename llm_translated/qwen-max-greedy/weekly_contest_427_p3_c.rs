use std::io::{self, BufRead, Write};

/// Returns the maximum of two values.
fn max_value(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

/// Computes the maximum sum of a subarray with the given constraints.
fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur = 0;
    let mut keep = vec![0; n - k + 1];

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;
    let mut max;

    for i in 0..k.min(n - k + 1) {
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
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Input size of the array and the value of k
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Invalid input for n");
    buffer.clear();

    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let k: usize = buffer.trim().parse().expect("Invalid input for k");
    buffer.clear();

    // Input the array elements
    let mut nums = vec![0; n];
    for i in 0..n {
        stdin.lock().read_line(&mut buffer).expect("Failed to read line");
        nums[i] = buffer.trim().parse().expect("Invalid input for nums");
        buffer.clear();
    }

    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    writeln!(stdout, "{}", result).expect("Failed to write result");
}