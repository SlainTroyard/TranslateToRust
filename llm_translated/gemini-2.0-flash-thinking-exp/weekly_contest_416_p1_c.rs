use std::io;
use std::vec::Vec;

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let mut banned_words_sorted = banned_words.to_vec();
    banned_words_sorted.sort_unstable(); // Sort banned words

    let mut cnt = 0;
    for msg_word in message {
        if banned_words_sorted.binary_search(msg_word).is_ok() {
            cnt += 1;
        }
        if cnt >= 2 {
            return true;
        }
    }
    false
}

fn main() {
    let mut message_size_str = String::new();
    io::stdin().read_line(&mut message_size_str).expect("Failed to read line");
    let message_size: i32 = message_size_str.trim().parse().expect("Invalid input");

    let mut message: Vec<String> = Vec::new();
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        message.push(word.trim().to_string());
    }

    let mut banned_words_size_str = String::new();
    io::stdin().read_line(&mut banned_words_size_str).expect("Failed to read line");
    let banned_words_size: i32 = banned_words_size_str.trim().parse().expect("Invalid input");

    let mut banned_words: Vec<String> = Vec::new();
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        banned_words.push(word.trim().to_string());
    }

    if report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}