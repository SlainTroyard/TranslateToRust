use std::io;

/// Checks if a number string is balanced according to the given algorithm.
/// 
/// The algorithm works by iterating through each character of the string, converting it to its
/// digit value, and maintaining a running total that is negated after each step. The string is
/// considered balanced if the total is zero after processing all characters.
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        total += ('0' as i32) - (c as i32);
        total = -total;
    }
    total == 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.split_whitespace().next().unwrap_or("");
    let balanced = is_balanced(num);
    println!("{}", balanced);
}