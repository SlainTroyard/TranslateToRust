use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_length(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let m = *nums.iter().max().unwrap_or(&0) as usize;

        // Create factorization array
        let mut fac: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
        for i in 2..=m {
            if fac[i].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j].push(i);
                    j += i;
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; m + 1];
        
        let mut j = 0;
        for i in 0..n {
            while j < n {
                // Check if we can include nums[j]
                let can_include = fac[nums[j] as usize].iter().all(|&p| !vis[p]);
                
                if can_include {
                    // Mark all prime factors of nums[j] as seen
                    for &p in &fac[nums[j] as usize] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            
            ans = cmp::max(ans, (j - i) as i32);
            
            // Remove the prime factors of nums[i] from our seen set
            for &p in &fac[nums[i] as usize] {
                vis[p] = false;
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of nums array
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and output result
    let result = Solution::max_length(&nums);
    println!("{}", result);
}