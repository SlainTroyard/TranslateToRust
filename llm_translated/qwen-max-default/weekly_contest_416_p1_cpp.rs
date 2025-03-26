use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read the number of messages
    let message_size: usize = read_input();
    // Read the messages
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(read_input());
    }
    // Read the number of banned words
    let banned_words_size: usize = read_input();
    // Read the banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(read_input());
    }

    // Create a set of banned words for quick lookup
    let banned_set: HashSet<String> = banned_words.into_iter().collect();

    // Check if the message contains more than one banned word
    let result = report_spam(&message, &banned_set);

    // Print the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}

/// Function to read a single line of input and parse it as a String
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

/// Function to check if the message contains more than one banned word
fn report_spam(message: &[String], banned: &HashSet<String>) -> bool {
    let mut count = 0;
    for word in message {
        if banned.contains(word) && count + 1 > 1 {
            return true;
        }
        if banned.contains(word) {
            count += 1;
        }
    }
    false
}