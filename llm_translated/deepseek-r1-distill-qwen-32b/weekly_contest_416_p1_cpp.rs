use std::collections::HashSet;
use std::io;

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let banned: HashSet<String> = banned_words.iter().cloned().collect();
    let mut count = 0;
    for word in message {
        if banned.contains(word) {
            count += 1;
            if count > 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let message_size = tokens[0].parse::<usize>().unwrap();
    let message = tokens[1..=message_size]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let banned_words_size = tokens[message_size + 1].parse::<usize>().unwrap();
    let banned_words = tokens[message_size + 2..message_size + 2 + banned_words_size]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let is_spam = report_spam(&message, &banned_words);
    println!("{}", if is_spam { "true" } else { "false" });
}