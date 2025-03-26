fn has_same_digits(s: &str) -> bool {
    // Convert each character to its digit value using bitmask as in original C code
    let mut digits: Vec<u8> = s.bytes().map(|b| b & 0x0f).collect();
    let mut current_len = digits.len();

    // Process digits by repeatedly combining adjacent elements until length <= 2
    while {
        current_len -= 1;
        current_len > 1
    } {
        // Update each element with (current + next) % 10 in-place
        for i in 0..current_len {
            let next = digits[i + 1];
            digits[i] = (digits[i] + next) % 10;
        }
    }

    // Check if first two digits match, ensuring we have at least two elements
    digits.len() >= 2 && digits[0] == digits[1]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();  // Match scanf's behavior by trimming whitespace

    // Convert boolean result to 1/0 integer output as in original C code
    println!("{}", has_same_digits(input) as i32);
}