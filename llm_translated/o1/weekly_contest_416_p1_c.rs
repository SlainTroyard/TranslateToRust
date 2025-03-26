use std::io::{self, BufRead};

/// Sorts the list of banned words, then checks how many words in the message
/// are in the banned list. If at least two appear, returns true (spam).
fn report_spam(message: &[String], banned_words: &mut [String]) -> bool {
    // Sort the banned words alphabetically (like qsort + strcmp in C)
    banned_words.sort_unstable();
    let mut count = 0;

    // Perform a binary search for each message string in the banned_words array
    for word in message {
        if banned_words.binary_search(word).is_ok() {
            count += 1;
            // If 2 or more banned words are found, report spam immediately
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    // Read all tokens from stdin, splitting by whitespace (mimics multiple scanf("%s") calls)
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(|s| s.to_string()));
    }

    // Use an index to iterate through the tokens
    let mut idx = 0;

    // 1) Read the number of message words
    let message_size = tokens[idx].parse::<usize>().expect("Failed to parse messageSize");
    idx += 1;

    // 2) Read the message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(tokens[idx].clone());
        idx += 1;
    }

    // 3) Read the number of banned words
    let banned_words_size = tokens[idx].parse::<usize>().expect("Failed to parse bannedWordsSize");
    idx += 1;

    // 4) Read the banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(tokens[idx].clone());
        idx += 1;
    }

    // 5) Perform the spam check and print the result (mimics printf("true")/("false"))
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}