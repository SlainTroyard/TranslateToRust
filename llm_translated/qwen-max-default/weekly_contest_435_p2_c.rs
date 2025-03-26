use std::io::{self, Write};

// Calculate the absolute value
fn abs_val(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// Return the minimum of two numbers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Return the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Main function implementation
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    let length = s.len();
    
    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (),
        }
        
        // Calculate the maximum distance based on the current position, coordinates, and k value
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, i as i32 + 1);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() {
    // Read input
    let mut s = String::new();
    let mut k = String::new();
    
    // Read the string and integer from stdin
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k).expect("Failed to read line");
    
    // Trim newline characters and convert k to an integer
    let s = s.trim();
    let k: i32 = k.trim().parse().expect("Please type a number!");
    
    // Call the function to calculate the result
    let result = max_distance(s, k);
    
    // Output the result
    println!("{}", result);
}