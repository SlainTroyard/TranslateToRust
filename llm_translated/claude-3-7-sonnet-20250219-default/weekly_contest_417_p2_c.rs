// Problem: Weekly Contest 417 Problem 2
use std::io;

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let len = word.len();
    let chars: Vec<char> = word.chars().collect();

    // Loop through all possible starting points of substrings
    for i in 0..=(len.saturating_sub(5)) {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            // Check for vowels and increment respective count
            match chars[j] {
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
    // Read word size
    let mut word_size_input = String::new();
    io::stdin().read_line(&mut word_size_input).expect("Failed to read word size");
    let _word_size: usize = word_size_input.trim().parse().expect("Invalid word size");

    // Read the word
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read word");
    let word = word.trim();

    // Read k
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read k");
    let k: i32 = k_input.trim().parse().expect("Invalid k value");

    // Calculate and print result
    println!("{}", count_of_substrings(word, k));
}