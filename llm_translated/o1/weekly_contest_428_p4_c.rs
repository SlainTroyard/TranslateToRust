// Translated from the original C code for LeetCode Weekly Contest 428 Problem 4
// REQUIREMENTS:
// 1. Translates the entire file as a complete program, including main function and I/O
// 2. Preserves the exact same algorithm logic
// 3. Uses idiomatic Rust, but keeps the same input/output format
// 4. Maintains the exact same stdin/stdout format
// 5. No assumptions about the input format beyond what the C code does

use std::io::{self, BufRead};

const INF: i32 = 1000000;

// The DFS function corresponds directly to the C version, preserving logic exactly.
fn dfs(dp: &mut [i32], a: &[i32], k: i32, i: usize, size: usize) -> i32 {
    // If we've reached or exceeded the end of the array, return 0
    if i >= size {
        return 0;
    }

    // If we have a cached result, return it
    if dp[i] != -1 {
        return dp[i];
    }

    let mut ans = INF;

    // If a[i] >= k
    if a[i] >= k {
        let more = a[i] - k;
        // ans = min(ans, a[i] - k + dfs(...))
        ans = ans.min(more + dfs(dp, a, k, i + 1, size));

        // If there's a next element
        if i + 1 < size && a[i + 1] <= k {
            // If a[i+1] + more >= k
            if a[i + 1] + more >= k {
                ans = ans.min(more + dfs(dp, a, k, i + 2, size));
            } else {
                // Equivalent to (more + k - (a[i+1] + more)) + dfs(...)
                let cost = more + (k - (a[i + 1] + more));
                ans = ans.min(cost + dfs(dp, a, k, i + 2, size));
            }
        }
    } else {
        // a[i] < k
        let add_cost = k - a[i];
        ans = ans.min(add_cost + dfs(dp, a, k, i + 1, size));

        if i + 1 < size && a[i + 1] <= k {
            // If a[i] + a[i+1] >= k
            if a[i] + a[i + 1] >= k {
                ans = ans.min(a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                let cost = a[i] + (k - (a[i] + a[i + 1]));
                ans = ans.min(cost + dfs(dp, a, k, i + 2, size));
            }
        }
    }

    dp[i] = ans;
    ans
}

// Equivalent logic to the original C function makeStringGood.
fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = [0i32; 26];

    // Count the frequency of each character
    for ch in s.bytes() {
        a[(ch - b'a') as usize] += 1;
    }

    // Initialize answer to the length of the string
    let mut ans = n as i32;

    // One simple approach: ans can be at least min(n - freq[i]) for all i
    for i in 0..26 {
        ans = ans.min(n as i32 - a[i]);
    }

    // For each k from 1 to n, run dfs
    for k in 1..=n as i32 {
        let mut dp = [-1; 26];
        let cost = dfs(&mut dp, &a, k, 0, 26);
        ans = ans.min(cost);
    }

    ans
}

fn main() {
    // The original C code uses scanf("%s", s), which reads a single token from stdin.
    // We mimic that behavior here. We'll read one line and trim it.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Attempt to read a single non-empty token
    let mut input_str = String::new();
    while let Some(Ok(line)) = lines.next() {
        let line = line.trim();
        if !line.is_empty() {
            input_str = line.to_string();
            break;
        }
    }

    // Call the function and print the result
    let result = make_string_good(&input_str);
    println!("{}", result);
}