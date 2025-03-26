use std::io::{self, BufRead, Write};

/// Constant bitmask representing vowels (in positions corresponding to a=0, e=4, i=8, o=14, u=20)
/// 1065233 in binary has bits set for these vowel positions.
const VOWEL_MASK: i32 = 1065233;

/// Compute the helper function f(word, k).
///
/// It traverses the word and maintains a sliding window, counting subarrays
/// that satisfy the following conditions:
/// - The window contains all 5 vowels at least once.
/// - The number of consonants in the window is >= k.
/// 
/// The function returns the total count accumulated for every position.
fn f(word: &[u8], k: i32) -> i64 {
    let mut ans: i64 = 0;
    // Array to count occurrence of vowels.
    // We only need to store counts for letters 'a' to 'u' (indices 0..=20)
    let mut cnt1 = [0; ('u' as u8 - b'a' + 1) as usize];
    let mut size1 = 0;  // count of distinct vowels encountered in the window
    let mut cnt2 = 0;   // count of consonants in the window
    let mut left = 0;   // left pointer for our sliding window

    // Iterate over each character in the word (sliding window extension)
    for &byte in word.iter() {
        // Convert char to an integer index (0-based, where 'a' corresponds to 0)
        let mut b = (byte - b'a') as i32;
        // Check if b corresponds to a vowel by using the bitmask
        if (VOWEL_MASK >> b) & 1 == 1 {
            // If the vowel hasn't been seen in the current window, increment distinct count
            if cnt1[b as usize] == 0 {
                size1 += 1;
            }
            cnt1[b as usize] += 1;
        } else {
            // Not a vowel => consonant
            cnt2 += 1;
        }

        // When the current window is valid (contains all 5 vowels and at least k consonants),
        // shrink the window from the left as much as possible while it remains valid.
        while size1 == 5 && cnt2 >= k {
            let out = (word[left] - b'a') as i32;
            if (VOWEL_MASK >> out) & 1 == 1 {
                // Remove a vowel from the window and update distinct vowel count if needed.
                cnt1[out as usize] -= 1;
                if cnt1[out as usize] == 0 {
                    size1 -= 1;
                }
            } else {
                // Remove a consonant
                cnt2 -= 1;
            }
            left += 1;
        }
        // The number of valid subarrays ending at the current position is equal to the left pointer index.
        ans += left as i64;
    }
    ans
}

/// Compute the count of substrings that satisfy the problem condition:
/// countOfSubstrings(word, k) = f(word, k) - f(word, k + 1)
fn count_of_substrings(word: &str, k: i32) -> i64 {
    // Convert word to bytes for faster indexing and arithmetic.
    let bytes = word.as_bytes();
    f(bytes, k) - f(bytes, k + 1)
}

/// Main entry point. Handles input and output according to the original code's format.
/// The expected input format (from stdin) is:
///   <wordSize>
///   <word>
///   <k>
/// The program outputs a single integer, which is the result of count_of_substrings.
fn main() -> io::Result<()> {
    // Setup input reading from standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read word size (this value is provided but not directly used in computation).
    let word_size = match lines.next() {
        Some(line) => line?.trim().parse::<usize>().unwrap_or(0),
        None => {
            eprintln!("Expected a word size input.");
            return Ok(());
        }
    };

    // Read the word.
    let word = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("Expected a word input.");
            return Ok(());
        }
    };

    // Optionally, we can check that the length of the word matches word_size.
    if word.len() != word_size {
        // Not an error in terms of functionality, but we note this discrepancy.
        // For the problem, we follow the original input logic and ignore the inconsistency.
    }

    // Read k.
    let k = match lines.next() {
        Some(line) => line?.trim().parse::<i32>().unwrap_or(0),
        None => {
            eprintln!("Expected a k value input.");
            return Ok(());
        }
    };

    // Compute the answer using the provided algorithm.
    let ans = count_of_substrings(&word, k);

    // Output the result.
    println!("{}", ans);

    Ok(())
}