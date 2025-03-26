use std::io::{self, BufRead};

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

    for (i, c) in s.chars().enumerate() {
        match c {
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

fn main() -> io::Result<()> {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s = parts[0];
            let k: i32 = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid k value"))?;

            // Call function to calculate result
            let result = max_distance(s, k);

            // Output result
            println!("{}", result);
        } else {
            eprintln!("Error reading input");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input format"));
        }
    } else {
        eprintln!("Error reading input");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input provided"));
    }

    Ok(())
}