// Problem: Weekly Contest 417 Problem 2
//
// This Rust program preserves the exact logic and I/O format of the original C code.
// It reads three tokens from standard input in the following order:
// 1) An integer representing the size of the word (though we do not strictly enforce this size)
// 2) A string (the word itself)
// 3) An integer k
//
// It then computes the number of substrings starting at each possible position,
// checking if all vowels ('a', 'e', 'i', 'o', 'u') appear at least once and if
// the number of non-vowel characters matches k. The result is printed to stdout.
//
// Usage example (matching the original C program's input/output format):
//   Input:
//     5
//     abcde
//     1
//   Output:
//     (some integer)
//
// No additional output or prompts are printed, maintaining compatibility with
// the original solution's format.

use std::error::Error;
use std::io::{self, BufRead};

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    // Loop through all possible starting points of substrings
    for i in 0..=len.saturating_sub(5) {
        let mut arr = [0; 5];    // Tracks counts of vowels: [a_count, e_count, i_count, o_count, u_count]
        let mut non_vowel_count = 0;

        // Extend the substring from index i to j
        for j in i..len {
            match chars[j] {
                'a' => arr[0] += 1,
                'e' => arr[1] += 1,
                'i' => arr[2] += 1,
                'o' => arr[3] += 1,
                'u' => arr[4] += 1,
                _ => non_vowel_count += 1,
            }

            // Check if all vowels are present at least once and non-vowel count == k
            if arr[0] > 0 && arr[1] > 0 && arr[2] > 0 && arr[3] > 0 && arr[4] > 0 && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    // We collect exactly three tokens from stdin:
    //   1) wordSize (ignored except for reading)
    //   2) the word (string)
    //   3) k (integer)
    let stdin = io::stdin();
    let mut tokens: Vec<String> = Vec::new();

    // Read lines until we get at least 3 tokens, mimicking scanf's whitespace-based token reading.
    for line in stdin.lock().lines() {
        let line = line?;
        for tok in line.split_whitespace() {
            tokens.push(tok.to_string());
            if tokens.len() == 3 {
                break;
            }
        }
        if tokens.len() == 3 {
            break;
        }
    }

    if tokens.len() < 3 {
        return Err("Not enough input tokens".into());
    }

    // Parse the inputs:
    let _word_size: usize = tokens[0].parse()?;  // Read but not strictly enforced in the logic
    let word = &tokens[1];
    let k: i32 = tokens[2].parse()?;

    // Compute the result and print
    let result = count_of_substrings(word, k);
    println!("{}", result);

    Ok(())
}