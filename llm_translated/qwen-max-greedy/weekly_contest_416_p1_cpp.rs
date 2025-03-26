use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read the size of the message and the messages from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let message_size: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse message size");
    let mut message: Vec<String> = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(lines.next().unwrap().unwrap());
    }

    // Read the size of the banned words and the banned words from stdin
    let banned_words_size: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse banned words size");
    let mut banned_words: Vec<String> = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(lines.next().unwrap().unwrap());
    }

    // Create a Solution instance and call report_spam
    let s = Solution;
    if s.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}

struct Solution;

impl Solution {
    /// Checks if the message contains more than one banned word.
    pub fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        // Convert the list of banned words into a HashSet for O(1) lookups
        let banned: HashSet<&str> = banned_words.iter().map(|s| s.as_str()).collect();
        let mut count = 0;

        // Iterate over each word in the message
        for word in message {
            if banned.contains(word.as_str()) {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }

        false
    }
}