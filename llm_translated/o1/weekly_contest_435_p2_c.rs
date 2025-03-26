use std::io::{self, Read};

/// Computes the maximum distance as described in the original C code.
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;

    // Iterate over each character in the string along with its index
    for (i, ch) in s.chars().enumerate() {
        // Update coordinates based on the current direction
        match ch {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => {}
        }

        // Convert the enumerate index (usize) to i32
        let i = i as i32;

        // Calculate the maximum distance considering k
        let current_max = (x.abs() + y.abs() + k * 2).min(i + 1);
        ans = ans.max(current_max);
    }

    ans
}

fn main() {
    // Read all input from stdin (mirroring the behavior of scanf which ignores whitespace)
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let mut tokens = input.split_whitespace();

    // Extract s (the direction string)
    let s = match tokens.next() {
        Some(val) => val,
        None => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    // Extract k (the integer)
    let k_str = match tokens.next() {
        Some(val) => val,
        None => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    // Parse k and check for errors
    let k: i32 = match k_str.parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    // Compute the result using the translated function
    let result = max_distance(s, k);

    // Print the result to stdout, matching the original format
    println!("{}", result);
}