use std::io;
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: &Vec<i32>) -> i32 {
        let mut flag = 0;
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
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // Read the array elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    
    // Create a Solution object and call the function to get the result
    let result = Solution::max_increasing_subarrays(&nums);
    
    // Output the result
    println!("{}", result);
}