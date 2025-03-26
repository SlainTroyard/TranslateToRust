use std::io::{self, Write};

/// Generates a sequence of strings based on the given target string.
///
/// # Arguments
///
/// * `target` - A string slice that holds the target string.
///
/// # Returns
///
/// A vector of strings representing the generated sequence.
fn string_sequence(target: &str) -> Vec<String> {
    // Calculate the total number of strings to be generated
    let return_size: usize = target.chars().map(|c| c as usize - 96).sum();

    let mut ans = Vec::with_capacity(return_size);
    let mut l = 0;

    for (i, &c) in target.as_bytes().iter().enumerate() {
        for ch in b'a'..=c {
            let mut new_string = String::with_capacity(i + 1);
            new_string.push_str(&target[..i]);
            new_string.push(ch as char);
            ans.push(new_string);
        }
        l += 1;
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let target = input.trim();

    // Generate the string sequence
    let ans = string_sequence(target);

    // Print the result to stdout
    for s in ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}