// Problem: Weekly Contest 416 Problem 1
use std::collections::HashSet;
use std::io::{self, BufRead};

// Solution struct equivalent to hold the problem's logic
struct Solution;

impl Solution {
    // Function to determine if the message should be reported as spam
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        // Create a HashSet from the banned words for quick lookup
        let banned: HashSet<String> = banned_words.into_iter().collect();
        let mut count = 0;

        // Iterate through the message, counting banned words
        for word in message {
            if banned.contains(&word) {
                count += 1;
                if count > 1 {
                    return true; // Return true immediately if more than one banned word found
                }
            }
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the message
    let message_size: usize = lines
        .next()
        .expect("Expected message size line")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse message size");

    // Initialize the message vector
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let line = lines.next().expect("Expected a message word").unwrap();
        message.push(line.trim().to_string());
    }

    // Read the size of the banned words list
    let banned_words_size: usize = lines
        .next()
        .expect("Expected banned words size line")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse banned words size");

    // Initialize the banned words vector
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let line = lines.next().expect("Expected a banned word").unwrap();
        banned_words.push(line.trim().to_string());
    }

    // Create a Solution instance and invoke the report_spam function
    let result = Solution::report_spam(message, banned_words);

    // Output the result as "true" or "false" to match the original code's output format
    if result {
        println!("true");
    } else {
        println!("false");
    }
}