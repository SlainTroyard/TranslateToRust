const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    fn make_string_good(s: String) -> i32 {
        let mut a = [0; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            a[idx] += 1;
        }

        let n = s.len() as i32;
        let initial_min = a.iter().map(|x| n - x).min().unwrap_or(n);
        let mut ans = initial_min;

        for k in 1..=n {
            let mut dp = [-1; 26];
            let current = dfs(&mut dp, &a, k, 0);
            ans = ans.min(current);
        }

        ans
    }
}

fn dfs(dp: &mut [i32; 26], a: &[i32], k: i32, i: usize) -> i32 {
    if i >= a.len() {
        return 0;
    }
    if dp[i] != -1 {
        return dp[i];
    }

    let mut ans = INF;

    if a[i] >= k {
        let more = a[i] - k;
        let option1 = more + dfs(dp, a, k, i + 1);
        ans = ans.min(option1);

        if i + 1 < a.len() && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                let option2 = more + dfs(dp, a, k, i + 2);
                ans = ans.min(option2);
            } else {
                let option2a = more + (k - (a[i + 1] + more));
                let option2b = more + a[i + 1];
                let option2 = option2a.min(option2b) + dfs(dp, a, k, i + 2);
                ans = ans.min(option2);
            }
        }
    } else {
        let take = (k - a[i]).min(a[i]);
        let option1 = take + dfs(dp, a, k, i + 1);
        ans = ans.min(option1);

        if i + 1 < a.len() && a[i + 1] <= k {
            if a[i] + a[i + 1] >= k {
                let option2 = a[i] + dfs(dp, a, k, i + 2);
                ans = ans.min(option2);
            } else {
                let option2 = a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2);
                ans = ans.min(option2);
            }
        }
    }

    dp[i] = ans;
    ans
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();
    let solution = Solution;
    let result = solution.make_string_good(s);
    println!("{}", result);
}