use std::io::{self, BufRead};
use std::cmp::Ordering;

/// Determines if a message should be reported as spam based on banned words.
/// A message is considered spam if it contains at least 2 banned words.
fn report_spam(message: &[String], banned_words: &mut [String]) -> bool {
    // Sort banned words for binary search
    banned_words.sort();
    
    let mut count = 0;
    for word in message {
        // Binary search to check if the word is banned
        if banned_words.binary_search(word).is_ok() {
            count += 1;
        }
        if count >= 2 {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read message size
    let message_size: usize = lines.next()
        .expect("Failed to read message size line")
        .expect("Failed to read message size")
        .trim()
        .parse()
        .expect("Failed to parse message size");
    
    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = lines.next()
            .expect("Failed to read message word line")
            .expect("Failed to read message word");
        message.push(word);
    }
    
    // Read banned words size
    let banned_words_size: usize = lines.next()
        .expect("Failed to read banned words size line")
        .expect("Failed to read banned words size")
        .trim()
        .parse()
        .expect("Failed to parse banned words size");
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = lines.next()
            .expect("Failed to read banned word line")
            .expect("Failed to read banned word");
        banned_words.push(word);
    }
    
    // Check if message is spam and print result
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}