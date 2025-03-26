use std::cmp::{max, min};
use std::io::{self, BufRead};

fn max_increasing_subarrays(nums: &[i32]) -> i32 {
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
        ans = max(ans, max(curr / 2, min(prev, curr)));
    }
    ans  // Return the maximum length of increasing subarrays
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Input the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the function to get the result
    let result = max_increasing_subarrays(&nums);

    // Output the result
    println!("{}", result);
}