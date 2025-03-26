use std::io::{self, Write};

// Function to calculate the absolute value
fn abs_val(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// Function to return the smaller of two values
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Function to return the larger of two values
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Main function to calculate the maximum distance
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
            _ => (),
        }
        
        // Calculate the current maximum distance
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Split the input into the string and the integer k
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid integer for k");
    
    // Call the function to calculate the result
    let result = max_distance(s, k);
    
    // Output the result
    println!("{}", result);
}