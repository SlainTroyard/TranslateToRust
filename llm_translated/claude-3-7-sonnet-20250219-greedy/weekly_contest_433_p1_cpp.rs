use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    fn subarray_sum(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // Create a prefix sum array (s[0] = 0, s[1] = nums[0], s[2] = nums[0] + nums[1], etc.)
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            // Calculate the sum of the subarray from max(i - nums[i], 0) to i
            ans += s[i + 1] - s[max(i as i32 - nums[i], 0) as usize];
        }
        return ans;
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
    
    // Create a solution instance and call the function
    let solution = Solution;
    println!("{}", Solution::subarray_sum(&nums));
}