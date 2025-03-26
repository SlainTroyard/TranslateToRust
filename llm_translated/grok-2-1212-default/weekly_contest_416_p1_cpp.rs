use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        let banned: HashSet<&str> = banned_words.iter().map(|s| s.as_str()).collect();
        let mut cnt = 0;
        for s in message {
            if banned.contains(s.as_str()) && {
                cnt += 1;
                cnt > 1
            } {
                return true;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read message size
    let message_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read message
    let message: Vec<String> = (0..message_size)
        .map(|_| lines.next().unwrap().unwrap())
        .collect();

    // Read banned words size
    let banned_words_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read banned words
    let banned_words: Vec<String> = (0..banned_words_size)
        .map(|_| lines.next().unwrap().unwrap())
        .collect();

    // Process and output result
    let result = Solution::report_spam(&message, &banned_words);
    println!("{}", if result { "true" } else { "false" });

    Ok(())
}