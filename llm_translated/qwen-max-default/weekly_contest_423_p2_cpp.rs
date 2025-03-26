use std::io::{self, BufRead};

fn main() {
    // Read the size of the array from stdin
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number!")
    };

    // Read the array elements from stdin
    let nums: Vec<i32> = {
        let stdin = io::stdin();
        let mut nums = Vec::with_capacity(n);
        for _ in 0..n {
            let mut input = String::new();
            stdin.lock().read_line(&mut input).expect("Failed to read line");
            nums.push(input.trim().parse().expect("Please type a number!"));
        }
        nums
    };

    // Create a Solution object and call the function to get the result
    let sol = Solution;
    let result = sol.max_increasing_subarrays(&nums);

    // Output the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Finds the maximum length of increasing subarrays.
    pub fn max_increasing_subarrays(nums: &[i32]) -> i32 {
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