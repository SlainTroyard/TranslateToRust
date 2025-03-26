use std::io::{self, Read};
use std::error::Error;

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
fn minimum_increments(nums: &[i32], target: &[i32]) -> Result<i32, Box<dyn Error>> {
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
    const INF: i64 = 1e18 as i64;
    let mut f = vec![vec![INF; 1 << m]; 2];
    
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
    
    // Get the result
    Ok(f[n & 1][(1 << m) - 1] as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut first_line = lines.next().ok_or("Missing first line")?.split_whitespace();
    let n: usize = first_line.next().ok_or("Missing n")?.parse()?;
    let m: usize = first_line.next().ok_or("Missing m")?.parse()?;

    // Read arrays
    let nums: Vec<i32> = lines.by_ref().take(n).map(|line| line.parse()).collect::<Result<_, _>>()?;
    let target: Vec<i32> = lines.by_ref().take(m).map(|line| line.parse()).collect::<Result<_, _>>()?;

    // Call function to calculate result
    let result = minimum_increments(&nums, &target)?;

    // Output result
    println!("{}", result);

    Ok(())
}