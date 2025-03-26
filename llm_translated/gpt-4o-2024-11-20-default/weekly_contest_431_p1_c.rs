use std::io;
use std::cmp::max;

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

// Returns the length of the longest subarray whose product equals the product of its GCD and LCM
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    let nums_size = nums.len();

    for i in 0..nums_size {
        let mut prod: i64 = 1;
        let mut g = nums[i] as i64;
        let mut l = nums[i] as i64;

        for j in i..nums_size {
            // Avoid overflow
            if prod > i64::MAX / nums[j] as i64 {
                break;
            }

            prod *= nums[j] as i64;
            g = gcd(g as i32, nums[j]);
            l = lcm(l as i32, nums[j]);

            if prod == l * g {
                let length = (j - i + 1) as i32;
                max_length = max(max_length, length);
            }
        }
    }

    max_length
}

fn main() {
    // Input handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read numsSize");
    let num_size: usize = input.trim().parse().expect("Input value must be an integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums array");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Values in nums must be integers"))
        .collect();

    if nums.len() != num_size {
        panic!("The number of input values does not match numsSize");
    }

    // Calculate the result
    let result = max_length(&nums);

    // Output the result
    println!("{}", result);
}