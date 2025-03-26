use std::io::{self, Read};

/// Computes the maximum difference between the largest odd frequency count (c1)
/// and the smallest even frequency count (c2) in the given string.
/// 
/// This function follows exactly the logic from the original C code:
/// 1. Count frequency for each lowercase letter.
/// 2. For each letter:
///    - If its count is odd, update c1 if this count is greater.
///    - Else if its count is even and nonzero, update c2 if this count is smaller.
/// 3. Return c1 - c2.
fn max_difference(s: &str) -> i32 {
    // Array for counting occurrences for 26 letters
    let mut count = [0; 26];

    // Count each character (only lowercase letters are considered)
    // This mimics: while (*s) ++cnt[*s++ - 97];
    for byte in s.bytes() {
        // The original C code assumes only proper lowercase letters (ASCII a-z).
        if byte.is_ascii_lowercase() {
            count[(byte - b'a') as usize] += 1;
        }
    }

    // Initialize counters: c1 for max odd count, c2 for min even count.
    let mut c1 = 0;   // maximum frequency among letters with an odd count
    let mut c2 = 100; // minimum frequency among letters with an even (non-zero) count

    // Process counts for each letter
    // This mimics the for-loop in C that updates c1 and c2.
    for &cnt in count.iter() {
        if cnt % 2 == 1 {
            // If frequency is odd, update c1 (maximum)
            if cnt > c1 {
                c1 = cnt;
            }
        } else if cnt != 0 {
            // If frequency is even and nonzero, update c2 (minimum)
            if cnt < c2 {
                c2 = cnt;
            }
        }
    }

    // Return the difference (c1 - c2) as per original logic.
    c1 - c2
}

fn main() -> io::Result<()> {
    // Read the entire input into a string.
    // This preserves the exact input parsing: we read tokens, potentially
    // from multiple lines or values per line, similar to scanf("%s", s) in C.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Get the first whitespace-separated token.
    // The original C code reads one string (word) into s[100].
    let token = input
        .split_whitespace()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No input provided"))?;

    // Call max_difference with the token.
    let result = max_difference(token);

    // Print the result exactly as the original code does using printf("%d\n", result).
    println!("{}", result);

    Ok(())
}