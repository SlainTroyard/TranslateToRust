use std::io::{self, BufRead};

/// Checks if the given string `s` contains a substring of `k` consecutive identical characters.
fn has_special_substring(s: &str, k: usize) -> bool {
    let mut consecutive = 0;
    for (i, &current_char) in s.as_bytes().iter().enumerate() {
        consecutive += 1;
        if i == s.len() - 1 || current_char != s.as_bytes()[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input in the same format as the original C code
    if let Some(line) = lines.next() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s = parts[0];
            let k: usize = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid k value"))?;

            // Process the input and print the result
            let result = if has_special_substring(s, k) { 1 } else { 0 };
            println!("{}", result);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input format"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No input provided"));
    }

    Ok(())
}