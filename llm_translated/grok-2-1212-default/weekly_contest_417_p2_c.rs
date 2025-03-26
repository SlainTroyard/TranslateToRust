use std::io::{self, BufRead};

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let len = word.len();

    // Loop through all possible starting points of substrings
    for i in 0..=len - 5 {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            // Check for vowels and increment respective count
            match word.as_bytes()[j] {
                b'a' => arr[0] += 1,
                b'e' => arr[1] += 1,
                b'i' => arr[2] += 1,
                b'o' => arr[3] += 1,
                b'u' => arr[4] += 1,
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read word size
    let word_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read word
    let word = lines.next().unwrap()?.trim().to_string();

    // Read k
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate and print result
    let result = count_of_substrings(&word, k);
    println!("{}", result);

    Ok(())
}