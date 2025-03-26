use std::io::{self, BufRead, Write};
use std::num::ParseIntError;

/// Custom error type to simplify error handling.
#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseError(err)
    }
}

/// Computes the kth character given k and the vector of operations.
/// The logic mimics the C++ solution:
/// - We adjust k by subtracting 1.
/// - Then, iterating from the most significant relevant bit to 0,
///   we add operations[i] if the i-th bit of k is 1.
/// - Finally, the resulting character is 'a' plus inc modulo 26.
fn kth_character(k: i64, operations: &[i32]) -> char {
    // Adjust k (note: original k is assumed to be >= 1)
    let k_adj = k - 1;
    let mut inc = 0;
    // Convert k_adj to u64 for bit manipulation.
    let k_adj_u64 = k_adj as u64;

    // Compute the index of the most significant bit that is set.
    // In C++ __lg(k_adj) yields floor(log2(k_adj)).
    // The number of leading zeros of 0 is 64, so if k_adj is 0 then there are no bits set.
    // According to the problem, k is always >= 1 so k_adj should be >= 0.
    let max_bit = if k_adj_u64 == 0 {
        0
    } else {
        63 - k_adj_u64.leading_zeros()
    };

    // Iterate over bits from max_bit downto 0.
    for i in (0..=max_bit).rev() {
        if ((k_adj_u64 >> i) & 1) == 1 {
            // As operations are stored in the same order, we can directly index them.
            // It is assumed that operations has enough elements.
            inc += operations[i as usize];
        }
    }
    // Return the character based on inc modulo 26.
    // 'a' as u8 plus remainder.
    (b'a' + (inc % 26) as u8) as char
}

/// Reads and parses input similar to the original C++ code.
/// Expected input tokens:
/// - First token: k (i64)
/// - Second token: operationSize (usize)
/// - Followed by operationSize integers.
fn parse_input() -> Result<(i64, Vec<i32>), Error> {
    // Read complete input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .join(" ");
    
    // Split input into words and parse tokens.
    let mut tokens = input.split_whitespace();

    // Parse k. If parsing fails, return an error.
    let k: i64 = tokens.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing k"))?.parse()?;
    // Parse the number of operations.
    let operation_size: usize = tokens.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing operation size"))?.parse()?;
    
    // Parse each operation.
    let mut operations = Vec::with_capacity(operation_size);
    for _ in 0..operation_size {
        let op: i32 = tokens.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing operation value"))?.parse()?;
        operations.push(op);
    }

    Ok((k, operations))
}

fn main() -> Result<(), Error> {
    // Using stdout lock for potentially better performance.
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Parse the input.
    let (k, operations) = parse_input()?;
    // Compute the kth character.
    let result = kth_character(k, &operations);
    // Write the result followed by a newline, matching the original C++ output.
    writeln!(out, "{}", result)?;

    Ok(())
}