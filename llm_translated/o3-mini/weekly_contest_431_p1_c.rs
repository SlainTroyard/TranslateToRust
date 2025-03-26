use std::io::{self, BufRead};

/// Compute the greatest common divisor (GCD) of two numbers.
/// This is the standard Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Compute the least common multiple (LCM) of two numbers.
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

/// Returns the length of the longest subarray
/// where product equals `lcm * gcd`.
fn max_length(nums: &[i32]) -> i32 {
    let nums_len = nums.len();
    let mut max_length = 0;
    
    // Iterate over all possible starting indices
    for i in 0..nums_len {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;
        // Iterate over all subarrays starting at index i
        for j in i..nums_len {
            let num = nums[j] as i64;

            // Check for potential overflow before multiplication.
            if prod > i64::MAX / num {
                break;
            }
            prod *= num;
            g = gcd(g, num);
            l = lcm(l, num);
            // If the subarray product equals lcm * gcd, update max_length.
            if prod == l * g {
                let length = j - i + 1;
                if length as i32 > max_length {
                    max_length = length as i32;
                }
            }
        }
    }
    max_length
}

fn main() -> io::Result<()> {
    // Create a buffered stdin reader.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the first line to get the number of elements.
    reader.read_line(&mut input)?;
    // Trim and parse the number of elements.
    let num_size: usize = input.trim().parse().expect("Failed to parse size");
    
    // Clear the buffer to read numbers.
    input.clear();
    // Read the next line which contains the numbers.
    // The input may be in multiple lines or on a single line.
    // We collect tokens until we have enough.
    let mut nums: Vec<i32> = Vec::with_capacity(num_size);
    while nums.len() < num_size {
        input.clear();
        if reader.read_line(&mut input)? == 0 {
            break;
        }
        // Split the line on whitespace and parse each token.
        for token in input.split_whitespace() {
            if nums.len() >= num_size {
                break;
            }
            let n: i32 = token.parse().expect("Failed to parse integer");
            nums.push(n);
        }
    }
    
    // Calculate the result using the translated algorithm.
    let result = max_length(&nums);
    // Print the result followed by a newline.
    println!("{}", result);
    Ok(())
}