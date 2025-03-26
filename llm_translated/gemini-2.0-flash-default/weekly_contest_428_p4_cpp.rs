use std::cmp::{min, max};
use std::io;
use std::io::Read;

const INF: i32 = 1_000_000;

struct Solution {}

impl Solution {
    fn make_string_good(s: String) -> i32 {
        let n = s.len() as i32;
        let mut a = vec![0; 26];
        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = n;
        for &x in &a {
            ans = min(ans, n - x);
        }

        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = min(ans, Self::dfs(&mut dp, &a, k, 0));
        }

        ans
    }

    fn dfs(dp: &mut Vec<i32>, a: &Vec<i32>, k: i32, i: usize) -> i32 {
        if i >= a.len() {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;
        if a[i] >= k {
            let more = a[i] - k;
            ans = min(ans, a[i] - k + Self::dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = min(ans, more + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, min(more + k - (a[i + 1] + more), more + a[i + 1]) + Self::dfs(dp, a, k, i + 2));
                }
            }

        } else {
            ans = min(ans, min(k - a[i], a[i]) + Self::dfs(dp, a, k, i + 1));
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = min(ans, a[i] + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, a[i] + (k - (a[i] + a[i + 1])) + Self::dfs(dp, a, k, i + 2));
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let mut lines = s.lines();
    let input_string = lines.next().unwrap().to_string();

    let solution = Solution {};
    let result = solution.make_string_good(input_string);
    println!("{}", result);

    Ok(())
}