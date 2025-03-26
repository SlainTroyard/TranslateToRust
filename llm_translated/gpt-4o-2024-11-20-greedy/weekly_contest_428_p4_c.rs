use std::io::{self, Read};
use std::cmp::{min, max};

const INF: i32 = 1_000_000;

// Recursive DFS function to calculate the minimum cost
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
        ans = min(ans, more + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = min(ans, more + dfs(dp, a, k, i + 2, size));
            } else {
                ans = min(
                    ans,
                    more + k - (a[i + 1] + more) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    } else {
        ans = min(ans, (k - a[i]) + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = min(ans, a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = min(
                    ans,
                    a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    }

    dp[i] = ans;
    ans
}

// Function to calculate the minimum cost to make the string "good"
fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = [0; 26];
    for ch in s.chars() {
        a[(ch as u8 - b'a') as usize] += 1;
    }

    let mut ans = n as i32;
    for i in 0..26 {
        ans = min(ans, n as i32 - a[i]);
    }

    for k in 1..=n as i32 {
        let mut dp = [-1; 26];
        ans = min(ans, dfs(&mut dp, &a, k, 0, 26));
    }

    ans
}

fn main() {
    // Read input string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();

    // Call make_string_good and output the result
    let result = make_string_good(s);
    println!("{}", result);
}