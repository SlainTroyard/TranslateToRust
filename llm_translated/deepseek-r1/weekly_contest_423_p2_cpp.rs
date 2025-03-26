use std::io::{self, Read};
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: &[i32]) -> i32 {
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
            // Calculate the maximum between curr/2 and min(prev, curr)
            let option1 = curr / 2;
            let option2 = cmp::min(prev, curr);
            ans = cmp::max(ans, cmp::max(option1, option2));
        }

        ans
    }
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Parse the first token as the size of the array
    let n: usize = tokens.next().unwrap().parse().unwrap();

    // Parse the next 'n' tokens as the array elements
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the result
    let result = Solution::max_increasing_subarrays(&nums);
    println!("{}", result);
}