use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        // Create a HashSet from the banned words for efficient lookup
        let banned: HashSet<String> = banned_words.into_iter().collect();
        let mut count = 0;

        // Iterate through the message and check for banned words
        for word in message {
            if banned.contains(&word) {
                count += 1;
                if count > 1 {
                    return true; // Return true if more than one banned word is found
                }
            }
        }
        false // Return false if no more than one banned word is found
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the message
    let message_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse message size");

    // Read the message words
    let mut message = Vec::new();
    for _ in 0..message_size {
        let word = lines.next().unwrap().unwrap();
        message.push(word);
    }

    // Read the size of the banned words
    let banned_words_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse banned words size");

    // Read the banned words
    let mut banned_words = Vec::new();
    for _ in 0..banned_words_size {
        let word = lines.next().unwrap().unwrap();
        banned_words.push(word);
    }

    // Create an instance of the Solution struct and call the report_spam method
    let solution = Solution;
    if solution.report_spam(message, banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}