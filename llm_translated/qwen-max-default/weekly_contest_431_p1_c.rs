use std::io::{self, BufRead, Write};

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: isize, b: isize) -> isize {
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
fn lcm(a: isize, b: isize) -> isize {
    (a / gcd(a, b)) * b
}

// Function to find the length of the longest product-equivalent subarray
fn max_length(nums: &[isize]) -> isize {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod = 1;
        let mut g = nums[i];
        let mut l = nums[i];
        for j in i..nums.len() {
            if prod > isize::MAX / nums[j] {
                break; // Prevent overflow
            }
            prod *= nums[j];
            g = gcd(g, nums[j]);
            l = lcm(l, nums[j]);
            if prod == l * g {
                let length = (j - i + 1) as isize;
                if length > max_length {
                    max_length = length;
                }
            }
        }
    }
    max_length
}

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let mut num_size = String::new();
    stdin.lock().read_line(&mut num_size).expect("Failed to read line");
    let num_size: usize = num_size.trim().parse().expect("Please type a number!");

    // Read the array of numbers
    let mut nums = vec![0; num_size];
    for i in 0..num_size {
        let mut num = String::new();
        stdin.lock().read_line(&mut num).expect("Failed to read line");
        nums[i] = num.trim().parse().expect("Please type a number!");
    }

    // Calculate the result
    let result = max_length(&nums);

    // Print the result
    println!("{}", result);
}