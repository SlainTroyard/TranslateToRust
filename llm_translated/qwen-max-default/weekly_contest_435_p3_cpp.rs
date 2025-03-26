use std::io::{self, BufRead, Write};
use std::cmp;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

struct Solution;

impl Solution {
    pub fn minimum_increments(nums: &Vec<i64>, target: &Vec<i64>) -> i64 {
        let n = nums.len();
        let m = target.len();

        let mut g = vec![1; 1 << m];
        for i in 0..(1 << m) {
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    g[i] = g[i] / gcd(g[i], target[j]) * target[j];
                }
            }
        }

        const INF: i64 = 1e18 as i64;
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;

        for i in 1..=n {
            for j in 0..(1 << m) {
                f[i & 1][j] = f[i & 1 ^ 1][j];
            }
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    let v = ((nums[i - 1] + g[k] - 1) / g[k] * g[k] - nums[i - 1]);
                    f[i & 1][j] = cmp::min(f[i & 1][j], f[i & 1 ^ 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the first line to get n and m
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");

    // Read the second line to get nums
    buffer.clear();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let nums: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse num"))
        .collect();

    // Read the third line to get target
    buffer.clear();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let target: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse target"))
        .collect();

    let solution = Solution;
    let result = solution.minimum_increments(&nums, &target);

    writeln!(stdout, "{}", result).expect("Failed to write result");
}