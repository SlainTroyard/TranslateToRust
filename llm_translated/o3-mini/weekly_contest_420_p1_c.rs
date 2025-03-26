use std::io::{self, BufRead, Write};

/// Given a target string, builds a sequence of strings as described in
/// LeetCode Weekly Contest 420 Problem 1:
/// For each character in the target at position l, for every character c from 'a' up to that character,
/// create a string consisting of the first l characters of target followed by c.
fn string_sequence(target: &str) -> Vec<String> {
    // Create an empty vector to store the resulting strings.
    let mut ans = Vec::new();

    // Iterate over each character in target along with its index.
    // We assume target is ASCII so that byte-indexing is safe.
    for (l, ch) in target.chars().enumerate() {
        // For each character c from 'a' up to the current character in target
        for c in 'a'..=ch {
            // Create a new string:
            // Copy the first l characters (prefix) then append the current candidate letter.
            // For ASCII input, slicing by bytes works fine.
            let mut s = String::with_capacity(l + 1);
            s.push_str(&target[..l]);
            s.push(c);
            ans.push(s);
        }
    }
    ans
}

fn main() -> io::Result<()> {
    // Create a buffered reader from stdin for efficient I/O.
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Read the entire input into a String.
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // The original C code uses scanf("%s", target) which reads a single token.
    // So we split based on whitespace and take the first token.
    let mut tokens = input.split_whitespace();
    let target = match tokens.next() {
        Some(token) => token,
        None => {
            // If no token is provided, simply exit.
            return Ok(());
        }
    };

    // Generate the sequence of strings.
    let ans = string_sequence(target);

    // The original output prints each string followed by a space, then a newline at the end.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for s in ans {
        write!(handle, "{} ", s)?;
    }
    writeln!(handle)?;
    Ok(())
}