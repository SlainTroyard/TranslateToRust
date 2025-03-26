use std::io::{self, BufRead};
use std::cmp;

/// Calculate absolute value
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

/// Main function implementation
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    
    for (i, ch) in s.chars().enumerate() {
        match ch {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (), // Ignore other characters
        }
        
        // Calculate maximum distance based on current position, coordinates, and k value
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse input line
    let input_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = input_line.split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k = parts[1].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Error parsing k value");
        std::process::exit(1);
    });
    
    // Call function to calculate result
    let result = max_distance(s, k);
    
    // Output result
    println!("{}", result);
}