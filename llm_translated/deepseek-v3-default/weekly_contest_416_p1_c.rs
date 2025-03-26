use std::cmp::Ordering;
use std::io::{self, Write};

// Comparator function for sorting and searching strings
fn cmp(a: &&str, b: &&str) -> Ordering {
    a.cmp(b)
}

// Function to check if a message contains at least two banned words
fn report_spam(message: &[&str], banned_words: &mut Vec<&str>) -> bool {
    banned_words.sort_by(cmp);
    let mut cnt = 0;
    for word in message {
        if banned_words.binary_search_by(|&x| cmp(&x, word)).is_ok() {
            cnt += 1;
        }
        if cnt >= 2 {
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    
    // Read the number of messages
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let message_size: usize = input.trim().parse().expect("Invalid number");
    input.clear();
    
    // Read the messages
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        message.push(input.trim());
        input.clear();
    }
    
    // Read the number of banned words
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let banned_words_size: usize = input.trim().parse().expect("Invalid number");
    input.clear();
    
    // Read the banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        banned_words.push(input.trim());
        input.clear();
    }
    
    // Check if the message is spam
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}