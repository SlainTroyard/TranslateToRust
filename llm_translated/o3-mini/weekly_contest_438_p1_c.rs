use std::io::{self, BufRead, Write};

fn has_same_digits(s: &mut [u8]) -> bool {
    // Get the current length of the slice.
    let n = s.len();
    // First phase: for each character (digit), perform bitwise AND with 0x0F.
    // This converts an ASCII digit (e.g. b'0' which is 48) into its numeric value (0).
    for i in 0..n {
        s[i] &= 0x0F;
    }

    // Second phase: repeatedly reduce the array until only two numbers remain.
    // The C code does: while (--len > 1) { for (i = 0; i < len; i++) s[i] = (s[i] + s[i + 1]) % 10; }
    // We mimic the same logic in Rust.
    let mut len = n;
    while {
        len -= 1; // pre-decrement len
        len > 1
    } {
        // For each adjacent pair, replace the current element with their sum modulo 10.
        for i in 0..len {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
    }
    // Compare the final two digits.
    s[0] == s[1]
}

fn main() -> io::Result<()> {
    // Create a locked stdin reader for efficient input handling.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read one line from stdin.
    // This will capture one token if the input is space-separated, similar to scanf("%s", s) in C.
    reader.read_line(&mut input_line)?;

    // Split the input by whitespace and take the first token.
    let token = input_line
        .split_whitespace()
        .next()
        .expect("No input provided");

    // Convert the token into a byte vector.
    // The original C program uses an array of char of fixed size.
    // Here, we use a Vec<u8> to hold the digits.
    let mut s: Vec<u8> = token.bytes().collect();

    // Call the algorithm function.
    let result = has_same_digits(&mut s);

    // Print 1 if true, or 0 if false, following the exact same output format as C.
    // The C code uses printf("%d\n", ...), so we print a digit followed by a newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", if result { 1 } else { 0 })?;

    Ok(())
}