use std::collections::HashSet;
use std::io::{self, Read};

/// Solution struct that contains the spam detection logic
struct Solution;

impl Solution {
    /// Checks if a message contains more than one banned word
    ///
    /// # Arguments
    /// * `message` - A slice of strings representing the message
    /// * `banned_words` - A slice of strings representing the banned words
    ///
    /// # Returns
    /// * `true` if the message contains more than one banned word
    /// * `false` otherwise
    fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        // Create a HashSet from banned_words for O(1) lookups
        let banned: HashSet<&String> = banned_words.iter().collect();
        
        // Counter for banned words found in the message
        let mut count = 0;
        
        // Check each word in the message
        for word in message {
            if banned.contains(word) && {
                count += 1;
                count > 1
            } {
                return true;
            }
        }
        
        false
    }
}

fn main() {
    // Read the entire input as a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    
    // Parse message size
    let message_size: usize = iter.next()
        .expect("Missing message size")
        .parse()
        .expect("Failed to parse message size");
    
    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(iter.next()
            .expect("Not enough message words")
            .to_string());
    }
    
    // Parse banned words size
    let banned_words_size: usize = iter.next()
        .expect("Missing banned words size")
        .parse()
        .expect("Failed to parse banned words size");
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(iter.next()
            .expect("Not enough banned words")
            .to_string());
    }
    
    // Check if message should be reported as spam
    let solution = Solution;
    if solution.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}