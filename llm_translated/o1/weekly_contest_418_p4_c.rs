// LeetCode Weekly Contest 418 Problem 4 in Rust
//
// REQUIREMENTS Fulfilled:
// 1) Translated as a complete Rust program, including main() and I/O handling.
// 2) Algorithm logic matches the C code exactly.
// 3) Written in idiomatic Rust with basic error handling.
// 4) Input/output format is the same as in the original C code.
// 5) Includes helpful comments.

use std::io::{self, BufRead};

////////////////////////////////////////////////////////////////////////////////
// A helper Scanner struct to read tokens (numbers) from stdin in a flexible way.
////////////////////////////////////////////////////////////////////////////////
struct Scanner<B> {
    reader: B,
    buf: Vec<String>,
}

impl<B: BufRead> Scanner<B> {
    fn new(reader: B) -> Self {
        Scanner {
            reader,
            buf: Vec::new(),
        }
    }

    // Reads the next i32 from stdin.
    fn next_i32(&mut self) -> io::Result<i32> {
        loop {
            if let Some(token) = self.buf.pop() {
                return Ok(token
                    .parse()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid i32"))?);
            }
            let mut line = String::new();
            if self.reader.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"));
            }
            let mut parts = line
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect::<Vec<_>>();
            self.buf.append(&mut parts);
        }
    }

    // Reads the next i64 from stdin.
    fn next_i64(&mut self) -> io::Result<i64> {
        loop {
            if let Some(token) = self.buf.pop() {
                return Ok(token
                    .parse()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid i64"))?);
            }
            let mut line = String::new();
            if self.reader.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"));
            }
            let mut parts = line
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect::<Vec<_>>();
            self.buf.append(&mut parts);
        }
    }

    // Reads the next usize from stdin.
    fn next_usize(&mut self) -> io::Result<usize> {
        loop {
            if let Some(token) = self.buf.pop() {
                return Ok(token
                    .parse()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid usize"))?);
            }
            let mut line = String::new();
            if self.reader.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"));
            }
            let mut parts = line
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect::<Vec<_>>();
            self.buf.append(&mut parts);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Translated gcdValues function in Rust. Logic matches the C version exactly.
////////////////////////////////////////////////////////////////////////////////
fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    // Find the maximum value in nums (equivalent to 'mx' in the C code)
    let mut mx = 0;
    for &num in nums {
        if num > mx {
            mx = num;
        }
    }

    // Count how many times each value appears (cnt_x in the C code)
    let mut cnt_x = vec![0; (mx as usize) + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    // Prepare the cnt_gcd array (long long in C -> i64 in Rust)
    let mut cnt_gcd = vec![0i64; (mx as usize) + 1];

    // Calculate pair counts by gcd values.
    // Matches the C loop: for (int i = mx; i > 0; i--) ...
    for i in (1..=mx as usize).rev() {
        let mut c = 0i64;
        // Summation over multiples of i (j = i, i + i, ...)
        let mut j = i;
        while j <= mx as usize {
            c += cnt_x[j] as i64;
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += c * (c - 1) / 2; // number of pairs among 'c' items
    }

    // Prefix sum accumulation for cnt_gcd
    // Matches: for (int i = 1; i <= mx; i++) cnt_gcd[i] += cnt_gcd[i - 1];
    for i in 1..=mx as usize {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // For each query, perform the binary search in [1..mx]
    // Matches the C code's while loop
    let mut ans = vec![0i32; queries.len()];
    for (idx, &query) in queries.iter().enumerate() {
        let (mut left, mut right) = (1usize, mx as usize);
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans[idx] = left as i32;
    }

    ans
}

////////////////////////////////////////////////////////////////////////////////
// Main function to match the C code's input/output format exactly.
////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    // Read numsSize (number of elements in nums)
    let nums_size = scanner.next_usize()?;
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(scanner.next_i32()?);
    }

    // Read queriesSize
    let queries_size = scanner.next_usize()?;
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        queries.push(scanner.next_i64()?);
    }

    // Compute the result using gcd_values
    let ans = gcd_values(&nums, &queries);

    // Print all results in one line separated by spaces, then a newline
    // Matches the C code's printing logic
    for val in ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}