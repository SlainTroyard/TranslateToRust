use std::collections::HashMap;
use std::io::{self, BufRead};

const INF: i32 = 1_000_000;

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
        ans = ans.min(more + dfs(dp, a, k, i + 1));
        if i + 1 < a.len() && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = ans.min(more + dfs(dp, a, k, i + 2));
            } else {
                ans = ans.min(more + (k - (a[i + 1] + more)).min(more + a[i + 1]) + dfs(dp, a, k, i + 2));
            }
        }
    } else {
        ans = ans.min((k - a[i]).min(a[i]) + dfs(dp, a, k, i + 1));
        if i + 1 < a.len() && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = ans.min(a[i] + dfs(dp, a, k, i + 2));
            } else {
                ans = ans.min(a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn make_string_good(s: String) -> i32 {
    let n = s.len();
    let mut a = vec![0; 26];
    for c in s.chars() {
        a[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans = n as i32;
    for &x in &a {
        ans = ans.min(n as i32 - x);
    }

    for k in 1..=n {
        let mut dp = vec![-1; 26];
        ans = ans.min(dfs(&mut dp, &a, k as i32, 0));
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();

    let result = make_string_good(s);
    println!("{}", result);
}