use std::io::{self, Read};
use std::cmp;

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
        ans = cmp::min(ans, more + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + more >= k {
                ans = cmp::min(ans, more + dfs(dp, a, k, i + 2, size));
            } else {
                ans = cmp::min(
                    ans,
                    (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    } else {
        ans = cmp::min(ans, (k - a[i]) + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            if a[i + 1] + a[i] >= k {
                ans = cmp::min(ans, a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = cmp::min(
                    ans,
                    a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size),
                );
            }
        }
    }

    dp[i] = ans;
    return ans;
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len() as i32;
    let mut a = [0; 26];
    
    // Count frequency of each character
    for c in s.chars() {
        a[(c as u8 - b'a') as usize] += 1;
    }

    // Initialize answer as the minimum number of characters to remove
    // to have only one type of character
    let mut ans = n;
    for i in 0..26 {
        ans = cmp::min(ans, n - a[i]);
    }

    // Try different values of k
    for k in 1..=n {
        let mut dp = [-1; 26];
        ans = cmp::min(ans, dfs(&mut dp, &a, k, 0, 26));
    }

    ans
}

fn main() {
    // Read input string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    // Call make_string_good and output the result
    let result = make_string_good(s);
    println!("{}", result);
}