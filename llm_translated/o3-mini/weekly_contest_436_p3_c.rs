use std::io::{self, BufRead, Write};

/// Compute the number of substrings where, when interpreted as a number,
/// the substring is divisible by the digit it ends with.
/// This follows exactly the logic of the C code provided.
fn count_substrings(s: &str) -> i64 {
    // Convert the input string to bytes for digit extraction.
    let bytes = s.as_bytes();
    // The final answer accumulator.
    let mut ans: i64 = 0;
    
    // Create a 2D array f[10][9], initialize with zeros.
    // f[m][r] is used for mod m (m in 1..=9) and remainder r in 0..m.
    // Although the array is defined for 10 x 9, we only use rows 1..9 
    // (and later the row corresponding to a digit d from the input).
    let mut f: [[i64; 9]; 10] = [[0; 9]; 10];

    // Process each character/digit in the input string.
    for &b in bytes {
        // Convert ASCII digit to its numerical value.
        let d = (b - b'0') as usize;

        // For every modulus m from 1 to 9 (matching original C loop: m in [1, 9])
        for m in 1..10 {
            // Temporary array to hold the new counts for modulus m.
            let mut nf = [0i64; 9];

            // Substring with just the single digit d:
            // Its remainder when divided by m is d % m.
            nf[d % m] = 1;

            // For every existing remainder for modulus m,
            // update the new remainder count by appending the new digit.
            for rem in 0..m {
                // Compute new remainder when the digit d is appended.
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem] += f[m][rem];
            }

            // Update the counts for modulus m with the new values.
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // Add to the result the count where the substring (ending at this digit)
        // is divisible by its last digit. In our table, that corresponds to f[d][0].
        ans += f[d][0];
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a standard input locked reader.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    
    // The original C code uses scanf("%s", s), which reads a whitespace-separated token.
    // Therefore, we read a line and then split it to take the first token.
    reader.read_line(&mut buffer)?;
    let token = buffer.split_whitespace().next().ok_or("Error reading input")?;
    
    // Compute the result using our count_substrings function.
    let result = count_substrings(token);
    
    // Print the result exactly matching the original output format.
    println!("{}", result);
    
    Ok(())
}