use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        // Create a HashSet from banned_words for O(1) lookups
        let banned: HashSet<&String> = banned_words.iter().collect();
        
        // Counter for banned words found
        let mut cnt = 0;
        
        for s in message {
            // If the word is banned, increment count and check if we've found more than 1
            if banned.contains(s) && {
                cnt += 1;
                cnt > 1
            } {
                return true;
            }
        }
        
        false
    }
}

fn main() {
    // Read message size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read message size");
    let message_size: usize = input.trim().parse().expect("Invalid message size");
    
    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read message word");
        message.push(word.trim().to_string());
    }
    
    // Read banned words size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read banned words size");
    let banned_words_size: usize = input.trim().parse().expect("Invalid banned words size");
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read banned word");
        banned_words.push(word.trim().to_string());
    }
    
    // Call the solution and print result
    let s = Solution;
    if s.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}