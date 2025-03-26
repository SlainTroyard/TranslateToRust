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
                // Check if current number can be added to the sequence
                let can_add = fac[nums[j] as usize].iter().all(|&p| !vis[p]);
                
                if can_add {
                    // Mark all prime factors as visited
                    for &p in &fac[nums[j] as usize] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            
            ans = cmp::max(ans, (j - i) as i32);
            
            // Remove the current number from consideration
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
    
    // Read the size of the array
    let num_size: usize = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), num_size);
    
    // Solve the problem
    let result = Solution::max_length(&nums);
    
    // Output the result
    println!("{}", result);
}