use std::collections::HashSet;
use std::io;

fn main() {
    // Read the size of the message vector
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let message_size: usize = input.trim().parse().expect("Invalid input");

    // Read the message vector
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read input");
        message.push(word.trim().to_string());
    }

    // Read the size of the banned words vector
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let banned_words_size: usize = input.trim().parse().expect("Invalid input");

    // Read the banned words vector
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read input");
        banned_words.push(word.trim().to_string());
    }

    // Check if the message contains more than one banned word
    if report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    // Convert the banned words vector to a HashSet for O(1) lookups
    let banned: HashSet<_> = banned_words.iter().collect();
    let mut cnt = 0;

    // Iterate over the message and count occurrences of banned words
    for word in message {
        if banned.contains(word) {
            cnt += 1;
            if cnt > 1 {
                return true;
            }
        }
    }

    false
}