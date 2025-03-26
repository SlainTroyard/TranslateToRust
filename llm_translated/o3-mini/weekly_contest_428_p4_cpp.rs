use std::cmp;
use std::io::{self, BufRead};

// A large constant, used as an "infinite" cost.
// It is equivalent to 1e6 in the C++ code.
const INF: i32 = 1_000_000;

// Struct representing our solution,
// similar to the C++ "Solution" class.
struct Solution;

impl Solution {
    // The main method that computes the minimal cost to adjust
    // the string to be "good", following the problem statement.
    pub fn make_string_good(s: String) -> i32 {
        // The length of the string.
        let n = s.len() as i32;
        
        // Frequency array for letters 'a' to 'z'.
        let mut a = vec![0; 26];
        for c in s.chars() {
            // Since the input is assumed to contain only lowercase letters,
            // subtracting 'a' gives the index for our array.
            if c.is_ascii_lowercase() {
                a[c as usize - 'a' as usize] += 1;
            }
        }
        
        // Initialize answer with worst-case scenario: remove all characters.
        let mut ans = n;
        // Baseline: for each letter, consider converting the string to that letter.
        // The cost is n minus the frequency of that letter.
        for &x in &a {
            ans = cmp::min(ans, n - x);
        }
        
        // For each possible target count (from 1 up to n),
        // try to adjust the frequency counts.
        for k in 1..=n {
            // dp array for memoization, 26 elements (-1 represents uncomputed).
            let mut dp = vec![-1; 26];
            ans = cmp::min(ans, Self::dfs(&mut dp, &a, k, 0));
        }
        
        ans
    }

    // The recursive DFS function with memoization.
    // dp: memoization vector where dp[i] stores the minimal cost starting from letter index i.
    // a: frequency vector for letters 'a' to 'z'.
    // k: target frequency for a given letter adjustment scenario.
    // i: current letter index (0 for 'a', 1 for 'b', etc.).
    fn dfs(dp: &mut Vec<i32>, a: &Vec<i32>, k: i32, i: usize) -> i32 {
        // Base case: if we have processed all letters, no additional cost.
        if i >= a.len() {
            return 0;
        }

        // Return the cached answer if available.
        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;
        if a[i] >= k {
            // For the current letter: There are extra occurrences "more" than needed.
            let more = a[i] - k;
            // Option 1: Remove the extra occurrences here and continue.
            ans = cmp::min(ans, a[i] - k + Self::dfs(dp, a, k, i + 1));
            // Option 2: Try to merge adjustments with the next letter if possible.
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    // If combining extras with next letter meets or exceeds k.
                    ans = cmp::min(ans, more + Self::dfs(dp, a, k, i + 2));
                } else {
                    // Otherwise, choose the best possibility among two ways to compensate.
                    let candidate = (more
                        + (k - (a[i + 1] + more)).min(more + a[i + 1]))
                        + Self::dfs(dp, a, k, i + 2);
                    ans = cmp::min(ans, candidate);
                }
            }
        } else {
            // Case when a[i] is less than k:
            // Option 1: Either add or remove enough occurrences to make it k.
            ans = cmp::min(ans, (k - a[i]).min(a[i]) + Self::dfs(dp, a, k, i + 1));
            // Option 2: Consider changing the frequency along with the next letter if possible.
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = cmp::min(ans, a[i] + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = cmp::min(ans, a[i] + (k - (a[i] + a[i + 1])) + Self::dfs(dp, a, k, i + 2));
                }
            }
        }

        // Cache the computed result.
        dp[i] = ans;
        ans
    }
}

fn main() -> io::Result<()> {
    // Use a buffered reader for robust I/O handling.
    let stdin = io::stdin();
    let mut input_line = String::new();

    // Read one line and then get the first token.
    // This mimics "cin >> s" in C++ which reads the next whitespace-separated token.
    stdin.read_line(&mut input_line)?;
    let s = input_line.split_whitespace().next().unwrap_or("").to_string();

    // Call the solution function and compute the result.
    let result = Solution::make_string_good(s);

    // Output the final result exactly as the C++ program does.
    println!("{}", result);

    Ok(())
}