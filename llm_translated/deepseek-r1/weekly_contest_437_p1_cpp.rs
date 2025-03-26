use std::io;

fn has_special_substring(s: &str, k: i32) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut cnt = 0;

    for i in 0..n {
        cnt += 1;
        // Check if current position is the end of a group
        if i == n - 1 || (i < n - 1 && bytes[i] != bytes[i + 1]) {
            if cnt == k {
                return true;
            }
            cnt = 0; // Reset counter for new group
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    // Ensure exactly two input components (string and k)
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        std::process::exit(1);
    }

    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid value for k");

    let result = has_special_substring(s, k);
    println!("{}", result as i32); // Convert bool to 1/0 for exact output format
}