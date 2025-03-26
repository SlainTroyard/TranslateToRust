use std::collections::HashSet;
use std::io::{self, BufRead};

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    // Convert banned words into a HashSet for O(1) lookups
    let banned: HashSet<_> = banned_words.iter().collect();
    let mut count = 0;
    
    for word in message {
        if banned.contains(word) {
            count += 1;
            // Check if we've found more than one banned word
            if count > 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    // Read all input tokens at once and create iterator
    let tokens: Vec<String> = io::stdin()
        .lock()
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .collect();
    
    let mut tokens_iter = tokens.into_iter();
    
    // Parse message vector
    let message_size = tokens_iter.next().unwrap().parse::<usize>().unwrap();
    let message: Vec<String> = tokens_iter.by_ref().take(message_size).collect();
    
    // Parse banned words vector
    let banned_size = tokens_iter.next().unwrap().parse::<usize>().unwrap();
    let banned_words: Vec<String> = tokens_iter.by_ref().take(banned_size).collect();
    
    // Check spam condition and print result
    println!("{}", report_spam(&message, &banned_words));
}