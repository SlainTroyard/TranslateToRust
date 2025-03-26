use std::io::{self, BufRead};
use std::cmp;
use std::num::ParseIntError;

// Calculate the greatest common divisor (GCD) of two numbers
fn gcd(mut a: i32, mut b: i32) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a as i64
}

// Calculate the least common multiple (LCM) of two numbers
fn lcm(a: i32, b: i32) -> i64 {
    (a as i64 / gcd(a, b)) * b as i64
}

// Return the length of the longest subarray where the product equals LCM * GCD
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    let nums_size = nums.len();

    for i in 0..nums_size {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;

        for j in i..nums_size {
            // Prevent overflow
            if prod > i64::MAX / nums[j] as i64 {
                break;
            }

            prod *= nums[j] as i64;
            g = gcd(g as i32, nums[j]);
            l = lcm(l as i32, nums[j]);

            if prod == l * g {
                let length = (j - i + 1) as i32;
                max_length = cmp::max(max_length, length);
            }
        }
    }

    max_length
}

fn main() -> Result<(), ParseIntError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let num_size: usize = lines.next()
        .ok_or_else(|| ParseIntError::from(std::num::IntErrorKind::Empty))??
        .trim()
        .parse()?;

    // Read the array elements
    let nums: Vec<i32> = lines.next()
        .ok_or_else(|| ParseIntError::from(std::num::IntErrorKind::Empty))??
        .trim()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    // Ensure the input size matches the declared size
    assert_eq!(nums.len(), num_size, "Input size does not match declared size");

    // Calculate the result
    let result = max_length(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}