use std::io::{self, BufRead};
use std::cmp;

// Solution struct to match the original CPP class
struct Solution;

impl Solution {
    fn max_increasing_subarrays(nums: &Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        // Traverse through the nums array
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1;  // Increase the length of the current increasing subarray
            } else {
                prev = curr;  // Update the previous subarray length
                curr = 1;     // Reset current subarray length
            }
            // Update the answer by considering both the current and previous subarray lengths
            ans = cmp::max(ans, cmp::max(curr / 2, cmp::min(prev, curr)));
        }
        return ans;  // Return the maximum length of increasing subarrays
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Input the size of the array
    let n: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Input the array elements
    let nums: Vec<i32> = iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Create a Solution object and call the function to get the result
    let result = Solution::max_increasing_subarrays(&nums);
    
    // Output the result
    println!("{}", result);
}