use std::io;

const INF: i32 = 1_000_000;

/// Recursive function to compute the minimum cost using dynamic programming.
fn dfs(dp: &mut [i32; 26], a: &[i32; 26], k: i32, i: usize, size: usize) -> i32 {
    if i >= size {
        return 0;
    }
    if dp[i] != -1 {
        return dp[i];
    }
    let mut ans = INF;
    if a[i] as i32 >= k {
        // Current count is sufficient or more than k
        let more = a[i] as i32 - k;
        // Option 1: take current and move to next
        let option1 = more + dfs(dp, a, k, i + 1, size);
        ans = ans.min(option1);
        // Check next element if possible
        if i + 1 < size && a[i + 1] as i32 <= k {
            let next = a[i + 1] as i32;
            if next + more >= k {
                // Option 2: combine current and next to cover k
                let option2 = more + dfs(dp, a, k, i + 2, size);
                ans = ans.min(option2);
            } else {
                // Need to add deficit to reach k
                let deficit = k - (next + more);
                let option2 = more + deficit + dfs(dp, a, k, i + 2, size);
                ans = ans.min(option2);
            }
        }
    } else {
        // Current count is less than k, need to add deficit
        let deficit_initial = k - (a[i] as i32);
        let option1 = deficit_initial + dfs(dp, a, k, i + 1, size);
        ans = ans.min(option1);
        // Check next element if possible
        if i + 1 < size && a[i + 1] as i32 <= k {
            let next = a[i + 1] as i32;
            if a[i] as i32 + next >= k {
                // Option 2: combine current and next to reach k
                let option2 = (a[i] as i32) + dfs(dp, a, k, i + 2, size);
                ans = ans.min(option2);
            } else {
                // Need to add more to reach k
                let needed = k - (a[i] as i32 + next);
                let option2 = (a[i] as i32) + needed + dfs(dp, a, k, i + 2, size);
                ans = ans.min(option2);
            }
        }
    }
    dp[i] = ans;
    ans
}

/// Calculates the minimum operations to make the string "good".
fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = [0; 26];
    for c in s.bytes() {
        let idx = (c - b'a') as usize;
        a[idx] += 1;
    }
    let mut ans = n as i32;
    // Initial answer considering removing the most frequent character
    for &count in a.iter() {
        ans = ans.min(n as i32 - count);
    }
    // Iterate over possible k values to find the minimum
    for k in 1..=n {
        let k_i32 = k as i32;
        let mut dp = [-1; 26];
        let current = dfs(&mut dp, &a, k_i32, 0, 26);
        ans = ans.min(current);
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let s = input.trim_end();
    let result = make_string_good(s);
    println!("{}", result);
}