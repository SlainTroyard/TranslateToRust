use std::io::{self, Read};

// Calculate absolute value
fn abs_val(x: i32) -> i32 {
    x.abs()
}

// Return the smaller of two numbers
fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

// Return the larger of two numbers
fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

// Main function implementation
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    
    for c in s.chars() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (),
        }
        
        // Calculate maximum distance based on current position, coordinates, and k value
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, s.len() as i32);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let k: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    // Call function to calculate result
    let result = max_distance(s, k);
    
    // Output result
    println!("{}", result);
    
    Ok(())
}