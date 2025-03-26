use std::io;

pub fn count_of_substrings(word: &str, k: i32) -> i32 {
    let bytes = word.as_bytes();
    let len = bytes.len();
    let mut count = 0;

    // Iterate over all possible starting indices for substrings
    for i in 0..=len - 5 {
        let mut vowels = [0; 5]; // Counts for 'a', 'e', 'i', 'o', 'u'
        let mut non_vowel_count = 0;

        // Extend the substring ending at j and track counts
        for j in i..len {
            let c = bytes[j];
            match c {
                b'a' => vowels[0] += 1,
                b'e' => vowels[1] += 1,
                b'i' => vowels[2] += 1,
                b'o' => vowels[3] += 1,
                b'u' => vowels[4] += 1,
                _ => non_vowel_count += 1,
            }

            // Check if all vowels are present and non-vowel count matches k
            if vowels.iter().all(|&x| x > 0) && non_vowel_count == k as usize {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    // Parse word size (unused, but required for input compatibility)
    let _word_size: usize = tokens.next().unwrap().parse().expect("Invalid word size");
    let word = tokens.next().unwrap().to_string();
    let k: i32 = tokens.next().unwrap().parse().expect("Invalid k");

    println!("{}", count_of_substrings(&word, k));
}