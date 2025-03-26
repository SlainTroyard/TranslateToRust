const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            a[idx] += 1;
        }

        let mut ans = n as i32;
        for &x in &a {
            ans = ans.min((n as i32) - x);
        }

        for k in 1..=n {
            let mut dp = vec![-1; 26];
            let current = dfs(&mut dp, &a, k as i32, 0);
            ans = ans.min(current);
        }

        ans
    }
}

fn dfs(dp: &mut [i32], a: &[i32], k: i32, i: usize) -> i32 {
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
                let option1 = more + (k - (a[i + 1] + more));
                let option2 = more + a[i + 1];
                let temp = option1.min(option2);
                ans = ans.min(temp + dfs(dp, a, k, i + 2));
            }
        }
    } else {
        let temp = (k - a[i]).min(a[i]);
        ans = ans.min(temp + dfs(dp, a, k, i + 1));
        if i + 1 < a.len() && a[i + 1] <= k {
            if a[i] + a[i + 1] >= k {
                ans = ans.min(a[i] + dfs(dp, a, k, i + 2));
            } else {
                let needed = k - (a[i] + a[i + 1]);
                ans = ans.min(a[i] + needed + dfs(dp, a, k, i + 2));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let s = input.trim().to_string();

    let result = Solution::make_string_good(s);
    println!("{}", result);
}