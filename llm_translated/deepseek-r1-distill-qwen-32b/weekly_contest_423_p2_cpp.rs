// Problem: Weekly Contest 423 Problem 2
use std::io;

struct Solution;

impl Solution {
    fn max_increasing_subarrays(&self, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;
        
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1;
            } else {
                prev = curr;
                curr = 1;
            }
            let temp = curr / 2;
            let temp2 = prev.min(curr);
            ans = ans.max(temp.max(temp2));
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    let n: usize = tokens.next()
        .expect("No value for n")
        .parse()
        .expect("Invalid n");
        
    let nums: Vec<i32> = tokens.take(n)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
        
    let sol = Solution;
    let result = sol.max_increasing_subarrays(nums);
    println!("{}", result);
}