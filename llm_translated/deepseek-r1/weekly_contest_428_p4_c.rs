use std::io;

const INF: i32 = 1_000_000;

fn dfs(dp: &mut [i32], a: &[i32], k: i32, i: usize) -> i32 {
    if i >= a.len() {
        return 0;
    }

    if dp[i] != -1 {
        return dp[i];
    }

    let current = a[i];
    let mut ans = INF;

    if current >= k {
        let more = current - k;
        // Option 1: Adjust current to k and proceed
        ans = ans.min(more + dfs(dp, a, k, i + 1));

        // Check if combining with next character is possible
        if i + 1 < a.len() && a[i + 1] <= k {
            let next_val = a[i + 1];
            let combined = next_val + more;
            if combined >= k {
                ans = ans.min(more + dfs(dp, a, k, i + 2));
            } else {
                let cost = more + (k - combined) + dfs(dp, a, k, i + 2);
                ans = ans.min(cost);
            }
        }
    } else {
        // Current count is less than k, need to add
        ans = ans.min((k - current) + dfs(dp, a, k, i + 1));

        // Check combination possibility with next
        if i + 1 < a.len() && a[i + 1] <= k {
            let next_val = a[i + 1];
            let combined = current + next_val;
            if combined >= k {
                ans = ans.min(current + dfs(dp, a, k, i + 2));
            } else {
                let needed = k - combined;
                ans = ans.min(current + needed + dfs(dp, a, k, i + 2));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len() as i32;
    let mut char_counts = vec![0; 26];
    for c in s.chars() {
        let idx = c as usize - 'a' as usize;
        char_counts[idx] += 1;
    }

    // Initialize with the case of removing all but one character type
    let mut min_operations = n;
    for &count in &char_counts {
        min_operations = min_operations.min(n - count);
    }

    // Check all possible k values from 1 to string length
    for k in 1..=n {
        let mut dp = vec![-1; 26];
        let current = dfs(&mut dp, &char_counts, k, 0);
        min_operations = min_operations.min(current);
    }

    min_operations
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let result = make_string_good(input);
    println!("{}", result);
}