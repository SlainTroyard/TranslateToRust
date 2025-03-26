use std::io::{self, BufRead};

fn has_special_substring(s: &str, k: i32) -> bool {
    let length = s.len();
    let mut consecutive = 0;
    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || s.chars().nth(i).unwrap() != s.chars().nth(i + 1).unwrap() {
            if consecutive == k as usize {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    if let Some(line) = lines.next() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s = parts[0];
            let k: i32 = parts[1].parse().unwrap_or(0);

            // Process and output result
            let result = if has_special_substring(s, k) { 1 } else { 0 };
            println!("{}", result);
        }
    }

    Ok(())
}