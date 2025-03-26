use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_coins(mut nums: Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort by the first element (start of range)
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; nums.len() + 1];
        
        // Calculate prefix sum of coin values
        for i in 1..=nums.len() {
            presum[i] = presum[i - 1] + ((nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64);
        }
        
        // First pass: left to right
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k as i64 - (nums[right][0] - nums[left][0]) as i64;
                ans = max(ans, tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }
        
        // Second pass: right to left
        let mut left = nums.len() as i32 - 1;
        let mut right = nums.len() as i32 - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right as usize][1] - nums[left as usize][0] + 1 > k {
                let tamp = k as i64 - (nums[right as usize][1] - nums[left as usize][1]) as i64;
                ans = max(ans, tamp * nums[left as usize][2] as i64 + presum[right as usize + 1] - presum[left as usize + 1]);
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = max(ans, presum[right as usize + 1] - presum[left as usize]);
            left -= 1;
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and K
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the vectors
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(values);
    }
    
    let sol = Solution;
    println!("{}", sol.maximum_coins(vec, k));
}