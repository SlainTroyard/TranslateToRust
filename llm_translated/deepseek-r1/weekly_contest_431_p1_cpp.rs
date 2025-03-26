use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<usize>) -> i32 {
        let m = *nums.iter().max().unwrap_or(&0);
        let mut factors = vec![vec![]; m + 1];
        
        // Sieve to compute prime factors for each number up to m
        for i in 2..=m {
            if factors[i].is_empty() {
                for j in (i..=m).step_by(i) {
                    factors[j].push(i);
                }
            }
        }
        
        let n = nums.len();
        let mut ans = 2;
        let mut vis = vec![false; m + 1];
        let mut j = 0;
        
        // Sliding window approach with two pointers i and j
        for i in 0..n {
            // Expand window to the right as much as possible
            while j < n {
                let num = nums[j];
                // Check if current number's factors are all not visited
                let can_include = factors[num].iter().all(|&p| !vis[p]);
                if can_include {
                    // Mark factors as visited
                    for &p in &factors[num] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            // Update the maximum window length
            ans = ans.max(j - i);
            // Remove factors of nums[i] from visited before moving left pointer
            for &p in &factors[nums[i]] {
                vis[p] = false;
            }
        }
        
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    let num_size: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<usize> = tokens.take(num_size).map(|s| s.parse().unwrap()).collect();
    
    let result = Solution::max_length(nums);
    println!("{}", result);
}