use std::error::Error;
use std::io::{self, Read};

/// Helper function to update the diff array and adjust cnt accordingly.
///
/// # Arguments
/// * `diff` - Mutable reference to the frequency difference array.
/// * `c` - The index corresponding to the character (0 for 'a', 1 for 'b', etc.).
/// * `add` - The value to add to diff[c] (+1 or -1).
/// * `cnt` - Mutable reference to the counter that tracks how many diff[c] are negative.
fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    // When adding a character:
    // If add is 1 and diff[c] becomes 0, that character's deficit is resolved.
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    // When removing a character:
    // If add is -1 and diff[c] becomes -1, that character now has a deficit.
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

/// Counts the valid substrings of word1 that can be transformed using modifications to match word2.
/// The algorithm uses a sliding window over word1 and a frequency difference array (diff).
///
/// # Arguments
/// * `word1` - The first word as a string slice.
/// * `word2` - The second word as a string slice.
///
/// # Returns
/// A 64-bit integer representing the number of valid substrings.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Initialize the frequency difference array for 26 letters to 0.
    let mut diff = [0; 26];

    // For each character c in word2, decrease diff at the corresponding index.
    // This effectively represents the "required" frequency to be compensated by word1.
    for b in word2.as_bytes() {
        diff[(b - b'a') as usize] -= 1;
    }

    // cnt: Count of characters for which diff is negative (i.e., deficit exists).
    let mut cnt = 0;
    for i in 0..26 {
        if diff[i] < 0 {
            cnt += 1;
        }
    }

    let mut res = 0i64;
    let mut l = 0usize;
    let mut r = 0usize;
    // Convert word1 to bytes for constant time indexing.
    let word1_bytes = word1.as_bytes();
    let len1 = word1_bytes.len();

    // Sliding window: l is the left index and r is the right index (exclusive).
    while l < len1 {
        // Expand the window until we have no deficit (cnt == 0) or we reach the end.
        while r < len1 && cnt > 0 {
            // Update diff with the new character from word1 at r.
            update(&mut diff, (word1_bytes[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        // If the current window [l, r) covers all character deficits, count valid substrings.
        if cnt == 0 {
            // Every extension of the current window to the right is valid.
            res += (len1 - r + 1) as i64;
        }
        // Remove the character at position l from the window.
        update(&mut diff, (word1_bytes[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace.
    let mut tokens = input.split_whitespace();

    // The input format is:
    // First integer: length of word1 (not used directly in our logic, but read for compatibility)
    // Then word1 string.
    // Then integer: length of word2 (not used directly)
    // Then word2 string.
    let _len1: usize = tokens
        .next()
        .ok_or("Missing len1 input")?
        .parse()
        .map_err(|_| "Unable to parse len1 as integer")?;
    let word1 = tokens.next().ok_or("Missing word1 input")?;
    let _len2: usize = tokens
        .next()
        .ok_or("Missing len2 input")?
        .parse()
        .map_err(|_| "Unable to parse len2 as integer")?;
    let word2 = tokens.next().ok_or("Missing word2 input")?;

    // Compute the valid substring count.
    let res = valid_substring_count(word1, word2);

    // Print the result with the exact same format.
    println!("{}", res);
    Ok(())
}