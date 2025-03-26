use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        // Create a HashSet from banned words for O(1) lookups
        let banned: HashSet<&String> = banned_words.iter().collect();
        
        // Count occurrences of banned words
        let mut count = 0;
        for word in message {
            if banned.contains(word) {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }
        
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read message size
    let message_size: usize = lines.next()
        .expect("Failed to read message size")
        .expect("Failed to parse message size")
        .trim()
        .parse()
        .expect("Invalid message size");
    
    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = lines.next()
            .expect("Failed to read message word")
            .expect("Failed to parse message word");
        message.push(word);
    }
    
    // Read banned words size
    let banned_words_size: usize = lines.next()
        .expect("Failed to read banned words size")
        .expect("Failed to parse banned words size")
        .trim()
        .parse()
        .expect("Invalid banned words size");
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = lines.next()
            .expect("Failed to read banned word")
            .expect("Failed to parse banned word");
        banned_words.push(word);
    }
    
    // Check if message should be reported as spam
    let solution = Solution;
    if Solution::report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}