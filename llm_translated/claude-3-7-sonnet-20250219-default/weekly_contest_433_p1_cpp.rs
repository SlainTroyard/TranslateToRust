use std::io::{self, BufRead};
use std::cmp::max;

// Solution struct that contains the function to calculate the subarray sum
struct Solution;

impl Solution {
    // Calculates the sum of values where each element adds the sum of elements
    // from max(i - nums[i], 0) to i
    fn subarray_sum(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // Create a prefix sum array where s[i] = sum of nums[0] to nums[i-1]
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            // For each position, calculate sum of the subarray from max(i - nums[i], 0) to i
            ans += s[i + 1] - s[max(i as i32 - nums[i], 0) as usize];
        }
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Create a solution instance and calculate the result
    let solution = Solution;
    println!("{}", Solution::subarray_sum(&nums));
}