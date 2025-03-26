use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = target.len();
        
        // Greatest common divisor helper function
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        
        // Precompute g array (LCM values for different combinations of targets)
        let mut g = vec![1i64; 1 << m];
        for i in 0..(1 << m) {
            g[i] = 1;
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    g[i] = g[i] / gcd(g[i], target[j] as i64) * (target[j] as i64);
                }
            }
        }
        
        // Define dp array and constants
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;
        
        // Dynamic programming to find minimum increments
        for i in 1..=n {
            // Copy previous state
            for j in 0..(1 << m) {
                f[i & 1][j] = f[(i & 1) ^ 1][j];
            }
            
            // Update dp values
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    // Calculate cost to increment nums[i-1] to match requirements
                    let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - (nums[i - 1] as i64);
                    f[i & 1][j] = f[i & 1][j].min(f[(i & 1) ^ 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }
        
        f[n & 1][(1 << m) - 1] as i32
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and m
    let nm_line = lines.next().unwrap().unwrap();
    let mut nm_iter = nm_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().unwrap();
    let m: usize = nm_iter.next().unwrap().parse().unwrap();
    
    // Parse nums vector
    let mut nums = vec![0; n];
    for i in 0..n {
        nums[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Parse target vector
    let mut target = vec![0; m];
    for i in 0..m {
        target[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Call the solution and print the result
    println!("{}", Solution::minimum_increments(nums, target));
}