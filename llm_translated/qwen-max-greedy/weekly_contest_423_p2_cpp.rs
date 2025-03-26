use std::io::{self, BufRead};

/// Represents the solution to the problem.
struct Solution;

impl Solution {
    /// Finds the maximum number of increasing subarrays.
    fn max_increasing_subarrays(nums: &Vec<i32>) -> i32 {
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
            ans = ans.max(curr / 2).max(prev.min(curr));
        }
        ans  // Return the maximum length of increasing subarrays
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    // Input the size of the array
    let mut n = String::new();
    stdin_lock.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Input the array elements
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_str = String::new();
        stdin_lock.read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Create a Solution object and call the function to get the result
    let sol = Solution;
    let result = Solution::max_increasing_subarrays(&nums);

    // Output the result
    println!("{}", result);
}