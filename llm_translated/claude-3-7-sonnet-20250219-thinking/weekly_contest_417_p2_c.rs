use std::io::{self, Read};

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let mut count = 0;
    let len = word.len();
    let word_bytes = word.as_bytes();

    // Loop through all possible starting points of substrings
    for i in 0..=len.saturating_sub(5) {
        let mut arr = [0; 5]; // Reset vowel counts for each starting point
        let mut non_vowel_count = 0;

        // Loop through the substrings starting at i
        for j in i..len {
            // Check for vowels and increment respective count
            match word_bytes[j] {
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();
    
    // Read word size
    let word_size: usize = lines.next().unwrap().parse()
        .expect("Failed to parse word size");
    
    // Read the word
    let word = lines.next().unwrap();
    
    // Read k
    let k: i32 = lines.next().unwrap().parse()
        .expect("Failed to parse k");
    
    // Calculate and output the result
    println!("{}", count_of_substrings(word, k));
}