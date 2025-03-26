use std::io::{self, Write};

fn count_of_substrings(word: &str, k: usize) -> usize {
    let mut count = 0;
    let len = word.len();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    // Loop through all possible starting points of substrings
    for i in 0..=len.saturating_sub(5) {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            let ch = word.chars().nth(j).unwrap();
            match ch {
                'a' => arr[0] += 1,
                'e' => arr[1] += 1,
                'i' => arr[2] += 1,
                'o' => arr[3] += 1,
                'u' => arr[4] += 1,
                _ => non_vowel_count += 1, // Increment non-vowel count for other characters
            }

            // Check if all vowels are present and non-vowel count equals k
            if arr.iter().all(|&count| count > 0) && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let word_size: usize = input.trim().parse().expect("Failed to parse word size");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let word = input.trim();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Failed to parse k");

    // Result of the count_of_substrings function
    let result = count_of_substrings(word, k);
    println!("{}", result);
}