// Problem: Weekly Contest 414 Problem 3
use std::io;

struct Solution;

impl Solution {
    fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] > mx {
                mx = nums[i];
            }
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let nums_size: usize = parts[0].parse().expect("Invalid input for nums size");
    let nums: Vec<i32> = parts[1..]
        .iter()
        .take(nums_size)
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();
    
    let sol = Solution;
    println!("{}", sol.find_maximum_score(&nums));
}