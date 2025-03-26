use std::io;

const INF: i32 = 1_000_000;

fn make_string_good(s: &str) -> i32 {
    let n = s.len() as i32;
    let mut a = vec![0; 26];
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        a[idx] += 1;
    }

    // Calculate the initial answer by considering the maximum possible deletions for each character
    let initial_ans = a.iter().map(|&x| n - x).min().unwrap_or(n);
    let mut ans = initial_ans;

    // Iterate over all possible k values to find the minimum operations needed
    for k in 1..=n {
        let mut dp = vec![-1; 26];
        let current_ans = dfs(&mut dp, &a, k, 0);
        ans = ans.min(current_ans);
    }

    ans
}

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
        // Option 1: Remove excess characters and proceed to next character
        ans = ans.min(more + dfs(dp, a, k, i + 1));

        // Check if next character can be merged with current excess
        if i + 1 < a.len() && a[i + 1] <= k {
            let sum_next = a[i + 1] + more;
            if sum_next >= k {
                // Merge and take the required steps
                let cost = more + dfs(dp, a, k, i + 2);
                ans = ans.min(cost);
            } else {
                // Need to add or remove to reach k
                let option_part = (more + (k - sum_next)).min(more + a[i + 1]);
                let cost = option_part + dfs(dp, a, k, i + 2);
                ans = ans.min(cost);
            }
        }
    } else {
        // Current character count is less than k, decide to add or remove
        let adjust = (k - a[i]).min(a[i]);
        ans = ans.min(adjust + dfs(dp, a, k, i + 1));

        // Check if next character can be merged with current
        if i + 1 < a.len() && a[i + 1] <= k {
            let sum = a[i] + a[i + 1];
            if sum >= k {
                // Merge and take the required steps
                let cost = a[i] + dfs(dp, a, k, i + 2);
                ans = ans.min(cost);
            } else {
                // Need to add characters to reach k
                let needed = k - sum;
                let cost = a[i] + needed + dfs(dp, a, k, i + 2);
                ans = ans.min(cost);
            }
        }
    }

    dp[i] = ans;
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    let result = make_string_good(s);
    println!("{}", result);
}