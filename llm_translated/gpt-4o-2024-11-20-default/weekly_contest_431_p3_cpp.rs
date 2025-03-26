use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_coins(nums: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut nums = nums;
        // Sort nums by their first element
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans: i64 = 0;
        let mut presum = vec![0i64; nums.len() + 1];

        // Calculate prefix sum
        for i in 1..=nums.len() {
            presum[i] = presum[i - 1]
                + ((nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64);
        }

        // Two pointers to iterate over segments from left to right
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][0] - nums[left][0]);
                ans = max(
                    ans,
                    tamp as i64 * nums[right][2] as i64 + presum[right] - presum[left],
                );
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        // Two pointers to iterate over segments from right to left
        left = nums.len() - 1;
        right = nums.len() - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][1] - nums[left][1]);
                ans = max(
                    ans,
                    tamp as i64 * nums[left][2] as i64 + presum[right + 1] - presum[left + 1],
                );
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            left -= 1;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and K
    let first_line = lines.next().unwrap().unwrap();
    let mut nk_iter = first_line.split_whitespace();
    let n: usize = nk_iter.next().unwrap().parse().unwrap();
    let k: i32 = nk_iter.next().unwrap().parse().unwrap();

    // Parse nums
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        nums.push(values);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.maximum_coins(nums, k);

    // Print the result
    println!("{}", result);
}