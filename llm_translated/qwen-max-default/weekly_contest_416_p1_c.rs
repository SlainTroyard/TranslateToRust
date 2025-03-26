use std::cmp::Ordering;
use std::io::{self, BufRead};

// Function to compare strings for sorting
fn cmp(a: &String, b: &String) -> Ordering {
    a.cmp(b)
}

// Function to check if the message contains at least two banned words
fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let mut banned_words = banned_words.to_vec();
    banned_words.sort_by(|a, b| cmp(a, b));

    let mut count = 0;
    for msg in message {
        if banned_words.binary_search(&msg).is_ok() {
            count += 1;
        }
        if count >= 2 {
            return true;
        }
    }
    false
}

fn main() {
    // Read the number of messages
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let message_size: usize = input.trim().parse().expect("Please enter a valid number");

    // Read the messages
    let mut messages = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        messages.push(input.trim().to_string());
    }

    // Read the number of banned words
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let banned_words_size: usize = input.trim().parse().expect("Please enter a valid number");

    // Read the banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        banned_words.push(input.trim().to_string());
    }

    // Check for spam and print the result
    if report_spam(&messages, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}