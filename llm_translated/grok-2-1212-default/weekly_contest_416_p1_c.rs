use std::io::{self, BufRead};
use std::cmp::Ordering;

// Custom comparison function for sorting and searching
fn cmp(a: &&str, b: &&str) -> Ordering {
    a.cmp(b)
}

// Function to check if a message is spam
fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let mut banned_words_sorted = banned_words.to_vec();
    banned_words_sorted.sort_unstable_by(|a, b| cmp(&a, &b));

    let mut count = 0;
    for msg in message {
        if banned_words_sorted.binary_search_by(|word| cmp(&word, &msg)).is_ok() {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read message size
    let message_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read messages
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(lines.next().unwrap()?);
    }

    // Read banned words size
    let banned_words_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(lines.next().unwrap()?);
    }

    // Check for spam and print result
    if report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}