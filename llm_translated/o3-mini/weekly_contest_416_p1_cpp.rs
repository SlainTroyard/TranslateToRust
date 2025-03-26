use std::collections::HashSet;
use std::io::{self, Read};

// Define the Solution struct (empty since no fields are needed)
struct Solution;

impl Solution {
    // report_spam checks if more than one word in the message is banned.
    // It takes slices of Strings and returns a bool.
    fn report_spam(message: &[String], banned_words: &[String]) -> bool {
        // Construct a HashSet from banned_words for O(1) lookup.
        let banned: HashSet<_> = banned_words.iter().cloned().collect();
        let mut cnt = 0;
        
        // Iterate through each word in the message.
        for word in message {
            if banned.contains(word) {
                cnt += 1;
                // If more than one banned word is found, return true.
                if cnt > 1 {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input on whitespace to mimic C++'s operator>>
    let mut tokens = input.split_whitespace();
    
    // Read the number of message words.
    let message_size: usize = tokens
        .next()
        .ok_or("Missing message size")?
        .parse()?;
    
    // Read the message words.
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let word = tokens
            .next()
            .ok_or("Missing a message word")?
            .to_string();
        message.push(word);
    }
    
    // Read the number of banned words.
    let banned_words_size: usize = tokens
        .next()
        .ok_or("Missing banned words size")?
        .parse()?;
    
    // Read the banned words.
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let word = tokens
            .next()
            .ok_or("Missing a banned word")?
            .to_string();
        banned_words.push(word);
    }
    
    // Call the report_spam method.
    let is_spam = Solution::report_spam(&message, &banned_words);
    
    // Output the result, ensuring the same format as the original C++ code.
    if is_spam {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}