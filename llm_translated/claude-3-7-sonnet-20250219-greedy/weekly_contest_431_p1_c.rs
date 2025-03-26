use std::io::{self, BufRead};
use std::cmp::max;

/// Calculate the greatest common divisor of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculate the least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

/// Return the length of the longest product-equivalent subarray
fn max_length(nums: &[i32]) -> i32 {
    let mut max_len = 0;
    for i in 0..nums.len() {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;
        
        for j in i..nums.len() {
            // Check for potential overflow before multiplying
            if prod > i64::MAX / (nums[j] as i64) {
                break;
            }
            
            prod *= nums[j] as i64;
            g = gcd(g, nums[j] as i64);
            l = lcm(l, nums[j] as i64);
            
            if prod == l * g {
                let length = (j - i + 1) as i32;
                max_len = max(max_len, length);
            }
        }
    }
    max_len
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number size");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Calculate the result
    let result = max_length(&nums);
    println!("{}", result);
}