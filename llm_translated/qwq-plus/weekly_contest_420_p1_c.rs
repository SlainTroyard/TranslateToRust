use std::io;

/// Generates all possible strings by replacing each character in the target with every character from 'a' up to that character.
///
/// # Arguments
///
/// * `target` - The input string to process.
///
/// # Returns
///
/// A vector of strings generated according to the problem's rules.
fn string_sequence(target: &str) -> Vec<String> {
    let total: usize = target.bytes().map(|b| (b - b'a' + 1) as usize).sum();
    let mut ans = Vec::with_capacity(total);

    for l in 0..target.len() {
        let current_char = target.as_bytes()[l] as char;
        let prefix = &target[0..l];
        for c in 'a'..=current_char {
            ans.push(format!("{}{}", prefix, c));
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.trim();

    let result = string_sequence(target);

    // Print each string followed by a space, then a newline at the end as per original code
    for s in &result {
        print!("{} ", s);
    }
    println!();
}