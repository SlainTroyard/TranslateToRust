use std::cmp;
use std::io::{self, BufRead, Write};

fn maximum_coins(nums: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort by the first element of each vector
    nums.sort_by(|a, b| a[0].cmp(&b[0]));

    let n = nums.len();
    let mut ans = 0i64;
    // Create prefix sum vector with one extra element.
    // Each element: number of coins from a full group,
    // computed as (nums[i][1] - nums[i][0] + 1) * nums[i][2]
    let mut presum = vec![0i64; n + 1];
    for i in 1..=n {
        let start = nums[i - 1][0] as i64;
        let end = nums[i - 1][1] as i64;
        let coin = nums[i - 1][2] as i64;
        presum[i] = presum[i - 1] + ((end - start + 1) * coin);
    }

    // First loop: sliding window with left and right pointers
    let mut left: usize = 0;
    let mut right: usize = 0;
    while right < n && left < n {
        // While the current window is too wide (not satisfying the coins constraint)
        while left < n && (nums[right][1] - nums[left][0] + 1) > k {
            // Compute the available coins in the partial group
            let available = k - (nums[right][0] - nums[left][0]);
            // available can be negative but this branch is only reached if condition > k, so available should be less than k?
            // However, we only compute the candidate if available is valid.
            let candidate = (available as i64) * (nums[right][2] as i64)
                + (presum[right] - presum[left]);
            ans = cmp::max(ans, candidate);
            left += 1;
        }
        // If left index reached beyond the bounds, break out
        if left >= n {
            break;
        }
        // Evaluate the entire coins sum from left to right group fully added (using prefix sums)
        ans = cmp::max(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second loop: process from the end backwards: left and right pointers
    if n > 0 {
        left = n - 1;
        right = n - 1;
        while right < n && left < n { // using unsigned type; condition is always true if not breaking inside.
            // While the current window is too wide
            while right < n && (nums[right][1] - nums[left][0] + 1) > k {
                let available = k - (nums[right][1] - nums[left][1]);
                let candidate = (available as i64) * (nums[left][2] as i64)
                    + (presum[right + 1] - presum[left + 1]);
                ans = cmp::max(ans, candidate);
                // Decrement right manually. Since right is usize, we must check for 0.
                if right == 0 {
                    // Cannot decrement below 0.
                    break;
                }
                right -= 1;
            }
            // Break early if right is 0 and we've already processed.
            if right >= n || (nums[right][1] - nums[left][0] + 1) > k {
                break;
            }
            // Add the full coin sum for the current window.
            ans = cmp::max(ans, presum[right + 1] - presum[left]);
            // Decrement left, careful of usize underflow.
            if left == 0 {
                break;
            }
            left -= 1;
        }
    }
    ans
}

fn main() -> io::Result<()> {
    // Create a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // Read the first line containing n and K.
    let first_line = reader
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing input"))??;
    // Split by whitespace and parse integers.
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing n"))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let k: i32 = first_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing k"))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Read subsequent n lines each containing 3 integers.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = reader
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Not enough lines"))??;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Not enough values in a line",
            ));
        }
        let a = parts[0]
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let b = parts[1]
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let c = parts[2]
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        nums.push(vec![a, b, c]);
    }

    // Compute the result by calling maximum_coins.
    let result = maximum_coins(&mut nums, k);

    // Write the result to stdout, preserving the original format.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}