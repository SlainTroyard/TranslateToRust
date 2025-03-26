use std::io::{self, BufRead};
use std::cmp;

// Function to calculate the greatest common divisor (GCD) of two numbers
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

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to find the length of the longest subarray where the product equals LCM * GCD
fn max_length(nums: &[i64]) -> i32 {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod = 1;
        let mut g = nums[i];
        let mut l = nums[i];
        for j in i..nums.len() {
            if prod > i64::MAX / nums[j] {
                break; // Prevent overflow
            }
            prod *= nums[j];
            g = gcd(g, nums[j]);
            l = lcm(l, nums[j]);
            if prod == l * g {
                let length = (j - i + 1) as i32;
                max_length = cmp::max(max_length, length);
            }
        }
    }
    max_length
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = max_length(&nums);

    // Print the result
    println!("{}", result);
}