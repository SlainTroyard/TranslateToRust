use std::io::{self, BufRead};

/// Calculate absolute value of a number
fn abs_val(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

/// Return the minimum of two numbers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// Return the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Calculate the maximum possible distance based on the given string and k value
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    
    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (), // ignore other characters
        }
        
        // Calculate max distance based on current position, coordinates, and k value
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap()?;
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return Ok(());
    }
    
    let s = parts[0];
    let k = parts[1].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Error parsing k value");
        0
    });
    
    // Calculate and output result
    let result = max_distance(s, k);
    println!("{}", result);
    
    Ok(())
}