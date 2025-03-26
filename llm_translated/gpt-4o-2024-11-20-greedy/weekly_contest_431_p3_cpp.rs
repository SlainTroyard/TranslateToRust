use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_coins(nums: Vec<Vec<i64>>, k: i64) -> i64 {
        let mut nums = nums;
        // Sort the intervals based on their starting points
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = 0;
        let mut presum = vec![0i64; nums.len() + 1];

        // Compute prefix sums
        for i in 1..=nums.len() {
            presum[i] = presum[i - 1] + (nums[i - 1][1] - nums[i - 1][0] + 1) * nums[i - 1][2];
        }

        let mut left = 0;
        let mut right = 0;

        // First pass: sliding window from left to right
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][0] - nums[left][0]);
                ans = max(
                    ans,
                    tamp * nums[right][2] + presum[right] - presum[left],
                );
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        left = nums.len() - 1;
        right = nums.len() - 1;

        // Second pass: sliding window from right to left
        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][1] - nums[left][1]);
                ans = max(
                    ans,
                    tamp * nums[left][2] + presum[right + 1] - presum[left + 1],
                );
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            if left == 0 {
                break;
            }
            left -= 1;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: n and K
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i64 = first_line_iter.next().unwrap().parse().unwrap();

    // Read the next n lines: the intervals
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let start: i64 = iter.next().unwrap().parse().unwrap();
        let end: i64 = iter.next().unwrap().parse().unwrap();
        let coins: i64 = iter.next().unwrap().parse().unwrap();
        nums.push(vec![start, end, coins]);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.maximum_coins(nums, k);

    // Print the result
    println!("{}", result);
}