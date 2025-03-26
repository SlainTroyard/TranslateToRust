use std::io;
use std::cmp::min;

const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    fn make_string_good(s: String) -> i32 {
        let n = s.len() as i32;
        let mut a = vec![0; 26];
        
        // Count frequency of each character
        for c in s.chars() {
            a[(c as usize) - ('a' as usize)] += 1;
        }
        
        // Initialize answer as the length of string
        let mut ans = n;
        
        // Try keeping a single character type
        for &x in &a {
            ans = min(ans, n - x);
        }
        
        // Try different values of k
        for k in 1..=n {
            let mut dp = vec![-1; 26];
            ans = min(ans, Self::dfs(&mut dp, &a, k, 0));
        }
        
        ans
    }
    
    fn dfs(dp: &mut Vec<i32>, a: &Vec<i32>, k: i32, i: usize) -> i32 {
        // Base case: reached end of characters
        if i >= a.len() {
            return 0;
        }
        
        // Return memoized result if available
        if dp[i] != -1 {
            return dp[i];
        }
        
        let mut ans = INF;
        
        // Case 1: We have enough characters (>= k)
        if a[i] >= k {
            let more = a[i] - k;
            // Option 1: Keep k of current character, remove excess
            ans = min(ans, more + Self::dfs(dp, a, k, i + 1));
            
            // Option 2: Consider combining with next character if possible
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = min(ans, more + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, min(more + k - (a[i + 1] + more), more + a[i + 1]) 
                             + Self::dfs(dp, a, k, i + 2));
                }
            }
        } 
        // Case 2: We don't have enough characters (< k)
        else {
            // Option 1: Add more or remove all of current character
            ans = min(ans, min(k - a[i], a[i]) + Self::dfs(dp, a, k, i + 1));
            
            // Option 2: Consider combining with next character if possible
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = min(ans, a[i] + Self::dfs(dp, a, k, i + 2));
                } else {
                    ans = min(ans, a[i] + (k - (a[i] + a[i + 1])) 
                             + Self::dfs(dp, a, k, i + 2));
                }
            }
        }
        
        // Memoize and return result
        dp[i] = ans;
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Trim whitespace and convert to string
    let s = input.trim().to_string();
    
    // Call solution method and print result
    let result = Solution::make_string_good(s);
    println!("{}", result);
    
    Ok(())
}