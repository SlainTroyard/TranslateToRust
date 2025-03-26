use std::io;

/// Checks if the message contains at least two banned words.
///
/// # Arguments
/// * `messages` - A slice of messages to check.
/// * `banned_words` - A slice of banned words.
///
/// # Returns
/// `true` if there are at least two messages that are banned, `false` otherwise.
pub fn report_spam(messages: &[String], banned_words: &[String]) -> bool {
    let mut sorted_banned = banned_words.to_vec();
    sorted_banned.sort();
    let mut cnt = 0;
    for msg in messages {
        if sorted_banned.binary_search(msg).is_ok() {
            cnt += 1;
            if cnt >= 2 {
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

    let mut tokens = input.split_whitespace();

    // Read message count and messages
    let message_size = tokens.next().expect("No message size")
        .parse::<usize>().expect("Invalid message size");
    let messages: Vec<String> = (0..message_size)
        .map(|_| {
            tokens.next().expect("Not enough messages")
                .to_string()
        })
        .collect();

    // Read banned words count and banned words
    let banned_size = tokens.next().expect("No banned size")
        .parse::<usize>().expect("Invalid banned size");
    let banned_words: Vec<String> = (0..banned_size)
        .map(|_| {
            tokens.next().expect("Not enough banned words")
                .to_string()
        })
        .collect();

    // Determine and print the result
    let result = report_spam(&messages, &banned_words);
    println!("{}", result);
}