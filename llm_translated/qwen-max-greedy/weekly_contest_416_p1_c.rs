use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut message: Vec<String> = Vec::new();
    let mut banned_words: Vec<String> = Vec::new();

    // Read the size of the message array
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let message_size: usize = input.trim().parse().unwrap();

    // Read the messages
    for _ in 0..message_size {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        message.push(input.trim().to_string());
    }

    // Read the size of the banned words array
    input.clear();
    stdin.lock().read_line(&mut input).unwrap();
    let banned_words_size: usize = input.trim().parse().unwrap();

    // Read the banned words
    for _ in 0..banned_words_size {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        banned_words.push(input.trim().to_string());
    }

    // Sort the banned words
    banned_words.sort_unstable();

    // Check if the message contains at least two banned words
    if report_spam(&message, &banned_words) {
        writeln!(stdout, "true").unwrap();
    } else {
        writeln!(stdout, "false").unwrap();
    }
}

/// Checks if the message contains at least two banned words.
///
/// # Arguments
///
/// * `message` - A slice of strings representing the message.
/// * `banned_words` - A sorted slice of strings representing the banned words.
///
/// # Returns
///
/// * `true` if the message contains at least two banned words, otherwise `false`.
fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let mut count = 0;
    for msg in message {
        if binary_search(banned_words, msg) {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

/// Performs a binary search to check if the word is in the sorted list of banned words.
///
/// # Arguments
///
/// * `banned_words` - A sorted slice of strings representing the banned words.
/// * `word` - The word to search for.
///
/// # Returns
///
/// * `true` if the word is found, otherwise `false`.
fn binary_search(banned_words: &[String], word: &str) -> bool {
    match banned_words.binary_search_by(|probe| probe.as_str().cmp(word)) {
        Ok(_) => true,
        Err(_) => false,
    }
}