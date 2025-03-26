use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: &Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        
        for i in 0..n {
            let mut curr_sum = 0;
            for j in i..n {
                curr_sum += nums[j];
                let length = (j - i + 1) as i32;
                if length >= l && length <= r && curr_sum > 0 {
                    mini = mini.min(curr_sum);
                }
            }
        }
        
        if mini == i32::MAX { -1 } else { mini }
    }
}

fn main() {
    let solution = Solution;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Expected a number");
    
    // Input the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected numbers"))
        .collect();
    
    // Input the range [l, r]
    let lr_line = lines.next().unwrap().unwrap();
    let mut lr_iter = lr_line.split_whitespace();
    let l: i32 = lr_iter.next().unwrap().parse().expect("Expected a number for l");
    let r: i32 = lr_iter.next().unwrap().parse().expect("Expected a number for r");
    
    // Compute the minimum sum subarray
    let result = Solution::minimum_sum_subarray(&nums, l, r);
    
    // Output the result
    println!("{}", result);
}