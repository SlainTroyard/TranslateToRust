use std::io::{self, Read};
use std::cmp;

fn max_value(a: i64, b: i64) -> i64 {
    cmp::max(a, b)
}

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

    for i in 0..cmp::min(k, n - k + 1) {
        let mut cur = 0;
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read n and k
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let k: usize = lines.next().unwrap().trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .take(n)
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);

    Ok(())
}