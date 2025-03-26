use std::collections::HashSet;
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn report_spam(&self, message: &Vec<String>, banned_words: &Vec<String>) -> bool {
        let banned: HashSet<String> = banned_words.iter().cloned().collect();
        let mut cnt = 0;
        for s in message {
            if banned.contains(s) {
                cnt += 1;
                if cnt > 1 {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    // Parse message
    let message_size: usize = lines.next().unwrap().parse().unwrap();
    let mut message: Vec<String> = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(lines.next().unwrap().to_string());
    }

    // Parse banned words
    let banned_words_size: usize = lines.next().unwrap().parse().unwrap();
    let mut banned_words: Vec<String> = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(lines.next().unwrap().to_string());
    }

    let s = Solution {};
    if s.report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }
}