// Problem: Weekly Contest 435 Problem 2
use std::io::{self, Write};

// Calculate the absolute value of an integer
fn abs_val(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// Return the smaller of two integers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Return the larger of two integers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Main function implementation
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
            _ => {} // Ignore invalid characters (though input is assumed valid)
        }

        // Calculate the maximum distance based on the current position, coordinates, and k
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input into the string `s` and the integer `k`
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().expect("Missing string input");
    let k: i32 = parts
        .next()
        .expect("Missing integer input")
        .parse()
        .expect("Failed to parse integer");

    // Call the function to calculate the result
    let result = max_distance(s, k);

    // Output the result
    println!("{}", result);
}