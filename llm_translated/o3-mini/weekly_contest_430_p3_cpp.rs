use std::collections::HashMap;
use std::io::{self, BufRead, Write};

// Helper function: Compute the greatest common divisor using Euclid's algorithm.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a.abs() // Ensure non-negative result.
}

fn number_of_subsequences(nums: &Vec<i32>) -> i64 {
    let n = nums.len();
    let mut suf: HashMap<u32, i32> = HashMap::new();

    // First loop: Build the suffix hashmap based on pairs (c, d)
    // Loop i from 4 to n-3 (because condition: i < n-2)
    for i in 4..(n - 2) {
        let c = nums[i];
        // j from i+2 to n-1 (because condition: j < n)
        for j in (i + 2)..n {
            let d = nums[j];
            let g = gcd(c, d);
            // Compute key: ((d / g) << 16) | (c / g)
            // Note: We cast to u32 for bit operations.
            let key = (((d / g) as u32) << 16) | ((c / g) as u32);
            *suf.entry(key).or_insert(0) += 1;
        }
    }

    let mut ans: i64 = 0;
    // Second loop: Process pairs (a, b) and update answer.
    // Loop i from 2 to n-5 (because condition: i < n - 4)
    for i in 2..(n - 4) {
        let b = nums[i];
        // For every j from 0 to i-2 (because condition: j < i-1)
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let key = (((a / g) as u32) << 16) | ((b / g) as u32);
            if let Some(&count) = suf.get(&key) {
                ans += count as i64;
            }
        }
        // Update the suffix map: For each pair (c, d), decrement the count.
        let c = nums[i + 2];
        // Loop j from i+4 to n-1 (because condition: j < n)
        for j in (i + 4)..n {
            let d = nums[j];
            let g = gcd(c, d);
            let key = (((d / g) as u32) << 16) | ((c / g) as u32);
            // Decrement the value in suf hashmap if it exists.
            if let Some(val) = suf.get_mut(&key) {
                *val -= 1;
            }
        }
    }
    ans
}

// Main function with exact same input/output interpretation as the original C++ code.
fn main() -> io::Result<()> {
    // Set up buffered input and output.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    let mut input = String::new();

    // Read the entire input into a single string.
    reader.read_to_string(&mut input)?;

    // Split the input into tokens, whitespace-separated.
    let mut tokens = input.split_whitespace();

    // Parse the first token as n (the size of the array).
    let n: usize = match tokens.next() {
        Some(token) => token.parse().expect("Failed to parse n"),
        None => return Ok(()),
    };

    // Parse the next n tokens as integers into the nums vector.
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(token) = tokens.next() {
            let num = token.parse().expect("Failed to parse a number");
            nums.push(num);
        } else {
            break;
        }
    }

    // Calculate the answer using the provided function.
    let result = number_of_subsequences(&nums);

    // Output the result with a newline.
    writeln!(writer, "{}", result)?;

    Ok(())
}