use std::io::{self, Read};
use std::cmp;

// A large constant to represent infinity for this problem
const INF: i32 = 1_000_000;

// Recursive DFS function with memoization
fn dfs(dp: &mut [i32], a: &[i32], k: i32, i: usize, size: usize) -> i32 {
    if i >= size {
        return 0;
    }

    if dp[i] != -1 {
        return dp[i];
    }

    let mut ans = INF;
    if a[i] >= k {
        let more = a[i] - k;
        ans = cmp::min(ans, more + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = cmp::min(ans, more + dfs(dp, a, k, i + 2, size));
            } else {
                ans = cmp::min(
                    ans,
                    more + k - (a[i + 1] + more) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    } else {
        ans = cmp::min(ans, (k - a[i]) + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = cmp::min(ans, a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = cmp::min(
                    ans,
                    a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    }

    dp[i] = ans;
    ans
}

// Main function to balance the string
fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = vec![0; 26];
    
    // Count the frequency of each character
    for ch in s.chars() {
        a[ch as usize - 'a' as usize] += 1;
    }

    let mut ans = n as i32;
    
    // Try removing all characters except one type
    for i in 0..26 {
        ans = cmp::min(ans, n as i32 - a[i]);
    }

    // Try balancing strings with different values of k
    for k in 1..=n as i32 {
        let mut dp = vec![-1; 26];
        ans = cmp::min(ans, dfs(&mut dp, &a, k, 0, 26));
    }

    ans
}

fn main() {
    let mut input = String::new();

    // Read input string from stdin
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Trim input to remove any excess whitespace
    let s = input.trim();

    // Call the processing function and print the result
    let result = make_string_good(s);
    println!("{}", result);
}