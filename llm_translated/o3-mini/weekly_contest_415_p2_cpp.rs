use std::error::Error;
use std::io::{self, BufRead};

// Define a struct to mimic the C++ Solution class
struct Solution;

impl Solution {
    // This function implements the algorithm described in the C++ code.
    // It takes two vectors, `a` and `b`, and computes the maximum score by 
    // dynamic programming using an array `f` of size 5.
    //
    // The dp array `f` is initialized with:
    // f[0] = 0
    // f[1] through f[4] = LLONG_MIN / 2 (to avoid overflow when adding)
    //
    // For each element y in vector `b`, the dp state is updated in reverse order.
    fn max_score(a: Vec<i64>, b: Vec<i64>) -> i64 {
        // Initialize dp array f with 5 values.
        // f[0] = 0 and f[1..5] = i64::MIN / 2 to avoid potential overflow.
        let mut f = [0i64; 5];
        for i in 1..5 {
            f[i] = i64::MIN / 2;
        }

        // Process each element from vector `b`
        for y in b {
            // Update dp array in reverse to avoid interfering with previous state.
            for j in (0..4).rev() {
                // candidate value using current state and multiplication with a[j]
                let candidate = f[j] + a[j] * y;
                if candidate > f[j + 1] {
                    f[j + 1] = candidate;
                }
            }
        }
        // Final answer is stored in f[4]
        f[4]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin. This follows the same logic as the C++ code,
    // which reads aSize, bSize, and then the two sequences of numbers.
    let stdin = io::stdin();
    let mut input = String::new();
    for line in stdin.lock().lines() {
        let line = line?;
        input.push_str(&line);
        // Add a space to ensure tokens are properly separated.
        input.push(' ');
    }
    let mut tokens = input.split_whitespace();

    // Parse aSize and bSize: first two integers from input
    let a_size: usize = tokens
        .next()
        .ok_or("Missing input for aSize")?
        .parse()?;
    let b_size: usize = tokens
        .next()
        .ok_or("Missing input for bSize")?
        .parse()?;

    // Read vector a with a_size integers
    let mut a = Vec::with_capacity(a_size);
    for _ in 0..a_size {
        let token = tokens.next().ok_or("Missing an element in vector a")?;
        let num = token.parse()?;
        a.push(num);
    }

    // Read vector b with b_size integers
    let mut b = Vec::with_capacity(b_size);
    for _ in 0..b_size {
        let token = tokens.next().ok_or("Missing an element in vector b")?;
        let num = token.parse()?;
        b.push(num);
    }

    // Compute the result using the max_score function
    let result = Solution::max_score(a, b);

    // Output the result
    println!("{}", result);

    Ok(())
}