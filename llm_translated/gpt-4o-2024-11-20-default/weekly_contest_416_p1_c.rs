use std::io::{self, BufRead};
use std::cmp::Ordering;

/// Compare two string slices for sorting, mimicking C-style strcmp behavior.
fn cmp(a: &&str, b: &&str) -> Ordering {
    a.cmp(b)
}

/// Perform binary search on a sorted list with the given key.
fn binary_search(key: &str, sorted_list: &[String]) -> bool {
    sorted_list.binary_search_by(|item| item.as_str().cmp(key)).is_ok()
}

/// Function to determine if the message contains at least 2 banned words.
fn report_spam(message: &[String], banned_words: &mut [String]) -> bool {
    // Sort banned_words, in-place, to ensure it's ready for binary search
    banned_words.sort_unstable_by(|a, b| cmp(&a, &b));

    let mut count = 0;

    // Search for banned words in the message using binary search
    for word in message {
        if binary_search(word, banned_words) {
            count += 1;
        }
        if count >= 2 {
            return true;
        }
    }

    false
}

fn main() {
    // Read all input from stdin
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines().map(|line| line.unwrap());

    // Parse `message_size`
    let message_size: usize = input_lines.next().unwrap().parse().unwrap();
    let mut message: Vec<String> = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(input_lines.next().unwrap());
    }

    // Parse `banned_words_size`
    let banned_words_size: usize = input_lines.next().unwrap().parse().unwrap();
    let mut banned_words: Vec<String> = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(input_lines.next().unwrap());
    }

    // Check if message is spam and print result
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}