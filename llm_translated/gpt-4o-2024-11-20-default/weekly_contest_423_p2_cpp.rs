use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
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
    // Read input size and array elements
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: size of the array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Second line: array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure we have the correct number of elements, matching the size of the array
    assert_eq!(nums.len(), n);

    // Create a Solution object and call the function to get the result
    let result = Solution::max_increasing_subarrays(nums);

    // Output the result
    println!("{}", result);
}