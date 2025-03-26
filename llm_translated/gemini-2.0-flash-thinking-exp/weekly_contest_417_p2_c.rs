// Problem: Weekly Contest 417 Problem 2 in Rust

use std::io;
use std::io::BufRead;

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let len = word.len();

    // Loop through all possible starting points of substrings
    for i in 0..=len.saturating_sub(5) {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            let char_at_j = word.chars().nth(j).unwrap(); // Get the character at index j

            // Check for vowels and increment respective count
            match char_at_j {
                'a' => arr[0] += 1,
                'e' => arr[1] += 1,
                'i' => arr[2] += 1,
                'o' => arr[3] += 1,
                'u' => arr[4] += 1,
                _ => non_vowel_count += 1, // Increment non-vowel count for other characters
            }

            // Check if all vowels are present and non-vowel count equals k
            if arr[0] > 0 && arr[1] > 0 && arr[2] > 0 && arr[3] > 0 && arr[4] > 0 && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read wordSize (not actually used in C logic, but read to match input format)
    let word_size = lines.next().unwrap().expect("Failed to read line");
    let _word_size: i32 = word_size.trim().parse().expect("Failed to parse wordSize");

    // Read word
    let word_line = lines.next().unwrap().expect("Failed to read line");
    let word = word_line.trim();

    // Read k
    let k_line = lines.next().unwrap().expect("Failed to read line");
    let k: i32 = k_line.trim().parse().expect("Failed to parse k");

    println!("{}", count_of_substrings(word, k));
}