use std::io::{self, BufRead};
use std::cmp::Ordering;

// Function to compare two strings for sorting
fn cmp(a: &&str, b: &&str) -> Ordering {
    a.cmp(b)
}

// Function to check if a message contains at least 2 banned words
fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    // Sort the banned words for binary search
    let mut sorted_banned_words = banned_words.to_vec();
    sorted_banned_words.sort_by(cmp);

    let mut cnt = 0;

    for word in message {
        // Perform binary search to check if the word is in the banned words list
        if sorted_banned_words.binary_search_by(|banned| banned.as_str().cmp(word)).is_ok() {
            cnt += 1;
        }
        if cnt >= 2 {
            return true;
        }
    }

    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the message
    let message_size: usize = lines
        .next()
        .expect("Expected input for message size")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid message size");

    // Read the message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = lines
            .next()
            .expect("Expected input for message word")
            .unwrap()
            .trim()
            .to_string();
        message.push(word);
    }

    // Read the size of the banned words list
    let banned_words_size: usize = lines
        .next()
        .expect("Expected input for banned words size")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid banned words size");

    // Read the banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = lines
            .next()
            .expect("Expected input for banned word")
            .unwrap()
            .trim()
            .to_string();
        banned_words.push(word);
    }

    // Check if the message is spam
    if report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}