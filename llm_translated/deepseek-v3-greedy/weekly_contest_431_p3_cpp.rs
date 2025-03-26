use std::cmp::max;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line for n and K
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the next n lines for the vectors
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let c: i32 = iter.next().unwrap().parse().unwrap();
        nums.push(vec![a, b, c]);
    }
    
    let sol = Solution;
    println!("{}", sol.maximum_coins(nums, k));
}

struct Solution;

impl Solution {
    pub fn maximum_coins(&self, mut nums: Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort the nums vector based on the first element of each sub-vector
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans: i64 = 0;
        let mut presum = vec![0; nums.len() + 1];
        
        // Calculate the prefix sum
        for i in 1..=nums.len() {
            let a = nums[i - 1][0] as i64;
            let b = nums[i - 1][1] as i64;
            let c = nums[i - 1][2] as i64;
            presum[i] = presum[i - 1] + ((b - a + 1) * c);
        }
        
        // First pass: left to right
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && (nums[right][1] - nums[left][0] + 1) > k {
                let tamp = k - (nums[right][0] - nums[left][0]);
                ans = max(ans, (tamp as i64) * (nums[right][2] as i64) + presum[right] - presum[left]);
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }
        
        // Second pass: right to left
        left = nums.len() - 1;
        right = nums.len() - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && (nums[right][1] - nums[left][0] + 1) > k {
                let tamp = k - (nums[right][1] - nums[left][1]);
                ans = max(ans, (tamp as i64) * (nums[left][2] as i64) + presum[right + 1] - presum[left + 1]);
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