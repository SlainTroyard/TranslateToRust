use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input from stdin into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace to simulate the C scanf behavior.
    // This works regardless of whether multiple values come in one line or separate lines.
    let mut tokens = input.split_whitespace();

    // Read the number of message strings.
    let message_size: usize = tokens
        .next()
        .ok_or("Missing message size")?
        .parse()
        .map_err(|_| "Invalid message size")?;
    
    // Read each message string.
    let mut message: Vec<String> = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let s = tokens
            .next()
            .ok_or("Missing message entry")?
            .to_string();
        message.push(s);
    }
    
    // Read the number of banned words.
    let banned_words_size: usize = tokens
        .next()
        .ok_or("Missing banned words size")?
        .parse()
        .map_err(|_| "Invalid banned words size")?;
    
    // Read each banned word.
    let mut banned_words: Vec<String> = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let s = tokens
            .next()
            .ok_or("Missing banned word")?
            .to_string();
        banned_words.push(s);
    }
    
    // Sort banned_words to prepare for binary search.
    // This is analogous to qsort in the C code.
    banned_words.sort();
    
    // Check how many words in the message are banned.
    // If the count is 2 or more, report spam.
    let mut count = 0;
    for word in &message {
        if banned_words.binary_search(word).is_ok() {
            count += 1;
        }
        if count >= 2 {
            println!("true");
            return Ok(());
        }
    }
    
    println!("false");
    Ok(())
}