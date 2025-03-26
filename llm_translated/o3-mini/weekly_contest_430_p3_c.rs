use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

/// Compute the greatest common divisor (GCD) of two integers using the Euclidean algorithm.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Computes the number of subsequences based on the given logic. 
/// This function has been directly translated from the original C code.
fn number_of_subsequences(nums: &[i32]) -> i64 {
    // Using a HashMap to replace the custom hash table implementation in C.
    // Keys are computed as: ((d / g) << 16) | (c / g) or ((a / g) << 16) | (b / g)
    let mut hash: HashMap<i32, i32> = HashMap::new();

    let n = nums.len();
    let mut ans: i64 = 0;

    // First loop: For i from 4 to nums.len() - 3 (since original range is [4, numsSize - 2))
    for i in 4..(n - 2) {
        let c = nums[i];
        for j in (i + 2)..n {
            let d = nums[j];
            let g = gcd(c, d);
            // Avoid division by zero, though given problem constraints, g should not be 0.
            let key = ((d / g) << 16) | (c / g);
            // Increment the count for this key by 1.
            *hash.entry(key).or_insert(0) += 1;
        }
    }

    // Process second part of the algorithm:
    // For i from 2 to nums.len() - 5 (since original C loop goes till numsSize-4, exclusive)
    for i in 2..(n - 4) {
        let b = nums[i];
        // First inner loop over j: from 0 to i-2 (i-1 in C, but Rust range is exclusive on end)
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            if let Some(&val) = hash.get(&key) {
                ans += val as i64;
            }
        }

        // Second inner loop: update the hash map with key= ((d/g)<<16) | (c/g) with -1
        let c = nums[i + 2];
        for j in (i + 4)..n {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split by whitespace to mimic the original scanf behavior which reads multiple integers
    let mut tokens = input.split_whitespace();

    // Parse the first integer which is n
    let n: usize = tokens
        .next()
        .ok_or("Expected first integer for n")?
        .parse()
        .map_err(|_| "Failed to parse n as an integer")?;

    // Parse the next n numbers into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens
            .next()
            .ok_or("Expected an integer in the input sequence")?
            .parse::<i32>()
            .map_err(|_| "Failed to parse a number as an integer")?;
        nums.push(num);
    }

    // Compute the result using the translated algorithm.
    let result = number_of_subsequences(&nums);

    // Print the result to stdout exactly as the original code did.
    println!("{}", result);

    Ok(())
}