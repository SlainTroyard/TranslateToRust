use std::io;
use std::cmp;
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
        
        // Calculate g array - LCM of various combinations of target values
        let mut g = vec![1i64; 1 << m];
        for i in 0..(1 << m) {
            g[i] = 1;
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    g[i] = g[i] / gcd(g[i], target[j] as i64) * (target[j] as i64);
                }
            }
        }
        
        // Dynamic programming to find minimum increments
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;
        
        for i in 1..=n {
            // Copy values from previous state
            for j in 0..(1 << m) {
                f[i & 1][j] = f[(i & 1) ^ 1][j];
            }
            
            // Update current state
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - nums[i - 1] as i64;
                    f[i & 1][j] = cmp::min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }
        
        f[n & 1][(1 << m) - 1] as i32
    }
}

fn main() -> Result<(), ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n: usize = parts[0].parse()?;
    let m: usize = parts[1].parse()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    
    // Ensure we have the right number of elements
    assert_eq!(nums.len(), n);
    assert_eq!(target.len(), m);
    
    let solution = Solution::minimum_increments(nums, target);
    println!("{}", solution);
    
    Ok(())
}