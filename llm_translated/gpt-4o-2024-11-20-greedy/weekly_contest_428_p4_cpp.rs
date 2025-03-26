use std::io;
use std::cmp;

const INF: i32 = 1_000_000;

struct Solution;

impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let n = s.len();
        let mut a = vec![0; 26];
        
        // Count character frequencies
        for c in s.chars() {
            a[c as usize - 'a' as usize] += 1;
        }

        // Start with the minimum value being the length of the string
        let mut ans = n as i32;

        // Calculate minimum operations to make all frequencies equal
        for x in &a {
            ans = cmp::min(ans, n as i32 - x);
        }

        // Try every k from 1 to n (inclusive)
        for k in 1..=n {
            let mut dp = vec![-1; 26]; // Use -1 as the uninitialized state
            ans = cmp::min(ans, Solution::dfs(&mut dp, &a, k as i32, 0));
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
            ans = cmp::min(ans, a[i] - k + Solution::dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + more >= k {
                    ans = cmp::min(ans, more + Solution::dfs(dp, a, k, i + 2));
                } else {
                    ans = cmp::min(
                        ans,
                        cmp::min(more + k - (a[i + 1] + more), more + a[i + 1])
                            + Solution::dfs(dp, a, k, i + 2),
                    );
                }
            }
        } else {
            ans = cmp::min(ans, cmp::min(k - a[i], a[i]) + Solution::dfs(dp, a, k, i + 1));
            
            if i + 1 < a.len() && a[i + 1] <= k {
                if a[i + 1] + a[i] >= k {
                    ans = cmp::min(ans, a[i] + Solution::dfs(dp, a, k, i + 2));
                } else {
                    ans = cmp::min(
                        ans,
                        a[i] + (k - (a[i] + a[i + 1])) + Solution::dfs(dp, a, k, i + 2),
                    );
                }
            }
        }

        dp[i] = ans;
        ans
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    // Solve the problem
    let solution = Solution;
    let result = solution.make_string_good(s);

    // Output the result
    println!("{}", result);
}