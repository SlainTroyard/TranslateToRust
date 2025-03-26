/// Problem: Weekly Contest 426 Problem 2
///
/// This Rust program preserves the exact logic and I/O format of the original C code.
/// It reads:
/// 1) An integer (numsSize) from stdin.
/// 2) Exactly numsSize integers (possibly spread across any number of lines).
/// Then it computes and prints the largest outlier as defined by the original C program.
use std::io::{self, BufRead};

/// Computes the largest outlier according to the exact logic from the original C code.
fn get_largest_outlier(nums: &[i32]) -> i32 {
    let mut total_sum = 0;
    for &num in nums {
        total_sum += num;
    }

    // 'set' array maps values from [-1000..1000] to [0..2000].
    // We store the frequency of each integer in nums by offsetting by 1000.
    let mut freq = vec![0; 2001];
    for &num in nums {
        freq[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        // Check if curr_sum is even:
        if curr_sum & 1 == 0 {
            let half = curr_sum / 2;
            // Threshold is 1 if half == num, else 0.
            let threshold = if half == num { 1 } else { 0 };

            // Check if half is within [-1000..1000].
            if (-1000..=1000).contains(&half) {
                // If the frequency of 'half' is greater than 'threshold', update ans.
                let idx = (half + 1000) as usize;
                if freq[idx] > threshold {
                    if ans < num {
                        ans = num;
                    }
                }
            }
        }
    }
    ans
}

/// A simple token-based scanner to replicate the "scanf" behavior from C.
/// It reads whitespace-separated tokens from stdin, regardless of line boundaries.
struct Scanner<B> {
    reader: B,
    buf: Vec<String>,
}

impl<B: BufRead> Scanner<B> {
    fn new(reader: B) -> Self {
        Self {
            reader,
            buf: Vec::new(),
        }
    }

    /// Returns the next token (whitespace-delimited) from stdin.
    /// If the buffer is empty, it reads another line.
    fn next_token(&mut self) -> Option<String> {
        loop {
            if let Some(token) = self.buf.pop() {
                return Some(token);
            }
            let mut input_line = String::new();
            if self.reader.read_line(&mut input_line).ok()? == 0 {
                // No more input
                return None;
            }
            // Split the line into tokens (in reverse so we can pop from the end)
            let mut line_tokens: Vec<String> = input_line
                .split_whitespace()
                .map(str::to_string)
                .rev()
                .collect();
            self.buf.append(&mut line_tokens);
        }
    }

    /// Reads and parses the next token as a type T.
    /// Returns an error if parsing fails or there is no more input.
    fn next<T: std::str::FromStr>(&mut self) -> io::Result<T> {
        let token = self
            .next_token()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "No more tokens"))?;
        token.parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Failed to parse token as required type")
        })
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    // Read the size of the array (numsSize).
    let nums_size: usize = scanner.next::<usize>()?;

    // Read exactly numsSize integers.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let value = scanner.next::<i32>()?;
        nums.push(value);
    }

    // Calculate the largest outlier.
    let result = get_largest_outlier(&nums);

    // Output the result (same format as the original code).
    println!("{}", result);

    Ok(())
}