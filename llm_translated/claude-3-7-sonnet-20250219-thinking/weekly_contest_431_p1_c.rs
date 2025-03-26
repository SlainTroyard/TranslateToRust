use std::io::{self, Read};
use std::cmp;

// Calculate the greatest common divisor of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Calculate the least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Return the length of the longest product-equivalent subarray
fn max_length(nums: &[i32], nums_size: usize) -> i32 {
    let mut max_length = 0;
    for i in 0..nums_size {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;
        
        for j in i..nums_size {
            // Prevent overflow
            if prod > i64::MAX / (nums[j] as i64) {
                break;
            }
            
            prod *= nums[j] as i64;
            g = gcd(g, nums[j] as i64);
            l = lcm(l, nums[j] as i64);
            
            if prod == l * g {
                let length = (j - i + 1) as i32;
                max_length = cmp::max(max_length, length);
            }
        }
    }
    max_length
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_size: usize = input.trim().parse().expect("Invalid input");
    
    // Read the numbers into an array
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid number");
        nums.push(num);
    }

    // Calculate and print the result
    let result = max_length(&nums, num_size);
    println!("{}", result);
}