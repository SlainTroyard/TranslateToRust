use std::io::{self, BufRead};
use std::cmp::min;
use std::num::ParseIntError;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    pub fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = target.len();

        // Calculate g array - LCM for each subset of target
        let mut g = vec![1i64; 1 << m];
        for i in 0..(1 << m) {
            g[i] = 1;
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    // Calculate LCM: lcm(a,b) = a * b / gcd(a,b)
                    g[i] = g[i] / gcd(g[i], target[j] as i64) * (target[j] as i64);
                }
            }
        }

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;

        for i in 1..=n {
            // Copy previous state
            for j in 0..(1 << m) {
                f[i & 1][j] = f[(i & 1) ^ 1][j];
            }
            
            // Dynamic programming transition
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - (nums[i - 1] as i64);
                    f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }

        f[n & 1][(1 << m) - 1] as i32
    }
}

fn main() -> Result<(), ParseIntError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and m
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse()?;
    let m: usize = iter.next().unwrap().parse()?;
    
    // Parse nums array
    let line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = line.split_whitespace()
        .take(n)
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    
    // Parse target array
    let line = lines.next().unwrap().unwrap();
    let target: Vec<i32> = line.split_whitespace()
        .take(m)
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    
    // Solve and output result
    let solution = Solution::minimum_increments(nums, target);
    println!("{}", solution);
    
    Ok(())
}