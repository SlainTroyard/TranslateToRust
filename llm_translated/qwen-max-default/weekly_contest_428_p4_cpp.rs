use std::io::{self, BufRead, Write};

const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    fn make_string_good(s: &str) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];

        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = n as i32;
        for x in &a {
            ans = ans.min(n as i32 - *x);
        }

        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = ans.min(Self::dfs(&mut dp, &a, k, 0));
        }

        ans
    }

    fn dfs(dp: &mut Vec<i32>, a: &[i32], k: usize, i: usize) -> i32 {
        if i >= a.len() {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;
        if a[i] >= k as i32 {
            let more = a[i] - k as i32;
            ans = ans.min(a[i] - k as i32 + Self::dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k as i32 {
                if a[i + 1] + more >= k as i32 {
                    ans = ans.min(more + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = ans.min((more + k as i32 - (a[i + 1] + more)).min(more + a[i + 1]) + Self::dfs(dp, a, k, i + 2));
                }
            }
        } else {
            ans = ans.min((k as i32 - a[i]).min(a[i]) + Self::dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k as i32 {
                if a[i + 1] + a[i] >= k as i32 {
                    ans = ans.min(a[i] + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = ans.min(a[i] + (k as i32 - (a[i] + a[i + 1])) + Self::dfs(dp, a, k, i + 2));
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the input string
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let s = buffer.trim(); // Trim the newline character

    // Call make_string_good and output the result
    let result = Solution::make_string_good(s);
    writeln!(stdout, "{}", result).expect("Failed to write result");

    // Clear the buffer for the next input
    buffer.clear();
}