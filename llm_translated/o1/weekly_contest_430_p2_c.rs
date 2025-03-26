/// Translated from the C code of LeetCode Weekly Contest 430 Problem 2.
/// 
/// This Rust program preserves the same algorithm logic and input/output format:
///  1) Reads a single "word" token from stdin (equivalent to scanf("%s", word)).
— ///  2) Reads an integer numFriends from stdin (equivalent to scanf("%d", &numFriends)).
///  3) Calls the answer_string function.
///  4) Prints the resulting substring to stdout.
///
/// The answer_string function replicates the logic of:
///   - If numFriends == 1, return the entire original word
///   - Otherwise, find the lexicographically largest substring of length len - numFriends + 1
///     by comparing all possible substrings. Then truncate the word in-place
///     and return the winning substring.
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A queue to store tokens read from stdin (simulating multiple scanf calls).
    let mut input_tokens = VecDeque::new();

    // Read the "word" (equivalent to scanf("%s", ...))
    let mut word = read_token(&mut input_tokens)?;

    // Read the integer (equivalent to scanf("%d", &numFriends))
    let num_friends: usize = read_token(&mut input_tokens)?.parse()?;

    // Call the logic function
    let result = answer_string(&mut word, num_friends);

    // Print the result (equivalent to printf("%s\n", result))
    println!("{}", result);

    Ok(())
}

/// Reads the next token from stdin (whitespace-delimited), simulating multiple calls to scanf.
fn read_token(queue: &mut VecDeque<String>) -> Result<String, Box<dyn std::error::Error>> {
    while queue.is_empty() {
        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            // If we cannot read any more input, it's an error.
            return Err("Insufficient input".into());
        }
        // Split the read line into tokens and push them to the queue.
        for token in line.split_whitespace() {
            queue.push_back(token.to_string());
        }
    }
    // Pop one token from the front of the queue.
    Ok(queue.pop_front().unwrap())
}

/// Translated logic from the `answerString` C function.
/// 
/// Modifies the input string in-place (truncation) and returns
/// a substring slice referencing the winning position.
fn answer_string(word: &mut String, num_friends: usize) -> &str {
    // If numFriends == 1, just return the entire word.
    if num_friends == 1 {
        return &word[..];
    }

    let len = word.len();
    let n = len.saturating_sub(num_friends).saturating_add(1);

    // ans will track the starting index of the best (largest) substring.
    let mut ans = 0_usize;

    // Compare substrings of length n, from every possible start.
    for i in 0..len {
        // In C, strncmp(word + i, word + ans, n) might read up to n chars,
        // but we must ensure we don't go out of bounds in Rust.
        if i + n <= len && ans + n <= len {
            if &word[i..i + n] > &word[ans..ans + n] {
                ans = i;
            }
        }
    }

    // If ans + n is less than the length, truncate the string at ans + n.
    // (equivalent to setting word[ans + n] = '\0' in C.)
    if ans + n < len {
        word.truncate(ans + n);
    }

    // Return the lexicographically largest substring starting at ans.
    &word[ans..]
}