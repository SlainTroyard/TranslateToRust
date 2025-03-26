use std::io;

fn has_same_digits(s: &str) -> bool {
    // Convert input string to a vector of digits for efficient processing
    let mut current: Vec<u8> = s.chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
        .collect();
    
    // Process the digits until length is 2 or less
    while current.len() > 2 {
        let mut next = Vec::with_capacity(current.len() - 1);
        for i in 0..current.len() - 1 {
            let sum = (current[i] + current[i + 1]) % 10;
            next.push(sum);
        }
        current = next;
    }
    
    // Check if the first two digits are equal (panics if length < 2 as per original logic)
    current[0] == current[1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();  // Trim whitespace to match C++ cin behavior
    
    let result = has_same_digits(s);
    // Output 1/0 with newline to match C++ cout << bool << endl
    println!("{}", if result { 1 } else { 0 });
}