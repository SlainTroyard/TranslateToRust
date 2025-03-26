use std::cmp;
use std::io;

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
                    (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2, size)
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
                    a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size)
                );
            }
        }
    }

    dp[i] = ans;
    return ans;
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    let mut a = [0; 26];
    
    // Count the frequency of each character
    for c in s.chars() {
        a[(c as u8 - b'a') as usize] += 1;
    }

    // Initialize answer with the string length
    let mut ans = n as i32;
    
    // Try to keep only one character
    for i in 0..26 {
        ans = cmp::min(ans, n as i32 - a[i]);
    }

    // Try each k value
    for k in 1..=n {
        let mut dp = [-1; 26];
        ans = cmp::min(ans, dfs(&mut dp, &a, k as i32, 0, 26));
    }

    return ans;
}

fn main() {
    let mut s = String::new();
    
    // Read the input string
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();
    
    // Call make_string_good and output the result
    let result = make_string_good(s);
    println!("{}", result);
}