use std::cmp;
use std::io::{self, BufRead};

/// Calculates the greatest common divisor of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

/// Calculates the least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

/// Calculates the maximum score according to the problem description
fn max_score(nums: &[i32]) -> i64 {
    let numsSize = nums.len();
    
    // Handle the special case where there's only one number
    if numsSize == 1 {
        return (nums[0] as i64) * (nums[0] as i64);
    }
    
    // Initialize arrays for storing lcm and gcd values
    let mut lcms = vec![0i64; numsSize];
    let mut gcds = vec![0i64; numsSize];
    
    // Set the last values
    lcms[numsSize - 1] = nums[numsSize - 1] as i64;
    gcds[numsSize - 1] = nums[numsSize - 1] as i64;
    
    // Fill the arrays from right to left
    for i in (0..numsSize - 1).rev() {
        lcms[i] = lcm(nums[i] as i64, lcms[i + 1]);
        gcds[i] = gcd(nums[i] as i64, gcds[i + 1]);
    }
    
    // Calculate the initial possible answers
    let mut ans = lcms[0] * gcds[0];
    ans = cmp::max(ans, lcms[1] * gcds[1]);
    
    // Try all possible combinations
    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;
    
    for i in 1..numsSize - 1 {
        ans = cmp::max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]), ans);
        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }
    
    // Check the last possible combination
    ans = cmp::max(ans, prelcm * pregcd);
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the array size
    let numsSize: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", max_score(&nums));
}