use std::collections::HashSet;
use std::io;

/// Checks if a message contains more than one banned word, indicating spam.
///
/// # Arguments
/// * `message` - Vector of words in the message.
/// * `banned_words` - Vector of banned words.
///
/// # Returns
/// `true` if the message contains at least two banned words, `false` otherwise.
fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
    let banned: HashSet<_> = banned_words.into_iter().collect();
    let mut cnt = 0;
    for word in message {
        if banned.contains(&word) {
            cnt += 1;
            // Early return if count exceeds 1
            if cnt > 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut iter = tokens.iter();

    // Parse message size and read message words
    let message_size = iter.next().expect("Missing message size")
        .parse::<usize>().expect("Invalid message size");
    let message: Vec<String> = iter.by_ref()
        .take(message_size)
        .map(|&s| s.to_string())
        .collect();

    // Parse banned words size and read banned words
    let banned_words_size = iter.next().expect("Missing banned words size")
        .parse::<usize>().expect("Invalid banned words size");
    let banned_words: Vec<String> = iter
        .take(banned_words_size)
        .map(|&s| s.to_string())
        .collect();

    // Determine and print the result
    let result = report_spam(message, banned_words);
    println!("{}", if result { "true" } else { "false" });
}