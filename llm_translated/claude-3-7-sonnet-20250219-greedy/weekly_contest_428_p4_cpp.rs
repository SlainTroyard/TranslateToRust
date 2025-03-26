use std::io;
use std::cmp::min;

const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];
        
        // Count frequency of each character
        for c in s.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }

        // Initialize answer as the length of the string
        let mut ans = n as i32;
        
        // Try removing all characters except one type
        for x in &a {
            ans = min(ans, n as i32 - x);
        }

        // Try different values of k
        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = min(ans, Self::dfs(&mut dp, &a, k as i32, 0));
        }

        ans
    }

    fn dfs(dp: &mut Vec<i32>, a: &Vec<i32>, k: i32, i: usize) -> i32 {
        if i >= a.len() {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = INF;
        if a[i] >= k {
            let more = a[i] - k;
            ans = min(ans, more + Self::dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = min(ans, more + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, min(more + k - (a[i + 1] + more), more + a[i + 1]) + Self::dfs(dp, a, k, i + 2));
                }
            }
        } else {
            ans = min(ans, min(k - a[i], a[i]) + Self::dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = min(ans, a[i] + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, a[i] + (k - (a[i] + a[i + 1])) + Self::dfs(dp, a, k, i + 2));
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() {
    let mut s = String::new();
    
    // Input the string
    io::stdin().read_line(&mut s).expect("Failed to read input");
    s = s.trim().to_string();
    
    // Call makeStringGood and output the result
    let result = Solution::make_string_good(s);
    println!("{}", result);
}