use std::io;

fn string_sequence(target: &str) -> Vec<String> {
    let bytes = target.as_bytes();
    // Calculate the total number of strings to generate by summing (c - 'a' + 1) for each character
    let total_size = bytes.iter().map(|&c| (c - b'a' + 1) as usize).sum();
    let mut ans = Vec::with_capacity(total_size);
    let mut prefix_length = 0;

    for &current_char in bytes {
        // Get the prefix string from the target up to current position (exclusive)
        let prefix = &bytes[0..prefix_length];
        // Generate all characters from 'a' to current character (inclusive)
        for c in b'a'..=current_char {
            // Build the new string by combining prefix and current character
            let mut s = prefix.to_vec();
            s.push(c);
            ans.push(
                String::from_utf8(s)
                    .expect("Invalid UTF-8 sequence generated (should never happen with valid input)"),
            );
        }
        prefix_length += 1;
    }

    ans
}

fn main() {
    // Read input line and extract first whitespace-separated token
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let target = input.split_whitespace().next().unwrap_or("");

    let result = string_sequence(target);

    // Print all generated strings separated by spaces, preserving original format
    for s in &result {
        print!("{} ", s);
    }
    println!(); // Final newline after all output
}