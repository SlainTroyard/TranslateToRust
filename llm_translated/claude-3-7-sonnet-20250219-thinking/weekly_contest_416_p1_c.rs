use std::io::{self, Read};

/// Checks if at least 2 messages are in the banned words list
fn report_spam(message: &[String], banned_words: &mut [String]) -> bool {
    // Sort banned words for binary search
    banned_words.sort();
    
    let mut count = 0;
    for msg in message {
        // Check if the message is in the banned words list
        if banned_words.binary_search(msg).is_ok() {
            count += 1;
        }
        if count >= 2 {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split input by whitespace to mimic scanf behavior
    let mut words = input.split_whitespace();
    
    // Read number of messages
    let message_size: usize = words.next()
                                   .expect("Missing message size")
                                   .parse()
                                   .expect("Invalid message size");
    
    // Read messages
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(words.next()
                         .expect("Not enough messages")
                         .to_string());
    }
    
    // Read number of banned words
    let banned_words_size: usize = words.next()
                                       .expect("Missing banned words size")
                                       .parse()
                                       .expect("Invalid banned words size");
    
    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(words.next()
                              .expect("Not enough banned words")
                              .to_string());
    }
    
    // Check if should report spam
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}