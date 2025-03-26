use std::io;

/// Checks if the digits of the input string, after repeatedly combining adjacent digits modulo 10, result in the first two digits being equal.
///
/// # Arguments
/// * `s` - The input string containing digits.
///
/// # Returns
/// * `true` if the final two digits are equal, `false` otherwise.
fn has_same_digits(s: &str) -> bool {
    let mut digits: Vec<u8> = s.chars().map(|c| (c as u8 & 0x0F)).collect();
    let mut current_len = digits.len();
    loop {
        current_len -= 1;
        if current_len <= 1 {
            break;
        }
        for i in 0..current_len {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }
    digits[0] == digits[1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    if s.is_empty() {
        println!("0");
        return;
    }
    let s = s.split_whitespace().next().unwrap();
    let result = has_same_digits(s);
    println!("{}", if result { 1 } else { 0 });
}