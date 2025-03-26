use std::collections::HashSet;
use std::io;

fn main() {
    // Read the size of the message vector
    let mut message_size = String::new();
    io::stdin().read_line(&mut message_size).expect("Failed to read line");
    let message_size: usize = message_size.trim().parse().expect("Please type a number!");

    // Read the message vector
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        message.push(word.trim().to_string());
    }

    // Read the size of the banned words vector
    let mut banned_words_size = String::new();
    io::stdin().read_line(&mut banned_words_size).expect("Failed to read line");
    let banned_words_size: usize = banned_words_size.trim().parse().expect("Please type a number!");

    // Read the banned words vector
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        banned_words.push(word.trim().to_string());
    }

    // Create a HashSet from the banned words vector for O(1) lookups
    let banned: HashSet<String> = banned_words.into_iter().collect();

    // Check if the message contains more than one banned word
    let mut cnt = 0;
    for s in &message {
        if banned.contains(s) {
            cnt += 1;
            if cnt > 1 {
                println!("true");
                return;
            }
        }
    }

    // If not, print false
    println!("false");
}