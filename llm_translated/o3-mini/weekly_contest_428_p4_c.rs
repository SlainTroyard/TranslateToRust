use std::cmp::min;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// A constant representing infinity (a suitably large number)
const INF: i32 = 1_000_000;

/// Recursive DFS function that computes the minimum cost to fix the string.
/// 
/// # Arguments
/// • `dp` - A mutable slice for memoization (DP).
/// • `a` - A reference to the frequency array of letters (size 26).
/// • `k` - The current target frequency.
/// • `i` - The current index in the frequency array.
///
/// # Returns
/// The minimum cost starting from index `i` given the target frequency `k`.
fn dfs(dp: &mut [i32], a: &[i32; 26], k: i32, i: usize) -> i32 {
    // If we have processed all 26 letters, no cost is needed.
    if i >= dp.len() {
        return 0;
    }
    
    // Use memoized result if available.
    if dp[i] != -1 {
        return dp[i];
    }
    
    let mut ans = INF;
    
    // If current frequency is at least k.
    if a[i] >= k {
        let more = a[i] - k;
        // Option 1: Adjust current letter only.
        ans = min(ans, a[i] - k + dfs(dp, a, k, i + 1));
        // Option 2: If next letter frequency is not over target.
        if i + 1 < dp.len() && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = min(ans, more + dfs(dp, a, k, i + 2));
            } else {
                ans = min(ans, (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2));
            }
        }
    } else {
        // If current frequency is less than k.
        ans = min(ans, (k - a[i]) + dfs(dp, a, k, i + 1));
        if i + 1 < dp.len() && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = min(ans, a[i] + dfs(dp, a, k, i + 2));
            } else {
                ans = min(ans, a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2));
            }
        }
    }
    
    dp[i] = ans;
    ans
}

/// The main function that reads input, calls `make_string_good`, and prints the result.
///
/// It closely adheres to the input/output format of the original C implementation.
fn main() -> Result<(), Box<dyn Error>> {
    // Create a buffered reader for standard input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the input string. This code assumes the string fits on one line,
    // as per the original code's behavior using scanf.
    reader.read_line(&mut input)?;
    // Trim any whitespace or newline characters.
    let s = input.trim();
    
    // Calculate the length of the string.
    let n = s.len() as i32;
    
    // Frequency array for characters 'a' to 'z'.
    let mut a = [0i32; 26];
    for ch in s.chars() {
        if let Some(idx) = (ch as u8).checked_sub(b'a') {
            if (idx as usize) < 26 {
                a[idx as usize] += 1;
            }
        }
    }
    
    // Initially, answer is set to the length of the string.
    let mut ans = n;
    
    // Try making the string good by removing all letters except one.
    for i in 0..26 {
        ans = min(ans, n - a[i]);
    }
    
    // Try all possible target frequency values k from 1 to n.
    for k in 1..=n {
        // dp array for memoization, with size 26 initialized to -1.
        let mut dp = vec![-1; 26];
        let result = dfs(&mut dp, &a, k, 0);
        ans = min(ans, result);
    }
    
    // Output the result to stdout, matching the original format.
    println!("{}", ans);
    
    // Ensure all output is flushed.
    io::stdout().flush()?;
    Ok(())
}