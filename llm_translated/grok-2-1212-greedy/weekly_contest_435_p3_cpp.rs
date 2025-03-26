use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    fn minimum_increments(nums: &[i64], target: &[i64]) -> i64 {
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
                f[i & 1][j] = f[(i & 1) ^ 1][j];
            }
            for j in 0..(1 << m) {
                for k in (1..=j).rev() {
                    if k & j == k {
                        let v = (nums[i - 1] + g[k] - 1) / g[k] * g[k] - nums[i - 1];
                        f[i & 1][j] = f[i & 1][j].min(f[(i & 1) ^ 1][j ^ k] + v);
                    }
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let nm: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    // Read nums
    let nums: Vec<i64> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read target
    let target: Vec<i64> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print result
    let solution = Solution;
    let result = solution.minimum_increments(&nums, &target);
    println!("{}", result);

    Ok(())
}