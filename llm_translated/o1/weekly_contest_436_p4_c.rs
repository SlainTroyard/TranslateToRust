// Problem: Weekly Contest 436 Problem 4
// Translated from C to Rust, preserving the same logic and I/O format.
//
// This program reads two integers n, m from stdin, then reads n integers
// for the points array, then computes the result using the same binary
// search approach as the original C code, and prints the result to stdout.
//
// Error handling is done in a similar style to the C version: if input
// parsing fails, an error message is printed to stderr and the program
// returns with exit code 1.

use std::io::{self, BufRead};
use std::process;

// Read integers from standard input one by one, ignoring whitespace/newlines.
// This struct buffers tokens and allows reading them in the same manner as
// repeated scanf calls in C.
struct InputReader {
    buffer: Vec<String>,
}

impl InputReader {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    // Reads and returns the next token from stdin as a String. If no token
    // can be read (EOF or parse error), returns None.
    fn next_token(&mut self) -> Option<String> {
        // If buffer is empty, read a new line and split it into tokens
        while self.buffer.is_empty() {
            let mut line = String::new();
            let bytes_read = io::stdin().lock().read_line(&mut line).ok()?;
            if bytes_read == 0 {
                // EOF reached
                return None;
            }
            // Split line into whitespace-delimited tokens
            let tokens = line.split_whitespace().map(String::from);
            self.buffer.extend(tokens);
        }
        // Pop the first token
        Some(self.buffer.remove(0))
    }

    // Tries to read a single i32. If it fails, returns None.
    fn next_i32(&mut self) -> Option<i32> {
        self.next_token()?.parse::<i32>().ok()
    }
}

// Equivalent of the C code "int max(int a, int b)".
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// Equivalent of the C code "int min_element(int* arr, int size)".
fn min_element(arr: &[i32]) -> i32 {
    let mut min_val = arr[0];
    for &val in arr.iter().skip(1) {
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

// Equivalent of the C code:
// bool check(int* points, int pointsSize, int m, long long low)
fn check(points: &[i32], points_size: i32, m: i32, low: i64) -> bool {
    let n = points_size as usize;
    let mut rem = m;
    let mut pre = 0;
    for (i, &p) in points.iter().enumerate() {
        // k = (int)((low - 1) / p + 1 - pre);
        // (low - 1) / p is integer division in Rust if p != 0
        let mut k = ((low - 1) / p as i64 + 1 - pre as i64) as i32;
        // if (i == n - 1 && k <= 0) { break; }
        if i == n - 1 && k <= 0 {
            break;
        }
        // k = max(k, 1);
        k = max(k, 1);
        // rem -= k * 2 - 1;
        rem -= k * 2 - 1;
        // if (rem < 0) return false;
        if rem < 0 {
            return false;
        }
        // pre = k - 1;
        pre = k - 1;
    }
    true
}

// Equivalent of the C code:
// long long maxScore(int* points, int pointsSize, int m)
fn max_score(points: &[i32], points_size: i32, m: i32) -> i64 {
    // long long left = 0;
    let mut left: i64 = 0;
    // long long right = ((m + 1) / 2) * min_element(...) + 1;
    let right_limit = ((m + 1) as i64 / 2) * min_element(points) as i64 + 1;
    let mut right: i64 = right_limit;

    // Binary search for the maximum possible score
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(points, points_size, m, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    let mut reader = InputReader::new();

    // Read n, m
    let (n, m) = match (reader.next_i32(), reader.next_i32()) {
        (Some(n_val), Some(m_val)) => (n_val, m_val),
        _ => {
            eprintln!("Error reading input for n and m");
            process::exit(1);
        }
    };

    // Allocate points array (in Rust, just create a Vec)
    let mut points = Vec::with_capacity(n as usize);

    // Read points array
    for i in 0..n {
        let val = match reader.next_i32() {
            Some(v) => v,
            None => {
                eprintln!("Error reading input for points[{}]", i);
                process::exit(1);
            }
        };
        points.push(val);
    }

    // Call the max_score function
    let result = max_score(&points, n, m);

    // Print the result (same format as the C code: printf("%lld\n", result))
    println!("{}", result);
}