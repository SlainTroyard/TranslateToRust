use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    // Read all input from stdin; the input format consists of:
    // - first, an integer n
    // - then n integers (they can be separated by whitespace over multiple lines)
    let stdin = io::stdin();
    let locked_stdin = stdin.lock();
    let mut input = String::new();
    for line in locked_stdin.lines() {
        input.push_str(&line?);
        input.push('\n');
    }
    // Split the input by whitespace into tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut token_iter = tokens.into_iter();

    // Parse the first token as the number of elements
    let n: usize = token_iter
        .next()
        .ok_or("Expected number of elements 'n'")?
        .parse()?;
    
    // Parse the next n tokens as the numbers in the array.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = token_iter
            .next()
            .ok_or("Expected a number in the input array")?
            .parse()?;
        nums.push(num);
    }

    // Compute the answer using our helper function
    let ans = subarray_sum(&nums);
    
    // Output the result exactly as in the original C++ code.
    println!("{}", ans);

    Ok(())
}

// Function that calculates the subarray sum as described in the problem.
// It mimics the C++ algorithm:
//
// 1. Construct a prefix sum vector `s` of size n+1, with s[0] = 0, s[i+1] = s[i] + nums[i].
// 2. Then for each index i, add (s[i+1] - s[max(i - nums[i], 0)]) to the answer.
fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    // Create a prefix sum array of size n+1, initialized to 0.
    let mut s = vec![0; n + 1];
    // Calculate prefix sums: s[i+1] = s[i] + nums[i]
    for i in 0..n {
        s[i + 1] = s[i] + nums[i];
    }

    let mut ans = 0;
    // Iterate over each index to compute the required sum using the prefix sums.
    for i in 0..n {
        // Calculate the lower bound index: max(i - nums[i], 0)
        let lower_index = if i as i32 - nums[i] >= 0 {
            (i as i32 - nums[i]) as usize
        } else {
            0
        };
        ans += s[i + 1] - s[lower_index];
    }
    ans
}