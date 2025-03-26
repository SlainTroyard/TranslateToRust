use std::cmp::min;
use std::io::{self, BufRead};

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Main function to calculate the minimum increments
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();
    
    // Calculate the g array - used for divisibility checks
    let mut g = vec![1i64; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                g[i] = g[i] / gcd(g[i], target[j] as i64) * target[j] as i64;
            }
        }
    }
    
    // Dynamic programming array
    const INF: i64 = 1e18 as i64;
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
                let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - nums[i - 1] as i64;
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }
    
    // Get the result
    f[n & 1][(1 << m) - 1] as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    
    // Read nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        nums.push(num);
    }
    
    // Read target array
    let mut target = Vec::with_capacity(m);
    for _ in 0..m {
        let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        target.push(t);
    }
    
    // Calculate the result
    let result = minimum_increments(&nums, &target);
    
    // Output the result
    println!("{}", result);
}