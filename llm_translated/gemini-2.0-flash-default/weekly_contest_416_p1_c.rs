use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn report_spam(message: &[String], banned_words: &[String]) -> bool {
    let mut sorted_banned_words = banned_words.to_vec();
    sorted_banned_words.sort();

    let mut cnt = 0;
    for word in message {
        if sorted_banned_words.binary_search(word).is_ok() {
            cnt += 1;
        }
        if cnt >= 2 {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let message_size: usize = lines.next().unwrap().parse().unwrap();
    let mut message: Vec<String> = Vec::with_capacity(message_size);
    for _ in 0..message_size {
        message.push(lines.next().unwrap().to_string());
    }

    let banned_words_size: usize = lines.next().unwrap().parse().unwrap();
    let mut banned_words: Vec<String> = Vec::with_capacity(banned_words_size);
    for _ in 0..banned_words_size {
        banned_words.push(lines.next().unwrap().to_string());
    }

    if report_spam(&message, &banned_words) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}