use std::io;

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    // Create a sorted copy of banned words for binary search
    let mut sorted_banned = banned_words.to_vec();
    sorted_banned.sort();

    let mut count = 0;
    for word in message {
        // Use binary search to check if the word is in the banned list
        if sorted_banned.binary_search(word).is_ok() {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read message size
    let mut message_size = String::new();
    io::stdin().read_line(&mut message_size)?;
    let message_size: usize = message_size.trim().parse()?;

    // Read message words
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word)?;
        message.push(word.trim().to_string());
    }

    // Read banned words size
    let mut banned_words_size = String::new();
    io::stdin().read_line(&mut banned_words_size)?;
    let banned_words_size: usize = banned_words_size.trim().parse()?;

    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        let mut word = String::new();
        io::stdin().read_line(&mut word)?;
        banned_words.push(word.trim().to_string());
    }

    // Check for spam and output result
    let result = report_spam(&message, &banned_words);
    println!("{}", result);

    Ok(())
}