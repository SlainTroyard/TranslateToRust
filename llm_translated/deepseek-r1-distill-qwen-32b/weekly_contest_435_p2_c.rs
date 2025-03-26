// Problem: Weekly Contest 435 Problem 2
use std::io;
use std::cmp;

/// Calculate the maximum distance based on the given string and k value.
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
            _ => (), // Ignore other characters as per problem statement
        }
        
        let current = x.abs() + y.abs() + k * 2;
        let current_max = cmp::min(current, (i + 1) as i32);
        ans = cmp::max(ans, current_max);
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Read input line
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Split into parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error: Expected exactly two inputs");
        return Ok(());
    }
    
    let s = parts[0];
    let k = parts[1].parse::<i32>().map_err(|e| {
        eprintln!("Error parsing k: {}", e);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid k value")
    })?;
    
    // Calculate and print result
    let result = max_distance(s, k);
    println!("{}", result);
    
    Ok(())
}