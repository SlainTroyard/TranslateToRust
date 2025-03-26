// Problem: Weekly Contest 424 Problem 1
use std::io;

struct Solution;

impl Solution {
    fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        
        // Compute prefix sums from the left
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }
        
        // Compute prefix sums from the right
        for i in 1..n {
            let index = n - i - 1;
            right[index] = right[index + 1] + nums[index + 1];
        }
        
        // Iterate through each element to count valid selections
        let mut res = 0;
        for i in 0..n {
            if nums[i] != 0 {
                continue;
            }
            if left[i] == right[i] {
                res += 2;
            }
            if (left[i] - right[i]).abs() == 1 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = tokens[0] as usize;
    let nums = tokens[1..n + 1].to_vec();
    
    let solution = Solution;
    let result = solution.count_valid_selections(nums);
    println!("{}", result);
}