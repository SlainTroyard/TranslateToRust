use std::io::{self, BufRead};

/// A struct representing our solution, mirroring the C++ Solution class.
struct Solution;

impl Solution {
    /// Translates the C++ `countValidSelections` method into Rust.
    /// 
    /// Given a vector of integers, computes two prefix-sum arrays (from the left and from the right),
    /// and then iterates through each element to determine the total valid selections.
    /// 
    /// This preserves exactly the same logic as the original C++ implementation.
    fn count_valid_selections(&self, nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        // Compute prefix sums from the left (left[i] holds the sum of nums[0..i])
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute prefix sums from the right (right[i] holds the sum of nums[i+1..n])
        for i in 1..n {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        // Iterate through each element to determine valid selections
        for i in 0..n {
            // Skip if current element is non-zero
            if nums[i] != 0 {
                continue;
            }
            // If left and right prefix sums match, add 2 to the result
            if left[i] == right[i] {
                res += 2;
            }
            // If their absolute difference is 1, add 1 to the result
            if (left[i] - right[i]).abs() == 1 {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    // Read all tokens from stdin
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            for num_str in l.split_whitespace() {
                // We use unwrap to mirror the C++ behavior and not handle parse errors.
                tokens.push(num_str.parse::<i32>().unwrap());
            }
        }
    }

    // The first token is the number of elements
    let n = tokens[0] as usize;

    // The next n tokens are the elements of the nums array
    let nums = tokens[1..1 + n].to_vec();

    // Create a Solution instance and compute the result
    let solution = Solution;
    let result = solution.count_valid_selections(&nums);

    // Print the result to stdout (equivalent to std::cout << result << std::endl)
    println!("{}", result);
}