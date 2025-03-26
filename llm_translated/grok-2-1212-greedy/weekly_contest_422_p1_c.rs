use std::io::{self, BufRead};

fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        total += '0' as i32 - c as i32;
        total = -total;
    }
    total == 0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number as a string
    if let Some(line) = lines.next() {
        let num = line?;
        
        // Check if the number is balanced
        if is_balanced(&num) {
            println!("true");
        } else {
            println!("false");
        }
    }

    Ok(())
}