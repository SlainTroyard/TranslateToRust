use std::io;

/// Finds the lexicographically largest substring of length `n` where `n = len(word) - num_friends + 1`.
///
/// # Arguments
///
/// * `word` - The input string slice.
/// * `num_friends` - The number of friends, determining the substring length.
///
/// # Returns
///
/// A substring of the input string as per the problem's requirements.
fn answer_string(word: &str, num_friends: usize) -> &str {
    if num_friends == 1 {
        return word;
    }

    let bytes = word.as_bytes();
    let len = bytes.len();
    let n = len - num_friends + 1;

    if n == 0 {
        panic!("Invalid input: num_friends exceeds the word length");
    }

    let mut ans = 0;
    for i in 0..=(len - n) {
        let a = &bytes[i..i + n];
        let b = &bytes[ans..ans + n];
        if a > b {
            ans = i;
        }
    }

    &word[ans..ans + n]
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return;
    }

    let word = parts[0];
    let num_friends: usize = parts[1]
        .parse()
        .expect("Second input must be a valid integer");

    let result = answer_string(word, num_friends);
    println!("{}", result);
}