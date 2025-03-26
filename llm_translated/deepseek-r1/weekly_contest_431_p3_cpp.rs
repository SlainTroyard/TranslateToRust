use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn maximum_coins(nums: &mut Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort intervals based on their start value
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        let n = nums.len();
        let mut ans = 0i64;

        // Compute prefix sums of (end - start + 1) * coins for each interval
        let mut presum = vec![0i64; n + 1];
        for i in 1..=n {
            let start = nums[i-1][0] as i64;
            let end = nums[i-1][1] as i64;
            let coins = nums[i-1][2] as i64;
            presum[i] = presum[i-1] + (end - start + 1) * coins;
        }

        // First pass: sliding window from left to right
        let mut left = 0;
        let mut right = 0;
        while right < n && left < n {
            // Adjust left pointer to maintain interval length <= k
            while left < n && (nums[right][1] - nums[left][0] + 1) > k {
                let tamp = k as i64 - (nums[right][0] - nums[left][0]) as i64;
                ans = ans.max(tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left >= n {
                break;
            }
            // Update answer with current window sum
            ans = ans.max(presum[right + 1] - presum[left]);
            right += 1;
        }

        // Second pass: sliding window from right to left using isize indices for underflow handling
        let mut left = n as isize - 1;
        let mut right = n as isize - 1;
        while right >= 0 && left >= 0 {
            // Adjust right pointer to maintain interval length <= k
            while right >= 0 && (nums[right as usize][1] - nums[left as usize][0] + 1) > k {
                let tamp = k as i64 - (nums[right as usize][1] - nums[left as usize][1]) as i64;
                ans = ans.max(tamp * nums[left as usize][2] as i64 + presum[right as usize + 1] - presum[left as usize + 1]);
                right -= 1;
            }
            if right < 0 {
                break;
            }
            // Update answer with current window sum
            ans = ans.max(presum[right as usize + 1] - presum[left as usize]);
            left -= 1;
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and K from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Read intervals and coins
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let c: i32 = parts.next().unwrap().parse().unwrap();
        nums.push(vec![a, b, c]);
    }

    // Compute and print the result
    println!("{}", Solution::maximum_coins(&mut nums, k));
}