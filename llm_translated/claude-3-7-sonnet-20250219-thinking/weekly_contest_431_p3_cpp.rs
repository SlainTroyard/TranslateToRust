use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn maximum_coins(nums: &mut Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort nums based on the first element
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans: i64 = 0;
        let mut presum = vec![0i64; nums.len() + 1];
        
        // Compute prefix sums
        for i in 1..nums.len() + 1 {
            presum[i] = presum[i - 1] + ((nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64);
        }
        
        // First sliding window from left to right
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k as i64 - (nums[right][0] - nums[left][0]) as i64;
                ans = cmp::max(ans, tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = cmp::max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }
        
        // Second sliding window from right to left
        let mut left = nums.len() as i32 - 1;
        let mut right = nums.len() as i32 - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right as usize][1] - nums[left as usize][0] + 1 > k {
                let tamp = k as i64 - (nums[right as usize][1] - nums[left as usize][1]) as i64;
                ans = cmp::max(ans, tamp * nums[left as usize][2] as i64 + presum[right as usize + 1] - presum[left as usize + 1]);
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = cmp::max(ans, presum[right as usize + 1] - presum[left as usize]);
            left -= 1;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and K
    let line = lines.next().unwrap()?;
    let parts: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse input"))
        .collect();
    
    let n = parts[0];
    let k = parts[1];
    
    // Read the vector of vectors
    let mut vec = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse input"))
            .collect();
        
        vec.push(row);
    }
    
    // Solve and print the result
    let result = Solution::maximum_coins(&mut vec, k);
    println!("{}", result);
    
    Ok(())
}