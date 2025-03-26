use std::io::{self, BufRead};

// Calculate the greatest common divisor
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

// Return the smaller of two numbers
fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

// Main function implementation
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();
    
    // Calculate g array - used for divisibility calculation
    let mut g = vec![1; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i & (1 << j)) != 0 {
                g[i] = g[i] / gcd(g[i], target[j] as i64) * target[j] as i64;
            }
        }
    }
    
    // Dynamic programming array
    let inf = 1e18 as i64;
    let mut f = vec![vec![inf; 1 << m]; 2];
    
    // Initialize DP array
    f[0][0] = 0;
    
    // Dynamic programming process
    for i in 1..=n {
        f[i & 1] = f[(i & 1) ^ 1].clone();
        
        for j in 0..(1 << m) {
            for k in (1..=j).rev() {
                if k & j == k {
                    let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - nums[i - 1] as i64;
                    f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                }
            }
        }
    }
    
    // Get result
    f[n & 1][(1 << m) - 1] as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Read target array
    let target_line = lines.next().unwrap()?;
    let target: Vec<i32> = target_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Call function to calculate result
    let result = minimum_increments(&nums, &target);

    // Output result
    println!("{}", result);

    Ok(())
}