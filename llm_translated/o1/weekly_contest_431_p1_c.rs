// Weekly Contest 431 Problem 1 in Rust
// This program reads an integer (numSize) from stdin, then reads numSize integers.
// It calculates the length of the longest subarray for which the product of its elements
// is equal to the product of its GCD and LCM, then prints that length to stdout.

use std::io::{self, BufRead};

// Calculate the greatest common divisor (GCD) of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Return the length of the longest subarray for which the product of its elements
// is equal to the product of its GCD and LCM.
fn max_length(nums: &[i32]) -> i32 {
    let mut max_len = 0;
    for i in 0..nums.len() {
        let mut prod: i64 = 1;
        let mut g = nums[i] as i64;
        let mut l = nums[i] as i64;
        for j in i..nums.len() {
            // Prevent overflow: if prod * nums[j] would exceed i64::MAX, break
            let next_val = nums[j] as i64;
            if next_val != 0 && prod > i64::MAX / next_val {
                break;
            }

            prod *= next_val;
            g = gcd(g, next_val);
            l = lcm(l, next_val);

            // Check if the product equals l * g
            if prod == l * g {
                let length = (j - i + 1) as i32;
                if length > max_len {
                    max_len = length;
                }
            }
        }
    }
    max_len
}

fn main() -> io::Result<()> {
    // Read the size of the array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: numSize
    let num_size: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // Read numSize integers (potentially across multiple lines)
    let mut nums = Vec::with_capacity(num_size);
    while nums.len() < num_size {
        if let Some(Ok(line)) = lines.next() {
            for val in line.split_whitespace() {
                if let Ok(num) = val.parse::<i32>() {
                    nums.push(num);
                    if nums.len() == num_size {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }

    // Compute the result
    let result = max_length(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}