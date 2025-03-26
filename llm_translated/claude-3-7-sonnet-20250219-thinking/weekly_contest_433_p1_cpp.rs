use std::io::{self, Read};
use std::cmp::max;

struct Solution;

impl Solution {
    fn subarray_sum(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // Create a prefix sum array
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            // Calculate the subarray sum using the prefix sum array
            ans += s[i + 1] - s[max(0, i as i32 - nums[i]) as usize];
        }
        ans
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut iter = input.split_whitespace();
    
    // Parse the size of the array
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    
    // Parse the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(iter.next().unwrap().parse::<i32>().expect("Failed to parse number"));
    }
    
    // Solve and print the result
    println!("{}", Solution::subarray_sum(&nums));
}