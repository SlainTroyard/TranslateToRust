fn is_balanced(num: &str) -> bool {
    let mut total: i32 = 0;
    for char_digit in num.chars() {
        // Calculate '0' - *num, preserving the C logic
        total += '0' as i32 - (char_digit as i32);
        // Negate total, preserving the C logic
        total = -total;
    }
    // Return true if total is 0, false otherwise
    total == 0
}

fn main() {
    // Initialize an empty mutable string to store the input
    let mut num = String::new();

    // Read a line from standard input and handle potential errors
    if std::io::stdin().read_line(&mut num).is_err() {
        return; // Exit if there's an error reading input
    }

    // Remove leading/trailing whitespace from the input string, including newline
    let num = num.trim();

    // Check if the number is balanced using the is_balanced function
    if is_balanced(num) {
        // Print "true" to standard output if the number is balanced
        println!("true");
    } else {
        // Print "false" to standard output if the number is not balanced
        println!("false");
    }
}