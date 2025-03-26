// LeetCode Weekly Contest 429 Problem 2
// Translated from C++ to Rust with the same algorithmic logic and I/O format.

use std::collections::BTreeSet;
use std::io::{self, BufRead};

// A small helper struct for token-based input reading (like C++'s cin >>).
// This allows us to read integers one by one, ignoring whether they
// are on the same line or multiple lines.
struct TokenReader {
    buf: Vec<String>,
    pos: usize,
}

impl TokenReader {
    fn new() -> Self {
        TokenReader {
            buf: Vec::new(),
            pos: 0,
        }
    }

    // Read the next token as an i32.
    // If we run out of tokens, read a new line from stdin.
    fn read_i32(&mut self) -> io::Result<i32> {
        while self.pos >= self.buf.len() {
            self.buf.clear();
            self.pos = 0;

            let mut line = String::new();
            let bytes_read = io::stdin().read_line(&mut line)?;
            // If there's no more input (bytes_read == 0), we'll stop
            // with an error. The original code does not handle EOF
            // specifically, but we do so here for safety.
            if bytes_read == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more input"));
            }

            // Split the newly read line into tokens
            self.buf = line.split_whitespace().map(String::from).collect();
        }

        let val = self.buf[self.pos].parse::<i32>().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {}", e))
        })?;
        self.pos += 1;

        Ok(val)
    }
}

// Translated solution function:
// Exactly replicates the logic of Solution::maxDistinctElements from the C++ code.
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> usize {
    let mut prev = i32::MIN;
    let mut distinct_values = BTreeSet::new();

    // Sort the array
    arr.sort_unstable();

    // Iterate over each element, place the new distinct value as close to
    // (prev + 1) or (arr[i] - diff), whichever is larger, but not exceeding arr[i] + diff.
    for &val in arr.iter() {
        let x = std::cmp::max(prev + 1, val - diff);
        if x <= val + diff {
            distinct_values.insert(x);
            prev = x;
        }
    }

    distinct_values.len()
}

fn main() -> io::Result<()> {
    let mut reader = TokenReader::new();

    // Read input: n (array size), diff (difference)
    let n = reader.read_i32()? as usize;
    let diff = reader.read_i32()?;

    // Read the array elements
    let mut arr = vec![0; n];
    for i in 0..n {
        arr[i] = reader.read_i32()?;
    }

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result (same format as the C++ version)
    println!("{}", result);

    Ok(())
}