use std::io::{self, BufRead};

// Calculate greatest common divisor (GCD) of two integers
fn gcd(a: i32, b: i32) -> i64 {
    let mut a = a.abs() as i64;
    let mut b = b.abs() as i64;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Calculate least common multiple (LCM) of two integers
fn lcm(a: i32, b: i32) -> i64 {
    let gcd_val = gcd(a, b);
    (a as i64 / gcd_val) * b as i64
}

// Find maximum length of subarray where product equals LCM * GCD
fn max_length(nums: &[i32]) -> i32 {
    let mut max_len = 0;
    for i in 0..nums.len() {
        let mut prod: i64 = 1;
        let mut current_gcd = nums[i] as i64;
        let mut current_lcm = nums[i] as i64;
        
        for j in i..nums.len() {
            let num = nums[j] as i64;
            
            // Prevent division by zero and overflow check
            if num == 0 || prod > i64::MAX / num {
                break;
            }
            prod *= num;
            
            // Update GCD and LCM for current subarray
            current_gcd = gcd(current_gcd as i32, nums[j]);
            current_lcm = lcm(current_lcm as i32, nums[j]);
            
            if prod == current_gcd * current_lcm {
                max_len = max_len.max((j - i + 1) as i32);
            }
        }
    }
    max_len
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    // Read array size from first line
    let n: usize = lines.next()
        .and_then(|s| s.trim().parse().ok())
        .expect("Invalid input size");
    
    // Read array elements from subsequent lines
    let nums: Vec<i32> = lines.flat_map(|s| 
        s.split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
    ).take(n).collect();
    
    println!("{}", max_length(&nums));
}