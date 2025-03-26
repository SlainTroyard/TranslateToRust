use std::cmp;
use std::io::{self, BufRead};

// Helper function equivalent to C's maxValue function
fn max_value(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

// Computes the max subarray sum based on the given algorithm
fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur: i64 = 0;
    // Create a vector to store the sum of every subarray of length k
    let mut keep = vec![0i64; n - k + 1];

    // Build the sliding window sum for subarray of size k
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            // Save the current window sum to keep and slide the window
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    // Initialize answer to the minimum i64 value, similar to LONG_MIN in C
    let mut ans = i64::MIN;

    // Loop over starting indices modulo k (or until end of keep vector)
    for i in 0..cmp::min(k, keep.len()) {
        cur = 0;
        // Start with the first window sum of the current series
        let mut max_here = keep[i];

        // For each sequence starting at i, step by k
        let mut l = i;
        while l < keep.len() {
            cur += keep[l];
            if cur > max_here {
                max_here = cur;
            }
            if cur < 0 {
                cur = 0;
            }
            l += k;
        }
        ans = max_value(ans, max_here);
    }
    ans
}

fn main() -> io::Result<()> {
    // Use a buffered reader for standard input
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input into a string.
    reader.read_to_string(&mut input)?;
    // Split the input by whitespace and create an iterator
    let mut tokens = input.split_whitespace();

    // Get the array size n
    let n = tokens
        .next()
        .expect("Expected an integer for n")
        .parse::<usize>()
        .expect("Failed to parse n as usize");

    // Get k
    let k = tokens
        .next()
        .expect("Expected an integer for k")
        .parse::<usize>()
        .expect("Failed to parse k as usize");

    // Read n integers into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens
            .next()
            .expect("Expected an integer in the array")
            .parse::<i32>()
            .expect("Failed to parse number as i32");
        nums.push(num);
    }

    // Compute the result using the given algorithm.
    let result = max_subarray_sum(&nums, n, k);

    // Print the result in the same format as the original C program.
    println!("{}", result);

    Ok(())
}