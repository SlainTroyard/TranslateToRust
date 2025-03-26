use std::io::{self, BufRead, Write};

fn count_of_substrings(word: &str, k: usize) -> usize {
    let mut count = 0;
    let len = word.len();

    // Loop through all possible starting points of substrings
    for i in 0..=len - 5 {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            match word.chars().nth(j).unwrap() {
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
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the word
    stdin.lock().read_line(&mut buffer).expect("Failed to read input");
    let word_size: usize = buffer.trim().parse().expect("Invalid input");
    buffer.clear();

    // Read the word
    stdin.lock().read_line(&mut buffer).expect("Failed to read input");
    let word = buffer.trim();
    buffer.clear();

    // Read the value of k
    stdin.lock().read_line(&mut buffer).expect("Failed to read input");
    let k: usize = buffer.trim().parse().expect("Invalid input");

    // Calculate the result
    let result = count_of_substrings(word, k);

    // Print the result
    writeln!(stdout, "{}", result).expect("Failed to write output");
}