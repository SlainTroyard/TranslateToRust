use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn maximum_coins(nums: &mut Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort the nums vector by the first element of each sub-vector
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; nums.len() + 1];
        
        // Calculate prefix sum
        for i in 1..=nums.len() {
            presum[i] = presum[i - 1] + ((nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64);
        }
        
        // First pass: left to right
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k as i64 - (nums[right][0] - nums[left][0]) as i64;
                ans = ans.max(tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            
            if left >= nums.len() {
                break;
            }
            
            ans = ans.max(presum[right + 1] - presum[left]);
            right += 1;
        }
        
        // Second pass: right to left
        left = nums.len() - 1;
        right = nums.len() - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k as i64 - (nums[right][1] - nums[left][1]) as i64;
                ans = ans.max(tamp * nums[left][2] as i64 + presum[right + 1] - presum[left + 1]);
                right -= 1;
            }
            
            if right < 0 {
                break;
            }
            
            ans = ans.max(presum[right + 1] - presum[left]);
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
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0] as usize;
    let k = parts[1];
    
    // Read the n vectors
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        vec.push(values);
    }
    
    let result = Solution::maximum_coins(&mut vec, k);
    println!("{}", result);
}