use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        // Traverse through the nums array
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1; // Increase the length of the current increasing subarray
            } else {
                prev = curr; // Update the previous subarray length
                curr = 1;    // Reset current subarray length
            }
            // Update the answer by considering both the current and previous subarray lengths
            ans = ans.max((curr / 2).max(prev.min(curr)));
        }
        ans // Return the maximum length of increasing subarrays
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Input the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), n, "Array size does not match the input size");

    // Create a Solution object and call the function to get the result
    let result = Solution::max_increasing_subarrays(nums);

    // Output the result
    println!("{}", result);
}