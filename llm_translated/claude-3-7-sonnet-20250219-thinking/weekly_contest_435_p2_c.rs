use std::io;
use std::cmp;

/// Calculate the maximum distance based on a path string and k value
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    
    // Iterate through each character in the path
    for (i, ch) in s.chars().enumerate() {
        match ch {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (), // Ignore any other characters
        }
        
        // Calculate maximum distance based on current position, coordinates, and k
        let current_max = cmp::min(x.abs() + y.abs() + k * 2, (i + 1) as i32);
        ans = cmp::max(ans, current_max);
    }
    
    ans
}

fn main() {
    // Read input: string s and integer k
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error reading input");
            return;
        }
    }
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k = match parts[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error reading input");
            return;
        }
    };
    
    // Calculate and output the result
    let result = max_distance(s, k);
    println!("{}", result);
}