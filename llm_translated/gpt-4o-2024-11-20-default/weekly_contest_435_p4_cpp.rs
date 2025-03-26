use std::cmp::{max, min};
use std::i32::{MAX as INT_MAX};
use std::io::{self, BufRead};

// Struct equivalent to the Solution class in C++
struct Solution;

impl Solution {
    pub fn max_difference(s: &str, k: usize) -> i32 {
        const INF: i32 = INT_MAX / 2; // Avoid overflow by dividing INT_MAX
        let mut ans = -INF;

        // Iterate over all pairs of characters (x, y) where x != y
        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }

                // State tracking for current and previous counts of each digit (0..4)
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                // Minimum differences for odd/even counts of x and y
                let mut min_s = [[INF, INF], [INF, INF]];
                let mut left = 0;

                let chars: Vec<char> = s.chars().collect(); // Convert string to a vector of chars
                for i in 0..chars.len() {
                    // Update current counts
                    cur_s[chars[i] as usize - '0' as usize] += 1;

                    // Expand the sliding window
                    let r = i + 1;
                    while r - left >= k &&
                          cur_s[x] > pre_s[x] &&
                          cur_s[y] > pre_s[y] {
                        // Update minimum differences
                        let p = &mut min_s[pre_s[x] & 1][pre_s[y] & 1];
                        *p = min(*p, pre_s[x] - pre_s[y]);

                        // Shrink window from the left
                        pre_s[chars[left] as usize - '0' as usize] += 1;
                        left += 1;
                    }

                    // Compute the current difference and try to maximize the result
                    ans = max(
                        ans,
                        cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1],
                    );
                }
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line is the string `s`
    let s = lines.next().unwrap().unwrap();

    // Second line is the integer `k`
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Compute the solution
    let sol = Solution;
    let result = sol.max_difference(&s, k);

    // Print result to stdout
    println!("{}", result);
}