// Problem: Weekly Contest 417 Problem 2
use std::io;
use std::io::Read;

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let len = word.len();

    // Loop through all possible starting points of substrings
    for i in 0..=(len.saturating_sub(5)) {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            // Check for vowels and increment respective count
            match word.chars().nth(j) {
                Some('a') => arr[0] += 1,
                Some('e') => arr[1] += 1,
                Some('i') => arr[2] += 1,
                Some('o') => arr[3] += 1,
                Some('u') => arr[4] += 1,
                Some(_) => non_vowel_count += 1, // Increment non-vowel count for other characters
                None => {} // Handle case where index is out of bounds, though shouldn't happen given the loops' bounds
            }

            // Check if all vowels are present and non-vowel count equals k
            if arr[0] > 0 && arr[1] > 0 && arr[2] > 0 && arr[3] > 0 && arr[4] > 0 && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let mut word_size = String::new();
    io::stdin().read_line(&mut word_size)?;
    let word_size: usize = word_size.trim().parse().unwrap();

    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    let word = word.trim();

    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    let k: i32 = k.trim().parse().unwrap();

    println!("{}", count_of_substrings(word, k));

    Ok(())
}