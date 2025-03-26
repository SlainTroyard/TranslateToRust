use std::io;

fn has_same_digits(s: String) -> bool {
    let mut current = s.into_bytes();
    while current.len() > 2 {
        let mut next = Vec::with_capacity(current.len() - 1);
        for window in current.windows(2) {
            let x = (window[0] - b'0') as u32;
            let y = (window[1] - b'0') as u32;
            let sum = (x + y) % 10;
            next.push((sum as u8) + b'0');
        }
        current = next;
    }
    current[0] == current[1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    let result = has_same_digits(trimmed.to_string());
    println!("{}", if result { 1 } else { 0 });
}