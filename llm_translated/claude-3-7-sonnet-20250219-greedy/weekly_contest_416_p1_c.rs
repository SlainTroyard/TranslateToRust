use std::cmp::Ordering;
use std::io::{self, BufRead};

/// Determines if a message should be reported as spam based on banned words.
/// 
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
    let message_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = lines.next().unwrap().unwrap();
        message.push(word);
    }
    
    // Read banned words size
    let banned_words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = lines.next().unwrap().unwrap();
        banned_words.push(word);
    }
    
    // Check if the message should be reported as spam
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}