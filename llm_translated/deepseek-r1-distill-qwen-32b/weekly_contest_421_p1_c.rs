// Problem: Weekly Contest 421 Problem 1
use std::io;

// Compute the greatest common divisor (GCD) using the Euclidean algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Compute the least common multiple (LCM) using the formula LCM(a, b) = (a / GCD(a, b)) * b
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Compute the maximum score based on the given array of integers
fn max_score(nums: &[i64]) -> i64 {
    let n = nums.len();
    if n == 1 {
        return nums[0] * nums[0];
    }

    let mut lcms = vec![0; n];
    let mut gcds = vec![0; n];
    
    lcms[n - 1] = nums[n - 1];
    gcds[n - 1] = nums[n - 1];
    
    for i in (0..n-1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }
    
    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);
    
    let mut prelcm = nums[0];
    let mut pregcd = nums[0];
    
    for i in 1..n-1 {
        let current = gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]);
        ans = ans.max(current);
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }
    
    ans = ans.max(prelcm * pregcd);
    
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input.trim().parse().expect("Invalid input size");
    
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read number");
        let num: i64 = num_str.trim().parse().expect("Invalid number");
        nums.push(num);
    }
    
    let result = max_score(&nums);
    println!("{}", result);
}