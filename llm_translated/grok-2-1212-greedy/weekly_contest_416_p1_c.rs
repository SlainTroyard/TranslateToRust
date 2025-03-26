use std::io::{self, BufRead};

fn cmp(a: &&str, b: &&str) -> std::cmp::Ordering {
    a.cmp(b)
}

fn report_spam(message: &[String], banned_words: &mut [String]) -> bool {
    banned_words.sort_unstable_by(cmp);
    let mut cnt = 0;
    for msg in message {
        if banned_words.binary_search_by(|word| cmp(&word, &msg)).is_ok() {
            cnt += 1;
            if cnt >= 2 {
                return true;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read message size
    let message_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read messages
    let mut message = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(lines.next().unwrap()?);
    }

    // Read banned words size
    let banned_words_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read banned words
    let mut banned_words = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(lines.next().unwrap()?);
    }

    // Process and output result
    if report_spam(&message, &mut banned_words) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}