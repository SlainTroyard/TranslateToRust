use std::io::{self, Read};

const INF: i32 = 1000000;

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
        ans = ans.min(a[i] - k + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = ans.min(more + dfs(dp, a, k, i + 2, size));
            } else {
                ans = ans.min(more + k - (a[i + 1] + more) + dfs(dp, a, k, i + 2, size));
            }
        }
    } else {
        ans = ans.min(k - a[i] + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = ans.min(a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = ans.min(a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len() as i32;
    let mut a = [0; 26];
    for c in s.chars() {
        a[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans = n;
    for i in 0..26 {
        ans = ans.min(n - a[i]);
    }

    for k in 1..=n {
        let mut dp = [-1; 26];
        ans = ans.min(dfs(&mut dp, &a, k, 0, 26));
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let s = input.trim();

    let result = make_string_good(s);
    println!("{}", result);

    Ok(())
}