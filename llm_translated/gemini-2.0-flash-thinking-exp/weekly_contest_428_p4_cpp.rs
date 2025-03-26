use std::io;
use std::io::BufRead;
use std::cmp::{min, max};

const INF: i32 = 1_000_000;

struct Solution {}

impl Solution {
    fn make_string_good(&self, s: String) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];
        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = n as i32;
        for x in &a {
            ans = min(ans, n as i32 - x);
        }

        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = min(ans, self.dfs(&mut dp, &a, k as i32, 0));
        }

        ans
    }

    fn dfs(&self, dp: &mut Vec<i32>, a: &Vec<i32>, k: i32, i: usize) -> i32 {
        if i >= a.len() {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;
        if a[i] >= k {
            let more = a[i] - k;
            ans = min(ans, a[i] - k + self.dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = min(ans, more + self.dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, min(more + k - (a[i + 1] + more), more + a[i + 1]) + self.dfs(dp, a, k, i + 2));
                }
            }

        } else {
            ans = min(ans, min(k - a[i], a[i]) + self.dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = min(ans, a[i] + self.dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, a[i] + (k - (a[i] + a[i + 1])) + self.dfs(dp, a, k, i + 2));
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() {
    let solution = Solution {};
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let s = iterator.next().unwrap().unwrap();

    let result = solution.make_string_good(s);
    println!("{}", result);
}