use std::io::{self, BufRead};

// Calculate the greatest common divisor of two numbers
fn gcd(a: i32, b: i32) -> i64 {
    let mut a = a as i64;
    let mut b = b as i64;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Calculate the least common multiple of two numbers
fn lcm(a: i32, b: i32) -> i64 {
    (a as i64 / gcd(a, b)) * b as i64
}

// Return the length of the longest product-equivalent subarray
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod = 1i64;
        let mut g = nums[i] as i64;
        let mut l = nums[i] as i64;
        for j in i..nums.len() {
            if prod > i64::MAX / nums[j] as i64 {
                break; // Prevent overflow
            }
            prod *= nums[j] as i64;
            g = gcd(g as i32, nums[j]) as i64;
            l = lcm(l as i32, nums[j]) as i64;
            if prod == l * g {
                let length = j - i + 1;
                if length > max_length {
                    max_length = length;
                }
            }
        }
    }
    max_length as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let num_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .take(num_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate the result
    let result = max_length(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}