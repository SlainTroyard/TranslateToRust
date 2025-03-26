use std::io::{self, Write};

const INF: i32 = 1_000_000;

fn dfs(dp: &mut Vec<i32>, a: &[i32], k: i32, i: usize) -> i32 {
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
                ans = ans.min(more + (k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2));
            }
        }
    } else {
        ans = ans.min((k - a[i]) + dfs(dp, a, k, i + 1));
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

fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = vec![0; 26];
    for c in s.chars() {
        a[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans = n as i32;
    for &count in &a {
        ans = ans.min(n as i32 - count);
    }

    for k in 1..=n {
        let mut dp = vec![-1; 26];
        ans = ans.min(dfs(&mut dp, &a, k as i32, 0));
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    let result = make_string_good(s);
    println!("{}", result);
}