// Translation of the C++ solution for LeetCode Weekly Contest 428 Problem 4
// to idiomatic Rust, preserving the exact algorithm logic and I/O format.

use std::io;

// A large constant to represent "infinity" for this problem
const INF: i32 = 1_000_000;

// A struct to encapsulate the solution methods
struct Solution;

impl Solution {
    /// Computes the minimum number of operations to make the string "good".
    /// Operations involve adjusting character frequencies to match a certain target k,
    /// combined across adjacent frequency elements.
    fn make_string_good(&self, s: &str) -> i32 {
        let n = s.len() as i32;

        // Count character frequencies in a 26-element array
        let mut a = vec![0; 26];
        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        // Start 'ans' as the minimum number of deletions to keep only one character
        let mut ans = n;
        for &x in &a {
            ans = ans.min(n - x);
        }

        // Try all possible target frequencies k from 1 to n
        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = ans.min(self.dfs(&mut dp, &a, k, 0));
        }

        ans
    }

    /// Recursive function with memoization to compute cost of adjustments
    /// for frequency array 'a', aiming for target frequency 'k' at position 'i'.
    fn dfs(&self, dp: &mut [i32], a: &[i32], k: i32, i: usize) -> i32 {
        // If we've processed all 26 possible frequencies, no more cost
        if i >= a.len() {
            return 0;
        }

        // Return memoized answer if already computed
        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;

        // If current frequency a[i] is >= k
        if a[i] >= k {
            let more = a[i] - k; // Excess amount beyond k
            // Option 1: adjust a[i] to k, add cost (a[i] - k), and move on
            ans = ans.min(more + self.dfs(dp, a, k, i + 1));

            // Option 2: possibly adjust with help from the next frequency
            if i + 1 < a.len() && a[i + 1] <= k {
                // If next freq + leftover from this freq can reach k
                if a[i + 1] + more >= k {
                    ans = ans.min(more + self.dfs(dp, a, k, i + 2));
                } else {
                    // Either bring the next freq up to k or adjust partially
                    ans = ans.min(
                        (more + (k - (a[i + 1] + more))).min(more + a[i + 1])
                            + self.dfs(dp, a, k, i + 2),
                    );
                }
            }
        } else {
            // a[i] < k
            // Option 1: either bring a[i] up to k or remove all from a[i]
            ans = ans.min((k - a[i]).min(a[i]) + self.dfs(dp, a, k, i + 1));

            // Option 2: possibly combine with the next freq
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = ans.min(a[i] + self.dfs(dp, a, k, i + 2));
                } else {
                    ans = ans.min(
                        a[i] + (k - (a[i] + a[i + 1])) + self.dfs(dp, a, k, i + 2),
                    );
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() {
    // Read the input string (single token) from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim();

    // Create a solution instance, compute the result, and print it
    let solution = Solution;
    let result = solution.make_string_good(s);
    println!("{}", result);
}