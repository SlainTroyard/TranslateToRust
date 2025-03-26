use std::io::{self, Write};

/// Function to check if the number is balanced
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for ch in num.chars() {
        total += '0' as i32 - ch as i32;
        total = -total;
    }
    total == 0
}

fn main() {
    // Create a buffer to read input
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the number as a string
    stdin.read_line(&mut input).expect("Failed to read input");
    let num = input.trim(); // Remove any trailing newline or whitespace

    // Check if the number is balanced
    if is_balanced(num) {
        writeln!(stdout_lock, "true").expect("Failed to write output");
    } else {
        writeln!(stdout_lock, "false").expect("Failed to write output");
    }
}