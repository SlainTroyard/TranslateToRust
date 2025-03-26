use std::io::{self, Read};

// Comparator for binary search is handled by the default string comparison in Rust

fn report_spam(message: &[&str], banned_words: &mut [&str]) -> bool {
    // Sort the banned words for binary search
    banned_words.sort_unstable();
    
    let mut count = 0;
    for word in message {
        // Check if the word is in the banned list using binary search
        if banned_words.binary_search(word).is_ok() {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

fn main() {
    // Read all input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Split into tokens and process sequentially
    let mut tokens = input.split_whitespace();
    
    // Parse message
    let message_size = tokens.next().expect("Missing message size")
        .parse::<usize>().expect("Invalid message size");
    let message: Vec<&str> = tokens.by_ref().take(message_size).collect();
    
    // Parse banned words
    let banned_words_size = tokens.next().expect("Missing banned words size")
        .parse::<usize>().expect("Invalid banned words size");
    let mut banned_words: Vec<&str> = tokens.by_ref().take(banned_words_size).collect();
    
    // Check spam condition and print result
    println!("{}", report_spam(&message, &mut banned_words));
}