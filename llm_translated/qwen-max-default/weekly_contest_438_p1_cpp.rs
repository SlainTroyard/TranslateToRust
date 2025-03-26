use std::io::{self, Write};

/// Determines if a string of digits will have the same first two digits after
/// repeatedly summing adjacent digits until the string is length 2 or less.
fn has_same_digits(mut s: String) -> bool {
    while s.len() > 2 {
        let mut t = String::new();
        for i in 0..s.len() - 1 {
            let x = s.chars().nth(i).unwrap().to_digit(10).unwrap();
            let y = s.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            t.push_str(&((x + y) % 10).to_string());
        }
        s = t;
    }
    s.chars().nth(0) == s.chars().nth(1)
}

fn main() {
    // Read the input string from stdin
    let mut s = String::new();
    print!("Enter a string of digits: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string(); // Trim any whitespace and newlines

    // Create an instance of the Solution and call the method
    let result = has_same_digits(s);

    // Output the result to stdout
    println!("{}", if result { "true" } else { "false" });
}