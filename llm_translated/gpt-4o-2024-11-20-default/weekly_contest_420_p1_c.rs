use std::io::{self, Write};
use std::string::String;

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;

    // Calculate the total number of strings in the sequence
    for &byte in target.as_bytes() {
        return_size += (byte as usize) - 96; // ASCII difference for 'a' = 96
    }

    let mut ans = Vec::with_capacity(return_size);

    // Generate the string sequence
    for (l, &byte) in target.as_bytes().iter().enumerate() {
        for c in b'a'..=byte {
            let mut new_str = String::from(&target[0..l]);
            new_str.push(c as char);
            ans.push(new_str);
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut target = String::new();
    io::stdin().read_line(&mut target)?;

    // Strip any trailing newline or whitespace
    let target = target.trim();

    // Calculate the string sequence
    let ans = string_sequence(target);

    // Output the results
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for s in &ans {
        write!(handle, "{} ", s)?;
    }
    writeln!(handle)?;

    Ok(())
}