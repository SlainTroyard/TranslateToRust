use std::cmp::{max, min};
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn max_difference(s: &str, k: usize) -> i32 {
        const INF: i32 = i32::MAX / 2;
        let mut ans = -INF;

        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }

                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[INF, INF], [INF, INF]];
                let mut left = 0;

                let chars: Vec<char> = s.chars().collect();
                for (i, &ch) in chars.iter().enumerate() {
                    cur_s[ch as usize - '0' as usize] += 1;
                    let r = i + 1;

                    while r - left >= k
                        && cur_s[x] > pre_s[x]
                        && cur_s[y] > pre_s[y]
                    {
                        let px = pre_s[x] & 1;
                        let py = pre_s[y] & 1;
                        min_s[px][py] = min(min_s[px][py], pre_s[x] - pre_s[y]);
                        pre_s[chars[left] as usize - '0' as usize] += 1;
                        left += 1;
                    }

                    let cx = cur_s[x] & 1;
                    let cy = cur_s[y] & 1;
                    ans = max(ans, cur_s[x] - cur_s[y] - min_s[cx ^ 1][cy]);
                }
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let k: usize = input.trim().parse().unwrap();

    // Solve the problem
    let solution = Solution;
    let result = solution.max_difference(&s, k);

    // Print the result to stdout
    println!("{}", result);
}