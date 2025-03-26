use std::io::{self, Write};

fn string_sequence(target: &str) -> Vec<String> {
    // Calculate the total size of the sequence
    let return_size = target.chars().map(|c| (c as u8 - b'a' + 1) as usize).sum();
    let mut ans = Vec::with_capacity(return_size);
    let mut l = 0;

    // Iterate over each character in the target string
    for t in target.chars() {
        // Generate sequences for each character from 'a' to the current character
        for c in ('a'..=t).into_iter() {
            let mut s = String::with_capacity(l + 1);
            // Copy the prefix from the target string
            s.push_str(&target[..l]);
            // Append the current character
            s.push(c);
            ans.push(s);
        }
        l += 1;
    }

    ans
}

fn main() {
    // Read the input string
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    // Remove the newline character if present
    if target.ends_with('\n') {
        target.pop();
    }

    // Generate the sequence
    let ans = string_sequence(&target);

    // Print the sequence
    for s in &ans {
        print!("{} ", s);
    }
    println!();
}