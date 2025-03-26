use std::io;
use std::cmp;

const INF: i32 = 1000000;

fn dfs(dp: &mut Vec<i32>, a: &[i32], k: i32, i: usize, size: usize) -> i32 {
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
                ans = cmp::min(ans, (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2, size));
            }
        }
    } else {
        ans = cmp::min(ans, (k - a[i]) + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = cmp::min(ans, a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = cmp::min(ans, (a[i] + (k - (a[i] + a[i + 1]))) + dfs(dp, a, k, i + 2, size));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = [0; 26];
    for char in s.chars() {
        a[(char as u8 - b'a') as usize] += 1;
    }

    let mut ans = n as i32;
    for i in 0..26 {
        ans = cmp::min(ans, (n as i32) - a[i]);
    }

    for k in 1..=n {
        let mut dp = vec![-1; 26];
        ans = cmp::min(ans, dfs(&mut dp, &a, k as i32, 0, 26));
    }

    ans
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let result = make_string_good(s);
    println!("{}", result);
}