// Problem: Weekly Contest 428 Problem 3
// Translated from C++ to Rust with the same logic and I/O format.

use std::io::{self, Read};

/// Calculates the number of "beautiful" splits in the given sequence.
/// 
/// lcp[i][j] represents the Longest Common Prefix (LCP) between the
/// subsequence starting at i and the subsequence starting at j.
fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    // 2D array to store LCP values, dimensions (n+1) x (n+1).
    let mut lcp = vec![vec![0; n + 1]; n + 1];

    // Fill the LCP table from the end to the start, just like the C++ code.
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut ans = 0;
    // Check all possible splits (i, j) with 1 <= i < j < n.
    for i in 1..(n - 1) {
        for j in (i + 1)..n {
            // Replicates the condition:
            // (i <= (j - i) AND lcp[0][i] >= i) OR (lcp[i][j] >= (j - i))
            if (i <= j - i && (lcp[0][i] as usize) >= i)
                || (lcp[i][j] as usize) >= (j - i)
            {
                ans += 1;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // Read all input from stdin as a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace into tokens
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // First token is n, the size of the array
    let n: usize = tokens[0].parse().expect("Failed to parse n");

    // Next n tokens are the array elements nums
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let val: i32 = tokens[i + 1].parse().expect("Failed to parse array element");
        nums.push(val);
    }

    // Calculate and print the result just like the C++ code
    let result = beautiful_splits(&nums);
    println!("{}", result);

    Ok(())
}