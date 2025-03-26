use std::collections::HashSet;
use std::io::{self, BufRead};

/// A simple struct to mirror the C++ `Solution` class
struct Solution;

impl Solution {
    /// Checks if a message contains more than one banned word
    fn report_spam(&self, message: &[String], banned_words: &[String]) -> bool {
        // Convert banned_words into a HashSet for efficient lookups
        let banned: HashSet<String> = banned_words.iter().cloned().collect();
        let mut count = 0;

        // Iterate over each word in the message
        for word in message {
            // If the word is banned, increment the count
            if banned.contains(word) {
                count += 1;
                // If we have more than one banned word, report spam
                if count > 1 {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    // Read all lines from stdin and split them by whitespace into tokens
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let tokens: Vec<String> = lines
        .map(|line| line.expect("Failed to read line"))
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
        .collect();

    // We'll keep an index to traverse the tokens vector
    let mut index = 0;

    // Read messageSize
    let message_size: usize = tokens[index].parse().expect("Failed to parse messageSize");
    index += 1;

    // Read the message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(tokens[index].clone());
        index += 1;
    }

    // Read bannedWordsSize
    let banned_words_size: usize = tokens[index].parse().expect("Failed to parse bannedWordsSize");
    index += 1;

    // Read the bannedWords
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(tokens[index].clone());
        index += 1;
    }

    // Create a Solution instance and check if the message is spam
    let s = Solution;
    if s.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}