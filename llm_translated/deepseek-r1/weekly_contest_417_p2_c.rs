use std::io::{self, Read};

fn count_of_substrings(word: &str, k: i32) -> i32 {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    if len < 5 {
        return 0;
    }

    let mut count = 0;

    // Iterate through all possible starting indices where a substring of at least 5 characters can exist
    for i in 0..=(len - 5) {
        let mut arr = [0; 5]; // Tracks counts for a, e, i, o, u
        let mut non_vowel_count = 0;

        // Check all substrings starting at index i
        for j in i..len {
            match chars[j] {
                'a' => arr[0] += 1,
                'e' => arr[1] += 1,
                'i' => arr[2] += 1,
                'o' => arr[3] += 1,
                'u' => arr[4] += 1,
                _ => non_vowel_count += 1,
            }

            // Check if all vowels are present and non-vowel count matches k
            if arr.iter().all(|&x| x > 0) && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse inputs in the same order as original C code
    let _word_size = tokens.next().unwrap().parse::<usize>().expect("Invalid word size");
    let word = tokens.next().expect("Missing word");
    let k = tokens.next().unwrap().parse::<i32>().expect("Invalid k value");

    println!("{}", count_of_substrings(word, k));
}