use std::collections::HashSet;
use std::io;

struct Solution {}

impl Solution {
    pub fn report_spam(&self, message: &Vec<String>, banned_words: &Vec<String>) -> bool {
        // Create a HashSet of banned words for efficient lookup
        let banned: HashSet<String> = banned_words.iter().cloned().collect();
        let mut cnt = 0;
        // Iterate through each word in the message
        for s in message.iter() {
            // Check if the word is in the banned set
            if banned.contains(s) {
                cnt += 1;
                // If the count of banned words exceeds 1, return true (spam)
                if cnt > 1 {
                    return true;
                }
            }
        }
        // If the loop finishes without returning true, return false (not spam)
        false
    }
}

fn main() {
    // Read message size from input
    let mut message_size_str = String::new();
    io::stdin().read_line(&mut message_size_str).unwrap();
    let message_size: usize = message_size_str.trim().parse().unwrap();

    // Read message words from input
    let mut message: Vec<String> = Vec::new();
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        message.push(word.trim().to_string());
    }

    // Read banned words size from input
    let mut banned_words_size_str = String::new();
    io::stdin().read_line(&mut banned_words_size_str).unwrap();
    let banned_words_size: usize = banned_words_size_str.trim().parse().unwrap();

    // Read banned words from input
    let mut banned_words: Vec<String> = Vec::new();
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        banned_words.push(word.trim().to_string());
    }

    // Create an instance of the Solution struct
    let s = Solution {};
    // Call the report_spam function and print the result
    if s.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}