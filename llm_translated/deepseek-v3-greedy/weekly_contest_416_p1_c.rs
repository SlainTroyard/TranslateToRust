use std::cmp::Ordering;
use std::io::{self, BufRead};

// Comparator function for sorting and searching strings
fn cmp(a: &&str, b: &&str) -> Ordering {
    a.cmp(b)
}

// Function to check if a message contains at least two banned words
fn report_spam(message: &[&str], banned_words: &mut Vec<&str>) -> bool {
    banned_words.sort_by(cmp);
    let mut cnt = 0;
    for word in message {
        if banned_words.binary_search_by(|&w| cmp(&w, word)).is_ok() {
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

    // Read the number of message words
    let message_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = lines.next().unwrap().unwrap();
        message.push(word);
    }

    // Read the number of banned words
    let banned_words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = lines.next().unwrap().unwrap();
        banned_words.push(word);
    }

    // Check if the message is spam
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}