use std::cmp::min;
use std::io::{self, Write};

const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    pub fn make_string_good(s: &str) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];
        
        // Count the frequency of each character
        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = n as i32;

        // Minimize by removing all but one type of character
        for &x in &a {
            ans = min(ans, n as i32 - x);
        }

        // Try every possible "k" value
        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = min(ans, self.dfs(&mut dp, &a, k as i32, 0));
        }

        ans
    }

    fn dfs(&self, dp: &mut Vec<i32>, a: &[i32], k: i32, i: usize) -> i32 {
        if i >= a.len() {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;

        if a[i] >= k {
            let more = a[i] - k;
            ans = min(ans, more + self.dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = min(ans, more + self.dfs(dp, a, k, i + 2));
                } else {
                    ans = min(
                        ans,
                        min(more + k - (a[i + 1] + more), more + a[i + 1]) + self.dfs(dp, a, k, i + 2),
                    );
                }
            }
        } else {
            ans = min(ans, min(k - a[i], a[i]) + self.dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = min(ans, a[i] + self.dfs(dp, a, k, i + 2));
                } else {
                    ans = min(
                        ans,
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
    let mut s = String::new();
    let stdin = io::stdin();

    // Input the string
    stdin.read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    // Solve the problem
    let solution = Solution;
    let result = solution.make_string_good(s);

    // Output the result
    println!("{}", result);
}