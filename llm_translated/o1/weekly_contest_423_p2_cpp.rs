use std::io::{self, BufRead};

// A struct replicating the original Solution class
struct Solution;

impl Solution {
    /// Returns the maximum length of increasing subarrays 
    /// based on the specific criteria described in the original C++ code.
    fn max_increasing_subarrays(&self, nums: &[i32]) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        // Iterate through the array from the second element to the end
        for i in 1..nums.len() {
            // Check if current element is greater than the previous one
            if nums[i - 1] < nums[i] {
                curr += 1;  // Extend current increasing subarray
            } else {
                // Current subarray ended, so store its length into 'prev' and reset 'curr'
                prev = curr;
                curr = 1;
            }
            // Update the answer using the specific logic from the original code
            ans = ans.max((curr / 2).max(prev.min(curr)));
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array (n) from stdin
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n: usize = line.trim().parse()?;

    // Read exactly n integers from stdin
    let mut nums = Vec::with_capacity(n);
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while nums.len() < n {
        if let Some(Ok(l)) = lines.next() {
            for num_str in l.split_whitespace() {
                let val: i32 = num_str.parse()?;
                nums.push(val);
                if nums.len() == n {
                    break;
                }
            }
        } else {
            // If there are no more lines or IO errors occur,
            // we simply break (mimicking the original C++ behavior).
            break;
        }
    }

    // Create a Solution object and call the function
    let sol = Solution;
    let result = sol.max_increasing_subarrays(&nums);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}