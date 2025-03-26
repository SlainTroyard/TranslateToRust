use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_length(nums: &[i32]) -> i32 {
        let n = nums.len();
        let m = *nums.iter().max().unwrap_or(&0) as usize;

        // Calculate prime factors for each number from 2 to m
        let mut fac: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
        for i in 2..=m {
            if fac[i].is_empty() {
                for j in (i..=m).step_by(i) {
                    fac[j].push(i);
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; m + 1];
        
        let mut j = 0;
        for i in 0..n {
            while j < n {
                // Check if nums[j] shares any prime factors with the current window
                let can_add = fac[nums[j] as usize].iter().all(|&p| !vis[p]);
                if can_add {
                    // Mark all prime factors of nums[j] as visited
                    for &p in &fac[nums[j] as usize] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            ans = ans.max(j - i);
            
            // Remove the prime factors of nums[i] from the visited set
            for &p in &fac[nums[i] as usize] {
                vis[p] = false;
            }
        }
        
        ans as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read numSize
    let num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse numSize");
    
    // Read nums
    let mut nums = Vec::with_capacity(num_size);
    let nums_line = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input");
    
    for num_str in nums_line.split_whitespace().take(num_size) {
        let num: i32 = num_str.parse().expect("Failed to parse number");
        nums.push(num);
    }
    
    // Calculate and print result
    let solution = Solution;
    let result = Solution::max_length(&nums);
    println!("{}", result);
}