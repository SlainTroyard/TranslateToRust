use std::io::{self, Write};

fn count_of_substrings(word: &str, k: usize) -> usize {
    let mut count = 0;
    let len = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    // Loop through all possible starting points of substrings
    for i in 0..=len - 5 {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            // Check for vowels and increment respective count
            match word_chars[j] {
                'a' => arr[0] += 1,
                'e' => arr[1] += 1,
                'i' => arr[2] += 1,
                'o' => arr[3] += 1,
                'u' => arr[4] += 1,
                _ => non_vowel_count += 1, // Increment non-vowel count for other characters
            }

            // Check if all vowels are present and non-vowel count equals k
            if arr.iter().all(|&x| x > 0) && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Read the size of the word
    stdin.read_line(&mut input).expect("Failed to read input");
    let word_size: usize = input.trim().parse().expect("Invalid input for word size");
    input.clear();

    // Read the word
    stdin.read_line(&mut input).expect("Failed to read input");
    let word = input.trim().to_string();
    input.clear();

    // Read the value of k
    stdin.read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input for k");

    // Ensure the word size matches the input word length
    assert_eq!(word.len(), word_size, "Word size does not match the input word length");

    // Calculate the result
    let result = count_of_substrings(&word, k);

    // Print the result
    writeln!(stdout, "{}", result).expect("Failed to write output");
}