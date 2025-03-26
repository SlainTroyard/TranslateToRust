use std::io::{self, BufRead};
use std::cmp;

// Calculate the greatest common divisor
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Main function implementation
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();
    
    // Calculate g array - used for divisibility calculations
    let mut g = vec![1i64; 1 << m];
    
    for i in 0..(1 << m) {
        g[i] = 1;
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                let target_j = target[j] as i64;
                g[i] = g[i] / gcd(g[i], target_j) * target_j;
            }
        }
    }
    
    // Dynamic programming array
    const INF: i64 = 1_000_000_000_000_000_000; // 1e18
    let mut f = vec![vec![INF; 1 << m]; 2];
    
    // Initialize DP array
    f[0][0] = 0;
    
    // Dynamic programming process
    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }
        
        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let nums_i = nums[i - 1] as i64;
                let v = ((nums_i + g[k] - 1) / g[k]) * g[k] - nums_i;
                f[i & 1][j] = cmp::min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                
                k = (k - 1) & j;
            }
        }
    }
    
    // Get result
    f[n & 1][(1 << m) - 1] as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read target array
    let target_line = lines.next().unwrap()?;
    let target: Vec<i32> = target_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have the right number of elements
    assert_eq!(nums.len(), n);
    assert_eq!(target.len(), m);
    
    // Call function to calculate result
    let result = minimum_increments(&nums, &target);
    
    // Output result
    println!("{}", result);
    
    Ok(())
}