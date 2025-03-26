use std::io;

fn has_same_digits(s: &mut String) -> bool {
    let mut digits: Vec<u8> = s.chars().map(|c| (c as u8) & 0x0f).collect();
    let mut len = digits.len();
    let mut i = 0;
    while i < len {
        digits[i] &= 0x0f;
        i += 1;
    }
    while len > 1 {
        len -= 1;
        for i in 0..len {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }
    digits[0] == digits[1]
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    // Remove trailing newline if present
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    let mut s_clone = s.clone(); // Create a mutable clone to pass to the function
    if has_same_digits(&mut s_clone) {
        println!("1");
    } else {
        println!("0");
    }
}